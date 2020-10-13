#[macro_use]
mod macros;
mod rreg_store;
mod rwreg_store;
// Reexports
pub use rreg_store::{BoxedRregStore, RregStore};
pub use rwreg_store::{BoxedRwregStore, RwregStore};

use crate::{
    modbus_master::{ModbusMaster, ModbusMasterError, ModbusMasterMessage},
    platine::{self, *},
    registers,
    serial_interface::SerialInterface,
};
use futures::channel::mpsc;
use gio::prelude::*;
use glib::{clone, signal};
use gtk::{prelude::*, Application, NotebookExt};
use std::{
    cell::RefCell,
    collections::HashMap,
    rc::Rc,
    sync::{Arc, Mutex},
};

const PKG_VERSION: &'static str = env!("CARGO_PKG_VERSION");
const PKG_NAME: &'static str = env!("CARGO_PKG_NAME");
const PKG_DESCRIPTION: &'static str = env!("CARGO_PKG_DESCRIPTION");

/// Representation der Grafischen Schnittstelle
pub struct Gui {
    combo_box_text_ports_changed_signal: glib::SignalHandlerId,
    combo_box_text_ports_map: Rc<RefCell<HashMap<String, u32>>>,
    combo_box_text_ports: gtk::ComboBoxText,
    infobar_error: gtk::InfoBar,
    infobar_info: gtk::InfoBar,
    infobar_question: gtk::InfoBar,
    infobar_warning: gtk::InfoBar,
    label_infobar_error_text: gtk::Label,
    label_infobar_info_text: gtk::Label,
    label_infobar_question_text: gtk::Label,
    label_infobar_warning_text: gtk::Label,
    platine: BoxedPlatine,
    revealer_infobar_error: gtk::Revealer,
    revealer_infobar_info: gtk::Revealer,
    revealer_infobar_question: gtk::Revealer,
    revealer_infobar_warning: gtk::Revealer,
    rreg_store: BoxedRregStore,
    rwreg_store: BoxedRwregStore,
    statusbar_application: gtk::Statusbar,
    statusbar_contexts: HashMap<StatusBarContext, u32>,
    toggle_button_connect: gtk::ToggleButton,
}

/// Kommandos an die Grafische Schnittstelle
#[derive(Debug)]
pub enum GuiMessage {
    /// Zeige Infobar mit Information an den Benutzer
    ShowInfo(String),
    /// Zeige Infobar mit Warnung an den Benutzer
    ShowWarning(String),
    /// Zeige Infobar mit Fehler an den Benutzer
    ShowError(String),
    /// Zeige Infobar mit Frage an den Benutzer
    ShowQuestion(String),
    /// Update verfügbare seriale Schnittstellen (Auswahlfeld oben links)
    UpdateSerialPorts(Vec<String>),
    /// Verarbeite Daten der Lese Register
    UpdateRregs(Result<Vec<(u16, u16)>, ModbusMasterError>),
    /// Verarbeite Daten der Schreib/ Lese Register
    UpdateRwregs(Result<Vec<(u16, u16)>, ModbusMasterError>),
}
/// Contexte für die Status Bar
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum StatusBarContext {
    PortOperation,
    _Error,
}

/// Startet die Grafische User Schnittstelle
pub fn launch() {
    let application = Application::new(
        Some("com.gaswarnanlagen.rgms.rgms_konfig"),
        Default::default(),
    )
    .expect("failed to initalize GTK application");

    application.connect_activate(|app| {
        ui_init(app);
    });

    application.run(&[]);
}

fn ui_init(app: &gtk::Application) {
    // Initalisierung
    // GUI Channel
    let (gui_tx, mut gui_rx) = mpsc::channel(0);
    // Modbus Master Thread
    let modbus_master = ModbusMaster::new(gui_tx.clone());
    // Modbus Master Channel
    let modbus_master_tx = modbus_master.tx;
    // Serial Interface Thread
    let _serial_interface = SerialInterface::new(gui_tx.clone());
    // Platine
    // Der Callback 'Auswahl Platine' setzt die verwendete Platine 'gui_platine' sowie
    // die Schreib und Schreib/Lese TreeStores 'rreg_store' und 'rwreg_store'
    let platine: BoxedPlatine = Arc::new(Mutex::new(None));
    let rreg_store: BoxedRregStore = Arc::new(Mutex::new(None));
    let rwreg_store: BoxedRwregStore = Arc::new(Mutex::new(None));

    // GUI Elemente
    //
    let glade_str = include_str!("rgms_konfig.ui");
    let builder = gtk::Builder::from_string(glade_str);
    let application_window: gtk::ApplicationWindow = build!(builder, "application_window");
    // Infobars
    let infobar_info: gtk::InfoBar = build!(builder, "infobar_info");
    let infobar_warning: gtk::InfoBar = build!(builder, "infobar_warning");
    let infobar_error: gtk::InfoBar = build!(builder, "infobar_error");
    let infobar_question: gtk::InfoBar = build!(builder, "infobar_question");
    let revealer_infobar_info: gtk::Revealer = build!(builder, "revealer_infobar_info");
    let revealer_infobar_warning: gtk::Revealer = build!(builder, "revealer_infobar_warning");
    let revealer_infobar_error: gtk::Revealer = build!(builder, "revealer_infobar_error");
    let revealer_infobar_question: gtk::Revealer = build!(builder, "revealer_infobar_question");
    let label_infobar_info_text: gtk::Label = build!(builder, "label_infobar_info_text");
    let label_infobar_warning_text: gtk::Label = build!(builder, "label_infobar_warning_text");
    let label_infobar_error_text: gtk::Label = build!(builder, "label_infobar_error_text");
    let label_infobar_question_text: gtk::Label = build!(builder, "label_infobar_question_text");
    let spin_button_modbus_address: gtk::SpinButton = build!(builder, "spin_button_modbus_address");
    let spin_button_new_modbus_address: gtk::SpinButton =
        build!(builder, "spin_button_new_modbus_address");
    let check_button_mcs: gtk::CheckButton = build!(builder, "check_button_mcs");
    let button_reset: gtk::Button = build!(builder, "button_reset");

    // Serial port selector
    let combo_box_text_ports: gtk::ComboBoxText = build!(builder, "combo_box_text_ports");
    let combo_box_text_ports_map = Rc::new(RefCell::new(HashMap::<String, u32>::new()));
    // Connect Toggle Button
    let toggle_button_connect: gtk::ToggleButton = build!(builder, "toggle_button_connect");
    // Statusbar
    let statusbar_application: gtk::Statusbar = build!(builder, "statusbar_application");
    let context_id_port_ops = statusbar_application.get_context_id("port operations");
    let statusbar_contexts: HashMap<StatusBarContext, u32> =
        [(StatusBarContext::PortOperation, context_id_port_ops)]
            .iter()
            .cloned()
            .collect();

    // Combo boxes
    // ComboBox Hardware Version
    let combo_box_text_hw_version: gtk::ComboBoxText = build!(builder, "combo_box_text_hw_version");
    for (id, name, _desc) in platine::HW_VERSIONS {
        combo_box_text_hw_version.append(Some(&id.to_string()), name);
    }
    // ComboBox Working Mode (Arbeitsweise)
    let combo_box_text_sensor_working_mode: gtk::ComboBoxText =
        build!(builder, "combo_box_text_sensor_working_mode");
    for (id, name) in platine::WORKING_MODES {
        combo_box_text_sensor_working_mode
            .append(Some(&id.to_string()), &format!("{} - {}", id, name));
    }

    // Menues
    let menu_item_quit: gtk::MenuItem = build!(builder, "menu_item_quit");
    let menu_item_about: gtk::MenuItem = build!(builder, "menu_item_about");
    let about_dialog: gtk::AboutDialog = build!(builder, "about_dialog");
    let about_dialog_button_ok: gtk::Button = build!(builder, "about_dialog_button_ok");
    about_dialog.set_program_name(PKG_NAME);
    #[cfg(feature = "ra-gas")]
    about_dialog.set_program_name(&format!("{} - RA-GAS intern!", PKG_NAME));
    about_dialog.set_version(Some(PKG_VERSION));
    about_dialog.set_comments(Some(PKG_DESCRIPTION));

    // HeaderBar
    let header_bar: gtk::HeaderBar = build!(builder, "header_bar");
    header_bar.set_title(Some(PKG_NAME));
    #[cfg(feature = "ra-gas")]
    header_bar.set_title(Some(&format!("{} - RA-GAS intern!", PKG_NAME)));
    header_bar.set_subtitle(Some(PKG_VERSION));

    let box_single_sensor: gtk::Box = build!(builder, "box_single_sensor");
    let box_duo_sensor: gtk::Box = build!(builder, "box_duo_sensor");
    let stack_sensor: gtk::Stack = build!(builder, "stack_sensor");

    let notebook_sensor: gtk::Notebook = build!(builder, "notebook_sensor");

    let button_nullpunkt: gtk::Button = build!(builder, "button_nullpunkt");
    let button_messgas: gtk::Button = build!(builder, "button_messgas");

    application_window.set_application(Some(app));

    //
    // CSS
    // Set CSS styles for the entire application.
    let css_provider = gtk::CssProvider::new();
    let display = gdk::Display::get_default().expect("Couldn't open default GDK display");
    let screen = display.get_default_screen();
    gtk::StyleContext::add_provider_for_screen(
        &screen,
        &css_provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
    css_provider
        .load_from_path("resources/style.css")
        .expect("Failed to load CSS stylesheet");
    // CSS for RA-GAS Version
    #[cfg(feature = "ra-gas")]
    {
        let css_provider_ra_gas = gtk::CssProvider::new();
        gtk::StyleContext::add_provider_for_screen(
            &screen,
            &css_provider_ra_gas,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );

        css_provider_ra_gas
            .load_from_path("resources/ra-gas.css")
            .expect("Failed to load CSS stylesheet (ra-gas features)");
    }

    //
    // Callbacks
    //

    let combo_box_text_ports_changed_signal = combo_box_text_ports.connect_changed(move |_| {});

    // Reset Button
    button_reset.connect_clicked(clone!(
        @strong spin_button_modbus_address => move |_| {
        spin_button_modbus_address.set_value(247.0);
    }));

    // Checkbox 'MCS Konfiguration?'
    check_button_mcs.connect_clicked(clone!(
        @strong spin_button_modbus_address,
        @strong spin_button_new_modbus_address => move |checkbox| {
            let adjustment_modbus_address = spin_button_modbus_address.get_adjustment();
            let adjustment_new_modbus_address = spin_button_new_modbus_address.get_adjustment();
            if checkbox.get_active() {
                spin_button_new_modbus_address.set_value(129.0);
                // In der MCS Konfiguration ist nur noch Modbus Adresse 247 mit
                // gestecktem Systemstecker möglich
                spin_button_modbus_address.set_value(247.0);
                adjustment_modbus_address.set_lower(247.0);
                adjustment_modbus_address.set_upper(247.0);
                // MCS Konfiguration erlaubt Adressen vom 129-256 (Modbus Standard erlaubt aber max. 255)
                adjustment_new_modbus_address.set_lower(129.0);
                adjustment_new_modbus_address.set_upper(255.0);
            } else {
                spin_button_modbus_address.set_value(247.0);
                adjustment_modbus_address.set_lower(1.0);
                adjustment_modbus_address.set_upper(255.0);
                adjustment_new_modbus_address.set_lower(1.0);
                adjustment_new_modbus_address.set_upper(255.0);
            }
        }
    ));

    // Button Connect (Live Ansicht)
    toggle_button_connect.connect_clicked(clone!(
        @strong platine,
        @strong combo_box_text_ports,
        @strong combo_box_text_ports_map,
        @strong modbus_master_tx,
        @strong gui_tx
        => move |button| {
            // Start Live Ansicht (get_active() == true für connect, false bei disconnect)
            if button.get_active() {
                // Lock Mutex, Unwrap Option ...
                let platine = platine.lock().unwrap();

                if let None = platine.as_ref() {
                    gui_tx.clone().try_send(GuiMessage::ShowError("Keine Platine ausgewählt!".to_string())).expect(r#"Failed to send Message"#);
                } else {
                    let active_port = combo_box_text_ports.get_active().unwrap_or(0);
                    // Extrahiert den Namen der Schnittstelle aus der HashMap, Key ist die Nummer der Schnittstelle
                    let mut tty_path = None;
                    for (p, i) in &*combo_box_text_ports_map.borrow() {
                        if *i == active_port {
                            tty_path = Some(p.to_owned());
                            break;
                        }
                    }
                    if let None = tty_path {
                        gui_tx.clone().try_send(GuiMessage::ShowError("Keine Schnittstelle gefunden!".to_string())).expect(r#"Failed to send Message"#);
                    }

                    // Extract Rregs, RwRegs from platine
                    let rregs = platine.as_ref().unwrap().vec_rregs();
                    let rwregs = platine.as_ref().unwrap().vec_rwregs();

                    // get modbus_address
                    let slave = spin_button_modbus_address.get_value() as u8;
                    info!("tty_path: {:?}, slave: {:?}", &tty_path, &slave);

                    modbus_master_tx.clone().try_send(ModbusMasterMessage::Connect(tty_path.unwrap(), slave, rregs, rwregs)).map_err(|e| {
                        gui_tx.clone().try_send(GuiMessage::ShowError(format!("Modbus Master konnte nicht erreicht werden: {}!", e))).expect(r#"Failed to send Message"#);
                    }).unwrap();
                }
            // Beende Live Ansicht
            } else {
                modbus_master_tx.clone().try_send(ModbusMasterMessage::Disconnect).map_err(|e| {
                    gui_tx.clone().try_send(GuiMessage::ShowError(format!("Modbus Master konnte nicht erreicht werden: {}!", e))).expect(r#"Failed to send Message"#);
                }).unwrap();
        }
        }
    ));

    // Button "Nullpunkt"
    button_nullpunkt.connect_clicked(clone!(
        @strong gui_tx
        => move |_| {
            // // Test Send Message an Infobar::Infor
            // gui_tx.clone().try_send(GuiMessage::ShowInfo("Lorem ipsum dolor sit amet consectetur, adipisicing elit. Aperiam eveniet nulla quam ea, saepe ut a quia blanditiis veniam voluptate expedita quidem at rerum est! Quaerat ratione incidunt sunt nisi.".to_string())).expect(r#"Failed to send Message"#);
            // gui_tx.clone().try_send(GuiMessage::ShowWarning("Lorem ipsum dolor sit amet consectetur adipisicing elit. Praesentium, aut?".to_string())).expect(r#"Failed to send Message"#);
            // gui_tx.clone().try_send(GuiMessage::ShowError("Lorem ipsum dolor sit amet.".to_string())).expect(r#"Failed to send Message"#);
            // gui_tx.clone().try_send(GuiMessage::ShowQuestion(format!("{:?} {}", std::time::SystemTime::now(), "lorem5".to_string()))).expect(r#"Failed to send Message"#);
        }
    ));

    // Button "Messgas"
    button_messgas.connect_clicked(clone!(
        @strong gui_tx
        => move |_| {
        }
    ));

    // Combo Box 'Auswahl Platine'
    //
    // Wird diese Auswahlbox selectiert werden die Anzeigen der Sensorwerte
    // entsprechend angepasst. Zudem wird die verwendete `Platine`
    // Anwendungsweit festgelegt.
    combo_box_text_hw_version.connect_changed(clone!(
        @strong box_duo_sensor,
        @strong box_single_sensor,
        @strong combo_box_text_hw_version,
        @strong notebook_sensor,
        @strong platine,
        @strong rreg_store,
        @strong rwreg_store,
        @strong stack_sensor
        => move |s| {
            match s.get_active_text().unwrap().as_str() {
                "Sensor-MB-CO2_O2_REV1_0" => {
                    // Lade Sensor Ansicht mit 2facher Messzelle
                    stack_sensor.set_visible_child_name("duo_sensor");
                    clean_notebook_tabs(&notebook_sensor);
                    // TODO: Create Error Infobar if csv parsing fails, Platine could not selected
                    let from_csv = Box::new(SensorMbCo2O2::new_from_csv().unwrap());
                    // Setzt die Platine die in der GUI verwendet werden soll
                    set_platine(&platine, from_csv);
                    // Setzt den TreeStore der Lese Register
                    // Füllt den TreeStore mit Daten und zeigt die TreeViews der Hardware im Notebook-Sensor an
                    set_rreg_store(&rreg_store, platine.clone(), &notebook_sensor);
                    // Setzt den TreeStore der Schreib/Lese Register
                    // Füllt den TreeStore mit Daten und zeigt die TreeViews der Hardware im Notebook-Sensor an
                    set_rwreg_store(&rwreg_store, platine.clone(), &notebook_sensor);
                    // fill_treestore_and_show_notebook(platine, &notebook_sensor);
                }
                "Sensor-MB-NAP5X_REV1_0" => {
                    stack_sensor.set_visible_child_name("single_sensor");
                    clean_notebook_tabs(&notebook_sensor);
                    // TODO: Create Error Infobar if csv parsing fails, Platine could not selected
                    let from_csv = Box::new(SensorMbNap5x::new_from_csv().unwrap());
                    // Setzt die Platine die in der GUI verwendet werden soll
                    set_platine(&platine, from_csv);
                    // Setzt den TreeStore der Lese Register
                    // Füllt den TreeStore mit Daten und zeigt die TreeViews der Hardware im Notebook-Sensor an
                    set_rreg_store(&rreg_store, platine.clone(), &notebook_sensor);
                    // Setzt den TreeStore der Schreib/Lese Register
                    // Füllt den TreeStore mit Daten und zeigt die TreeViews der Hardware im Notebook-Sensor an
                    set_rwreg_store(&rwreg_store, platine.clone(), &notebook_sensor);
                    // fill_treestore_and_show_notebook(platine, &notebook_sensor);
                }
                "Sensor-MB-NAP5xx_REV1_0" => {
                    // Lade Sensor Ansicht mit 2facher Messzelle
                    stack_sensor.set_visible_child_name("duo_sensor");
                    clean_notebook_tabs(&notebook_sensor);
                    // TODO: Create Error Infobar if csv parsing fails, Platine could not selected
                    let from_csv = Box::new(SensorMbNap5xx::new_from_csv().unwrap());
                    // Setzt die Platine die in der GUI verwendet werden soll
                    set_platine(&platine, from_csv);
                    // Setzt den TreeStore der Lese Register
                    // Füllt den TreeStore mit Daten und zeigt die TreeViews der Hardware im Notebook-Sensor an
                    set_rreg_store(&rreg_store, platine.clone(), &notebook_sensor);
                    // Setzt den TreeStore der Schreib/Lese Register
                    // Füllt den TreeStore mit Daten und zeigt die TreeViews der Hardware im Notebook-Sensor an
                    set_rwreg_store(&rwreg_store, platine.clone(), &notebook_sensor);
                    // fill_treestore_and_show_notebook(platine, &notebook_sensor);
                }
                "Sensor-MB-NE4_REV1_0" => {
                    stack_sensor.set_visible_child_name("single_sensor");
                    clean_notebook_tabs(&notebook_sensor);
                    // TODO: Create Error Infobar if csv parsing fails, Platine could not selected
                    let from_csv = Box::new(SensorMbNe4::new_from_csv().unwrap());
                    // Setzt die Platine die in der GUI verwendet werden soll
                    set_platine(&platine, from_csv);
                    // Setzt den TreeStore der Lese Register
                    // Füllt den TreeStore mit Daten und zeigt die TreeViews der Hardware im Notebook-Sensor an
                    set_rreg_store(&rreg_store, platine.clone(), &notebook_sensor);
                    // Setzt den TreeStore der Schreib/Lese Register
                    // Füllt den TreeStore mit Daten und zeigt die TreeViews der Hardware im Notebook-Sensor an
                    set_rwreg_store(&rwreg_store, platine.clone(), &notebook_sensor);
                    // fill_treestore_and_show_notebook(platine, &notebook_sensor);
                }
                "Sensor-MB-NE4-V1.0" => {
                    stack_sensor.set_visible_child_name("single_sensor");
                    clean_notebook_tabs(&notebook_sensor);
                    // TODO: Create Error Infobar if csv parsing fails, Platine could not selected
                    let from_csv = Box::new(SensorMbNe4Legacy::new_from_csv().unwrap());
                    // Setzt die Platine die in der GUI verwendet werden soll
                    set_platine(&platine, from_csv);
                    // Setzt den TreeStore der Lese Register
                    // Füllt den TreeStore mit Daten und zeigt die TreeViews der Hardware im Notebook-Sensor an
                    set_rreg_store(&rreg_store, platine.clone(), &notebook_sensor);
                    // Setzt den TreeStore der Schreib/Lese Register
                    // Füllt den TreeStore mit Daten und zeigt die TreeViews der Hardware im Notebook-Sensor an
                    set_rwreg_store(&rwreg_store, platine.clone(), &notebook_sensor);
                    // fill_treestore_and_show_notebook(platine, &notebook_sensor);
                }
                "Sensor-MB-SP42A_REV1_0" => {
                    stack_sensor.set_visible_child_name("single_sensor");
                    clean_notebook_tabs(&notebook_sensor);
                    // TODO: Create Error Infobar if csv parsing fails, Platine could not selected
                    let from_csv = Box::new(SensorMbSp42a::new_from_csv().unwrap());
                    // Setzt die Platine die in der GUI verwendet werden soll
                    set_platine(&platine, from_csv);
                    // Setzt den TreeStore der Lese Register
                    // Füllt den TreeStore mit Daten und zeigt die TreeViews der Hardware im Notebook-Sensor an
                    set_rreg_store(&rreg_store, platine.clone(), &notebook_sensor);
                    // Setzt den TreeStore der Schreib/Lese Register
                    // Füllt den TreeStore mit Daten und zeigt die TreeViews der Hardware im Notebook-Sensor an
                    set_rwreg_store(&rwreg_store, platine.clone(), &notebook_sensor);
                    // fill_treestore_and_show_notebook(platine, &notebook_sensor);
                }
                _ => {
                    stack_sensor.set_visible_child_name("single_sensor");
                }
            }
        }
    ));

    menu_item_quit.connect_activate(clone!(
        @weak application_window => move |_| {
            application_window.close()
        }
    ));

    menu_item_about.connect_activate(clone!(
        @strong about_dialog => move |_| {
            about_dialog.show()
        }
    ));

    about_dialog_button_ok.connect_clicked(clone!(
        @strong about_dialog => move |_| {
            about_dialog.hide()
        }
    ));

    // Infobar callbacks
    if let Some(button_close_infobar_info) = infobar_info.add_button("Ok", gtk::ResponseType::Close)
    {
        let _ = button_close_infobar_info.connect_clicked(clone!(
        @strong infobar_info
        => move |_| {
            &infobar_info.hide();
        }));
    }
    if let Some(button_close_infobar_warning) =
        infobar_warning.add_button("Ok", gtk::ResponseType::Close)
    {
        let _ = button_close_infobar_warning.connect_clicked(clone!(
        @strong infobar_warning
        => move |_| {
            &infobar_warning.hide();
        }));
    }
    if let Some(button_close_infobar_error) =
        infobar_error.add_button("Ok", gtk::ResponseType::Close)
    {
        let _ = button_close_infobar_error.connect_clicked(clone!(
        @strong infobar_error
        => move |_| {
            &infobar_error.hide();
        }));
    }
    if let Some(button_close_infobar_question) =
        infobar_question.add_button("Ok", gtk::ResponseType::Close)
    {
        let _ = button_close_infobar_question.connect_clicked(clone!(
        @strong infobar_question
        => move |_| {
            &infobar_question.hide();
        }));
    }

    // Ende Callbacks

    let gui = Gui {
        combo_box_text_ports_changed_signal,
        combo_box_text_ports_map,
        combo_box_text_ports,
        infobar_error,
        infobar_info,
        infobar_question,
        infobar_warning,
        label_infobar_error_text,
        label_infobar_info_text,
        label_infobar_question_text,
        label_infobar_warning_text,
        platine,
        revealer_infobar_error,
        revealer_infobar_info,
        revealer_infobar_question,
        revealer_infobar_warning,
        rreg_store,
        rwreg_store,
        statusbar_application,
        statusbar_contexts,
        toggle_button_connect,
    };

    application_window.show_all();

    // future on main thread has access to UI
    let future = {
        use futures::stream::StreamExt;

        async move {
            while let Some(event) = gui_rx.next().await {
                match event {
                    GuiMessage::ShowInfo(msg) => {
                        debug!("Show Infobar Information with: {}", msg);
                        gui.show_infobar_info(&msg);
                    }
                    GuiMessage::ShowWarning(msg) => {
                        debug!("Show Infobar Warning with: {}", msg);
                        gui.show_infobar_warning(&msg);
                    }
                    GuiMessage::ShowError(msg) => {
                        debug!("Show Infobar Error with: {}", msg);
                        gui.show_infobar_error(&msg);
                    }
                    GuiMessage::ShowQuestion(msg) => {
                        debug!("Show Infobar Question with: {}", msg);
                        gui.show_infobar_question(&msg);
                    }
                    GuiMessage::UpdateSerialPorts(ports) => {
                        debug!("Update Serial Ports with: {:?}", &ports);
                        update_serial_ports(&gui, ports);
                    }
                    GuiMessage::UpdateRregs(results) => {
                        println!("Update Rregs with: {:?}", &results);
                    }
                    GuiMessage::UpdateRwregs(results) => {
                        println!("Update Rwregs with: {:?}", &results);
                    }
                }
            }
        }
    };

    let c = glib::MainContext::default();
    c.spawn_local(future);
}

impl Gui {
    // wählt die Serielle Schnittstelle aus
    fn select_port(&self, num: u32) {
        // Restore selected serial interface
        signal::signal_handler_block(
            &self.combo_box_text_ports,
            &self.combo_box_text_ports_changed_signal,
        );
        &self.combo_box_text_ports.set_active(Some(num));
        signal::signal_handler_unblock(
            &self.combo_box_text_ports,
            &self.combo_box_text_ports_changed_signal,
        );
        &self.combo_box_text_ports.set_sensitive(true);
        &self.toggle_button_connect.set_sensitive(true);
    }

    /// Zeigt Status Nachrichten am unteren Bildschirmrand
    ///
    /// # Parameters
    /// - `context`     ein `StatusBarContext`
    /// - `message`     ein String Slice mit dem Text der angezeigt werden soll
    fn log_status(&self, context: StatusBarContext, message: &str) {
        if let Some(context_id) = self.statusbar_contexts.get(&context) {
            let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S");
            let formatted_message = format!("[{}]: {}", timestamp, message);
            self.statusbar_application
                .push(*context_id, &formatted_message);
        }
    }

    /// Show InfoBar Info
    ///
    fn show_infobar_info(&self, message: &str) {
        let label = &self.label_infobar_info_text;
        label.set_line_wrap(true);
        label.set_text(message);

        &self.infobar_info.show_all();
        &self.revealer_infobar_info.set_reveal_child(true);
    }

    /// Show InfoBar Warning
    ///
    fn show_infobar_warning(&self, message: &str) {
        let label = &self.label_infobar_warning_text;
        label.set_line_wrap(true);
        label.set_text(message);

        &self.infobar_warning.show_all();
        &self.revealer_infobar_warning.set_reveal_child(true);
    }

    /// Show InfoBar Error
    ///
    fn show_infobar_error(&self, message: &str) {
        let label = &self.label_infobar_error_text;
        label.set_line_wrap(true);
        label.set_text(message);

        &self.infobar_error.show_all();
        &self.revealer_infobar_error.set_reveal_child(true);
    }

    /// Show InfoBar Question
    ///
    fn show_infobar_question(&self, message: &str) {
        let label = &self.label_infobar_question_text;
        label.set_line_wrap(true);
        label.set_text(message);

        &self.infobar_question.show_all();
        &self.revealer_infobar_question.set_reveal_child(true);
    }
}

// TODO: Überprüfe diese Funktion
/// Update verfügbare serielle Schnittstellen
///
/// Diese Funktion wird von der GuiMessage::UpdateSerialPorts aufgerufen
fn update_serial_ports(gui: &Gui, ports: Vec<String>) {
    info!("Execute event UiCommand::UpdatePorts: {:?}", ports);
    debug!("active port: {:?}", gui.combo_box_text_ports.get_active());
    let active_port = gui.combo_box_text_ports.get_active().unwrap_or(0);
    let old_num_ports = gui.combo_box_text_ports_map.borrow().len();
    // Update the port listing and other UI elements
    gui.combo_box_text_ports.remove_all();
    gui.combo_box_text_ports_map.borrow_mut().clear();
    // keine Seriellen Schittstellen gefunden
    if ports.is_empty() {
        debug!("kein Port gefunden",);

        //     disable_ui_elements(&ui);

        gui.combo_box_text_ports
            .append(None, "Keine Schnittstelle gefunden");
        gui.combo_box_text_ports.set_active(Some(0));
        gui.combo_box_text_ports.set_sensitive(false);
        gui.toggle_button_connect.set_sensitive(false);
    } else {
        for (i, p) in (0u32..).zip(ports.clone().into_iter()) {
            gui.combo_box_text_ports.append(None, &p);
            gui.combo_box_text_ports_map.borrow_mut().insert(p, i);
        }
        let num_ports = gui.combo_box_text_ports_map.borrow().len();
        // Einen oder mehrere Serial Ports verloren
        if num_ports < old_num_ports {
            debug!(
                "Port entfernt: active_port:{:?} num_ports:{:?} old_num_ports:{:?}",
                active_port, num_ports, old_num_ports
            );
            // tokio_thread_sender
            //     .clone()
            //     .try_send(TokioCommand::Disconnect)
            //     .expect("Failed to send tokio command");

            // Restore selected serial interface
            gui.select_port(active_port - 1);

            // Nachricht an Statusbar
            gui.log_status(
                StatusBarContext::PortOperation,
                &format!(
                    "Schnittstelle verloren! Aktuelle Schnittstellen: {:?}",
                    ports
                ),
            );
        // New serial port found
        } else if num_ports > old_num_ports {
            debug!(
                "Port gefunden: active_port:{:?} num_ports:{:?} old_num_ports:{:?}",
                active_port, num_ports, old_num_ports
            );
            // // Enable graphical elements
            // enable_ui_elements(&ui);

            // Restore selected serial interface
            gui.select_port(num_ports as u32 - 1);

            // Nachricht an Statusbar
            gui.log_status(
                StatusBarContext::PortOperation,
                &format!("Neue Schnittstelle gefunden: {:?}", ports),
            );
        } else if num_ports == old_num_ports {
            debug!(
                "Ports unverändert: active_port:{:?} num_ports:{:?} old_num_ports:{:?}",
                active_port, num_ports, old_num_ports
            );
            // Restore selected serial interface
            gui.select_port(active_port);
        }
    }
}

// Lösche Notebook Tabs wenn schon 3 angezeigt werden
//
// Diese Funktion löscht erst den 3. Tab anschließend den 2.
fn clean_notebook_tabs(notebook: &gtk::Notebook) {
    if notebook.get_n_pages() == 3 {
        // Tap 3
        let child = notebook.get_nth_page(None).unwrap();
        notebook.detach_tab(&child);
        // Tab 2
        let child = notebook.get_nth_page(None).unwrap();
        notebook.detach_tab(&child);
    };
}

/// Setzt die Platine die in der GUI verwendet wird.
///
pub fn set_platine(gui_platine: &BoxedPlatine, platine: Box<dyn Platine>) {
    if let Ok(mut gui_plaine) = gui_platine.lock() {
        *gui_plaine = Some(platine);
    }
}

/// Setzt die Lese Register TreeStore der in der GUI verwendet wird.
///
pub fn set_rreg_store(rreg_store: &BoxedRregStore, platine: BoxedPlatine, notebook: &gtk::Notebook) {
    let store = RregStore::new(platine);
    if let Ok(mut ptr) = rreg_store.lock() {
        let ui = store.fill_and_build_ui();
        notebook.add(&ui);
        notebook.set_tab_label_text(&ui, registers::REGISTER_TYPES[0].1);
        notebook.show_all();
        *ptr = Some(store);
    }
}

/// Setzt die Schreib/Lese Register TreeStore der in der GUI verwendet wird.
/// Bildet danach den Treestore und zeigt diesen im Notebook Widget an.
///
pub fn set_rwreg_store(rwreg_store: &BoxedRwregStore, platine: BoxedPlatine, notebook: &gtk::Notebook) {
    let store = RwregStore::new(platine);
    if let Ok(mut ptr) = rwreg_store.lock() {
        let ui = store.fill_and_build_ui();
        notebook.add(&ui);
        notebook.set_tab_label_text(&ui, registers::REGISTER_TYPES[1].1);
        notebook.show_all();
        *ptr = Some(store);
    }
}
