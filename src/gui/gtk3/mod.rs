#[macro_use]
mod macros;
mod rreg_store;
mod rwreg_store;
// Reexports
pub use rreg_store::{BoxedRregStore, RregStore};
pub use rwreg_store::{BoxedRwregStore, RwregStore};

use crate::{
    modbus_master::{ModbusMaster, ModbusMasterMessage},
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
#[allow(dead_code)]
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
    label_sensor_value_value: gtk::Label,
    label_sensor1_value_value: gtk::Label,
    label_sensor1_value_si: gtk::Label,
    label_sensor2_value_value: gtk::Label,
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
    check_button_mcs: gtk::CheckButton,
    spin_button_new_modbus_address: gtk::SpinButton,
    button_new_modbus_address: gtk::Button,
    combo_box_text_hw_version: gtk::ComboBoxText,
    combo_box_text_sensor_working_mode: gtk::ComboBoxText,
    button_sensor_working_mode: gtk::Button,
    button_nullpunkt: gtk::Button,
    button_messgas: gtk::Button,
    button_duo_sensor1_nullpunkt: gtk::Button,
    button_duo_sensor1_messgas: gtk::Button,
    button_duo_sensor2_nullpunkt: gtk::Button,
    button_duo_sensor2_messgas: gtk::Button,
}

/// Kommandos an die Grafische Schnittstelle
#[derive(Debug)]
pub enum GuiMessage {
    /// Deaktiviert die GUI Elemente
    DisableUiElements,
    /// Aktiviert die GUI Elemente
    EnableUiElements,
    /// Zeige Infobar mit Information an den Benutzer
    ShowInfo(String),
    /// Zeige Infobar mit Warnung an den Benutzer
    ShowWarning(String),
    /// Zeige Infobar mit Fehler an den Benutzer
    ShowError(String),
    /// Zeige Infobar mit Frage an den Benutzer
    ShowQuestion(String),
    /// Diese Nachricht kommt vom RwregStore
    /// RwregStore -> Gui -> ModbusMaster -> Gui
    ModbusMasterUpdateRegister {
        /// Register Nummer
        reg_nr: Result<u16, ()>,
        /// Neuer Wert
        new_value: String,
        // /// Modbus Master tx Channel
        // modbus_master_tx: tokio::sync::mpsc::Sender<crate::modbus_master::ModbusMasterMessage>,
    },
    /// Update Sensor Werte
    UpdateSensorValues(Vec<(u16, u16)>),
    /// Update verfügbare seriale Schnittstellen (Auswahlfeld oben links)
    UpdateSerialPorts(Vec<String>),
    /// Verarbeite Daten der Lese-Register
    UpdateRregs(Vec<(u16, u16)>),
    /// Verarbeite Daten der Schreib.-/ Lese-Register
    UpdateRwregs(Vec<(u16, u16)>),
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
    let label_sensor_value_value: gtk::Label = build!(builder, "label_sensor_value_value");
    let label_sensor1_value_value: gtk::Label = build!(builder, "label_sensor1_value_value");
    let label_sensor1_value_si: gtk::Label = build!(builder, "label_sensor1_value_si");
    let label_sensor2_value_value: gtk::Label = build!(builder, "label_sensor2_value_value");
    let spin_button_new_modbus_address: gtk::SpinButton =
        build!(builder, "spin_button_new_modbus_address");
    let check_button_mcs: gtk::CheckButton = build!(builder, "check_button_mcs");
    let button_new_modbus_address: gtk::Button = build!(builder, "button_new_modbus_address");
    let button_reset: gtk::Button = build!(builder, "button_reset");
    let button_sensor_working_mode: gtk::Button = build!(builder, "button_sensor_working_mode");
    let button_duo_sensor1_nullpunkt: gtk::Button = build!(builder, "button_duo_sensor1_nullpunkt");
    let button_duo_sensor1_messgas: gtk::Button = build!(builder, "button_duo_sensor1_messgas");
    let button_duo_sensor2_nullpunkt: gtk::Button = build!(builder, "button_duo_sensor2_nullpunkt");
    let button_duo_sensor2_messgas: gtk::Button = build!(builder, "button_duo_sensor2_messgas");

    // Serial port selector
    let combo_box_text_ports: gtk::ComboBoxText = build!(builder, "combo_box_text_ports");
    let combo_box_text_ports_map = Rc::new(RefCell::new(HashMap::<String, u32>::new()));
    // Connect Toggle Button
    let toggle_button_connect: gtk::ToggleButton = build!(builder, "toggle_button_connect");
    toggle_button_connect.get_style_context().add_class("suggested-action");
    // Statusbar message
    let statusbar_application: gtk::Statusbar = build!(builder, "statusbar_application");
    let context_id_port_ops = statusbar_application.get_context_id("port operations");
    let statusbar_contexts: HashMap<StatusBarContext, u32> =
        [(StatusBarContext::PortOperation, context_id_port_ops)]
            .iter()
            .cloned()
            .collect();

    // Combo boxes
    // ComboBox Hardware Version
    // TODO: Hardware Version pro Platine
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
    #[cfg(not(feature = "ra-gas"))]
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
    // Begin Callbacks
    //

    let combo_box_text_ports_changed_signal = combo_box_text_ports.connect_changed(move |_| {});

    // Callback: Reset Button
    //
    // Dieser Callback setzt die Modbus Adresse wieder auf 247 (Systemstecker zurück)
    button_reset.connect_clicked(clone!(
        @strong spin_button_modbus_address => move |_| {
        spin_button_modbus_address.set_value(247.0);
    }));

    #[cfg(feature = "ra-gas")]
    // Callback: Checkbox 'MCS Konfiguration?'
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

    #[cfg(feature = "ra-gas")]
    // Callback: Speichern der neuen Modbus ID
    button_new_modbus_address.connect_clicked(clone!(
        @strong check_button_mcs,
        @strong combo_box_text_ports_map,
        @strong combo_box_text_ports,
        @strong gui_tx,
        @strong modbus_master_tx,
        @strong platine,
        @strong spin_button_new_modbus_address,
        @strong spin_button_modbus_address
        => move |_| {
            // FIXME: Implementiere mich!
            // let tty_path = get_tty_path(&gui_tx);
            // let reg_protection = get_reg_protection(&gui_tx);
            // let slave = get_slave_id(&gui_tx);

            // get modbus_address
            let slave = spin_button_modbus_address.get_value() as u8;
            // get new modbus_address
            let new_slave_id = spin_button_new_modbus_address.get_value() as u16;

            // get MCS Konfig
            let mcs_config = check_button_mcs.get_active();

            match platine.lock() {
                Ok(platine) => {
                    match platine.as_ref() {
                        Some(platine) => {
                            // tty path
                            let active_port = combo_box_text_ports.get_active().unwrap_or(0);
                            // Extrahiert den Namen der Schnittstelle aus der HashMap, Key ist die Nummer der Schnittstelle
                            let mut tty_path = None;
                            for (p, i) in &*combo_box_text_ports_map.borrow() {
                                if *i == active_port {
                                    tty_path = Some(p.to_owned());
                                    break;
                                }
                            }
                            let tty_path = match tty_path {
                                Some(tty_path) => tty_path,
                                None => {
                                    show_error(&gui_tx, "Keine Schnittstelle gefunden!");
                                    return
                                }
                            };
                            // reg_protection
                            let reg_protection = platine.reg_protection();

                            // Sende Nachricht an Modbus Master und werte diese aus
                            if mcs_config {
                                match modbus_master_tx.clone()
                                .try_send(ModbusMasterMessage::SetNewMcsBusId {
                                    tty_path,
                                    slave,
                                    new_slave_id,
                                    reg_protection
                                })
                                {
                                    Ok(_) => {
                                        show_info(&gui_tx, &format!("MCS BUS Adresse: <b>{}</b> gespeichert.", &new_slave_id));
                                    }
                                    Err(error) => {
                                        show_error(&gui_tx, &format!("Modbus Master konnte nicht erreicht werden: {}!", error));
                                    }
                                }
                            } else {
                                match modbus_master_tx.clone()
                                .try_send(ModbusMasterMessage::SetNewModbusId {
                                    tty_path,
                                    slave,
                                    new_slave_id,
                                    reg_protection
                                })
                                {
                                    Ok(_) => {
                                        show_info(&gui_tx, &format!("Modbus Adresse: <b>{}</b> gespeichert.", &new_slave_id));
                                    }
                                    Err(error) => {
                                        show_error(&gui_tx, &format!("Modbus Master konnte nicht erreicht werden: {}!", error));
                                    }
                                }
                            }
                        },
                        // keine Platine gewählt
                        None => {
                            show_error(&gui_tx, "Keine Platine ausgewählt!");
                        }
                    }
                },
                Err(_) => { }
            }
        }
    ));

    // Callback: Button Connect (Live Ansicht)
    toggle_button_connect.connect_clicked(clone!(
        @strong combo_box_text_ports_map,
        @strong combo_box_text_ports,
        @strong combo_box_text_hw_version,
        @strong gui_tx,
        @strong modbus_master_tx,
        @strong platine,
        @strong spin_button_modbus_address
        => move |button| {
            // Start Live Ansicht
            // (get_active() == true für connect, false bei disconnect)
            if button.get_active() {

                // Lock Mutex, Unwrap Option ...
                match platine.lock() {
                    Ok(platine) => {
                        match platine.as_ref() {
                            Some(platine) => {
                                let active_port = combo_box_text_ports.get_active().unwrap_or(0);
                                // Extrahiert den Namen der Schnittstelle aus der HashMap, Key ist die Nummer der Schnittstelle
                                let mut tty_path = None;
                                for (p, i) in &*combo_box_text_ports_map.borrow() {
                                    if *i == active_port {
                                        tty_path = Some(p.to_owned());
                                        break;
                                    }
                                }
                                let tty_path = match tty_path {
                                    Some(tty_path) => tty_path,
                                    None => {
                                        show_error(&gui_tx, "Keine Schnittstelle gefunden!");
                                        return
                                    }
                                };

                                // Extract Rregs, RwRegs, Lock Register from platine
                                let rregs = platine.vec_rregs();
                                let rwregs = platine.vec_rwregs();
                                let reg_protection = platine.reg_protection();

                                // get modbus_address
                                let slave = spin_button_modbus_address.get_value() as u8;
                                info!("tty_path: {:?}, slave: {:?}", &tty_path, &slave);

                                // Sende Nachricht an Modbus Master und werte diese aus
                                match modbus_master_tx.clone()
                                .try_send(ModbusMasterMessage::Connect(
                                    tty_path,
                                    slave,
                                    rregs,
                                    rwregs,
                                    reg_protection
                                )) {
                                    Ok(_) => {}
                                    Err(error) => {
                                        show_error(&gui_tx, &format!("Modbus Master konnte nicht erreicht werden: {}!", error));
                                    }
                                }
                                // disable gui elemente
                                let _ = gui_tx.clone().try_send(GuiMessage::DisableUiElements);
                                combo_box_text_hw_version.set_sensitive(false);
                            }
                            None => {
                                show_error(&gui_tx, "Keine Platine ausgewählt!");
                            }
                        }
                    }
                    Err(_) => {
                        show_error(&gui_tx, "Konnte Platine Mutex nicht entsperren!");
                    }
                }
            // Beende Live Ansicht
            } else {
                // Sende Nachricht an Modbus Master und werte diese aus
                match modbus_master_tx.clone()
                .try_send(ModbusMasterMessage::Disconnect) {
                    Ok(_) => {
                        show_info(&gui_tx, &format!("Live Ansicht beendet"));
                    }
                    Err(error) => {
                        show_error(&gui_tx, &format!("Modbus Master konnte nicht erreicht werden: {}!", error));
                    }
                }
                // enable gui elemente
                let _ = gui_tx.clone().try_send(GuiMessage::EnableUiElements);
                combo_box_text_hw_version.set_sensitive(true);
            }
        }
    ));

    // Callback: Button "Nullpunkt"
    button_nullpunkt.connect_clicked(clone!(
        @strong combo_box_text_ports_map,
        @strong combo_box_text_ports,
        @strong combo_box_text_sensor_working_mode,
        @strong gui_tx,
        @strong modbus_master_tx,
        @strong platine,
        @strong spin_button_modbus_address
        => move |_| {
            // FIXME: Refactor das!
            match platine.lock() {
                Ok(platine) => {
                    match platine.as_ref() {
                        Some(platine) => {
                            let active_port = combo_box_text_ports.get_active().unwrap_or(0);
                            // Extrahiert den Namen der Schnittstelle aus der HashMap, Key ist die Nummer der Schnittstelle
                            let mut tty_path = None;
                            for (p, i) in &*combo_box_text_ports_map.borrow() {
                                if *i == active_port {
                                    tty_path = Some(p.to_owned());
                                    break;
                                }
                            }
                            let tty_path = match tty_path {
                                Some(tty_path) => tty_path,
                                None => {
                                    show_error(&gui_tx, "Keine Schnittstelle gefunden!");
                                    return
                                }
                            };

                            // Extract Lock Register und TTY Pfad
                            let reg_protection = platine.reg_protection();

                            // get modbus_address
                            let slave = spin_button_modbus_address.get_value() as u8;
                            debug!("tty_path: {:?}, slave: {:?}", &tty_path, &slave);

                            // Sende Nachricht an Modbus Master und werte diese aus
                            match modbus_master_tx.clone()
                            .try_send(ModbusMasterMessage::Nullgas {
                                tty_path,
                                slave,
                                reg_protection,
                                sensor_num: 1,
                            }) {
                                Ok(_) => {}
                                Err(error) => {
                                    show_error(&gui_tx, &format!("Modbus Master konnte nicht erreicht werden: {}!", error));
                                }
                            }
                        },
                        None => {
                            show_error(&gui_tx, &format!("Es wurde keine Platine ausgewählt!"));
                        }
                    }
                },
                Err(error) => {
                    show_error(&gui_tx, &format!("Platine Mutex Lock konnte nicht entfernt werden:\r\n{}!", error));
                }
            }
        }
    ));

    // Callback: Button "Nullpunkt" Messzelle 1
    button_duo_sensor1_nullpunkt.connect_clicked(clone!(
        @strong combo_box_text_ports_map,
        @strong combo_box_text_ports,
        @strong combo_box_text_sensor_working_mode,
        @strong gui_tx,
        @strong modbus_master_tx,
        @strong platine,
        @strong spin_button_modbus_address
        => move |_| {
            // FIXME: Refactor das!
            match platine.lock() {
                Ok(platine) => {
                    match platine.as_ref() {
                        Some(platine) => {
                            let active_port = combo_box_text_ports.get_active().unwrap_or(0);
                            // Extrahiert den Namen der Schnittstelle aus der HashMap, Key ist die Nummer der Schnittstelle
                            let mut tty_path = None;
                            for (p, i) in &*combo_box_text_ports_map.borrow() {
                                if *i == active_port {
                                    tty_path = Some(p.to_owned());
                                    break;
                                }
                            }
                            let tty_path = match tty_path {
                                Some(tty_path) => tty_path,
                                None => {
                                    show_error(&gui_tx, "Keine Schnittstelle gefunden!");
                                    return
                                }
                            };

                            // Extract Lock Register und TTY Pfad
                            let reg_protection = platine.reg_protection();

                            // get modbus_address
                            let slave = spin_button_modbus_address.get_value() as u8;
                            debug!("tty_path: {:?}, slave: {:?}", &tty_path, &slave);

                            // Sende Nachricht an Modbus Master und werte diese aus
                            match modbus_master_tx.clone()
                            .try_send(ModbusMasterMessage::Nullgas {
                                tty_path,
                                slave,
                                reg_protection,
                                sensor_num: 1,
                            }) {
                                Ok(_) => {}
                                Err(error) => {
                                    show_error(&gui_tx, &format!("Modbus Master konnte nicht erreicht werden: {}!", error));
                                }
                            }
                        },
                        None => {
                            show_error(&gui_tx, &format!("Es wurde keine Platine ausgewählt!"));
                        }
                    }
                },
                Err(error) => {
                    show_error(&gui_tx, &format!("Platine Mutex Lock konnte nicht entfernt werden:\r\n{}!", error));
                }
            }
        }
    ));

    // Callback: Button "Nullpunkt" 2. Messzelle
    button_duo_sensor2_nullpunkt.connect_clicked(clone!(
        @strong combo_box_text_ports_map,
        @strong combo_box_text_ports,
        @strong combo_box_text_sensor_working_mode,
        @strong gui_tx,
        @strong modbus_master_tx,
        @strong platine,
        @strong spin_button_modbus_address
        => move |_| {
            // FIXME: Refactor das!
            match platine.lock() {
                Ok(platine) => {
                    match platine.as_ref() {
                        Some(platine) => {
                            let active_port = combo_box_text_ports.get_active().unwrap_or(0);
                            // Extrahiert den Namen der Schnittstelle aus der HashMap, Key ist die Nummer der Schnittstelle
                            let mut tty_path = None;
                            for (p, i) in &*combo_box_text_ports_map.borrow() {
                                if *i == active_port {
                                    tty_path = Some(p.to_owned());
                                    break;
                                }
                            }
                            let tty_path = match tty_path {
                                Some(tty_path) => tty_path,
                                None => {
                                    show_error(&gui_tx, "Keine Schnittstelle gefunden!");
                                    return
                                }
                            };

                            // Extract Lock Register und TTY Pfad
                            let reg_protection = platine.reg_protection();

                            // get modbus_address
                            let slave = spin_button_modbus_address.get_value() as u8;
                            debug!("tty_path: {:?}, slave: {:?}", &tty_path, &slave);

                            // Sende Nachricht an Modbus Master und werte diese aus
                            match modbus_master_tx.clone()
                            .try_send(ModbusMasterMessage::Nullgas {
                                tty_path,
                                slave,
                                reg_protection,
                                sensor_num: 2,
                            }) {
                                Ok(_) => {}
                                Err(error) => {
                                    show_error(&gui_tx, &format!("Modbus Master konnte nicht erreicht werden: {}!", error));
                                }
                            }
                        },
                        None => {
                            show_error(&gui_tx, &format!("Es wurde keine Platine ausgewählt!"));
                        }
                    }
                },
                Err(error) => {
                    show_error(&gui_tx, &format!("Platine Mutex Lock konnte nicht entfernt werden:\r\n{}!", error));
                }
            }
        }
    ));

    // Callback: Button "Messgas"
    button_messgas.connect_clicked(clone!(
        @strong combo_box_text_ports_map,
        @strong combo_box_text_ports,
        @strong combo_box_text_sensor_working_mode,
        @strong gui_tx,
        @strong modbus_master_tx,
        @strong platine,
        @strong spin_button_modbus_address
        => move |_| {
            // FIXME: Refactor das!
            match platine.lock() {
                Ok(platine) => {
                    match platine.as_ref() {
                        Some(platine) => {
                            let active_port = combo_box_text_ports.get_active().unwrap_or(0);
                            // Extrahiert den Namen der Schnittstelle aus der HashMap, Key ist die Nummer der Schnittstelle
                            let mut tty_path = None;
                            for (p, i) in &*combo_box_text_ports_map.borrow() {
                                if *i == active_port {
                                    tty_path = Some(p.to_owned());
                                    break;
                                }
                            }
                            let tty_path = match tty_path {
                                Some(tty_path) => tty_path,
                                None => {
                                    show_error(&gui_tx, "Keine Schnittstelle gefunden!");
                                    return
                                }
                            };

                            // Extract Lock Register und TTY Pfad
                            let reg_protection = platine.reg_protection();

                            // get modbus_address
                            let slave = spin_button_modbus_address.get_value() as u8;
                            info!("tty_path: {:?}, slave: {:?}", &tty_path, &slave);

                            // Sende Nachricht an Modbus Master und werte diese aus
                            match modbus_master_tx.clone()
                            .try_send(ModbusMasterMessage::Messgas{
                                tty_path,
                                slave,
                                reg_protection,
                                sensor_num: 1,
                            }) {
                                Ok(_) => {}
                                Err(error) => {
                                    show_error(&gui_tx, &format!("Modbus Master konnte nicht erreicht werden: {}!", error));
                                }
                            }
                        },
                        None => {
                            show_error(&gui_tx, &format!("Es wurde keine Platine ausgewählt!"));
                        }
                    }
                },
                Err(error) => {
                    show_error(&gui_tx, &format!("Platine Mutex Lock konnte nicht entfernt werden:\r\n{}!", error));
                }
            }
        }
    ));

    // Callback: Button "Messgas" 1. Messzelle
    button_duo_sensor1_messgas.connect_clicked(clone!(
        @strong combo_box_text_ports_map,
        @strong combo_box_text_ports,
        @strong combo_box_text_sensor_working_mode,
        @strong gui_tx,
        @strong modbus_master_tx,
        @strong platine,
        @strong spin_button_modbus_address
        => move |_| {
            // FIXME: Refactor das!
            match platine.lock() {
                Ok(platine) => {
                    match platine.as_ref() {
                        Some(platine) => {
                            let active_port = combo_box_text_ports.get_active().unwrap_or(0);
                            // Extrahiert den Namen der Schnittstelle aus der HashMap, Key ist die Nummer der Schnittstelle
                            let mut tty_path = None;
                            for (p, i) in &*combo_box_text_ports_map.borrow() {
                                if *i == active_port {
                                    tty_path = Some(p.to_owned());
                                    break;
                                }
                            }
                            let tty_path = match tty_path {
                                Some(tty_path) => tty_path,
                                None => {
                                    show_error(&gui_tx, "Keine Schnittstelle gefunden!");
                                    return
                                }
                            };

                            // Extract Lock Register und TTY Pfad
                            let reg_protection = platine.reg_protection();

                            // get modbus_address
                            let slave = spin_button_modbus_address.get_value() as u8;
                            info!("tty_path: {:?}, slave: {:?}", &tty_path, &slave);

                            // Sende Nachricht an Modbus Master und werte diese aus
                            match modbus_master_tx.clone()
                            .try_send(ModbusMasterMessage::Messgas{
                                tty_path,
                                slave,
                                reg_protection,
                                sensor_num: 1,
                            }) {
                                Ok(_) => {}
                                Err(error) => {
                                    show_error(&gui_tx, &format!("Modbus Master konnte nicht erreicht werden: {}!", error));
                                }
                            }
                        },
                        None => {
                            show_error(&gui_tx, &format!("Es wurde keine Platine ausgewählt!"));
                        }
                    }
                },
                Err(error) => {
                    show_error(&gui_tx, &format!("Platine Mutex Lock konnte nicht entfernt werden:\r\n{}!", error));
                }
            }
        }
    ));

    // Callback: Button "Messgas" 2. Messzelle
    button_duo_sensor2_messgas.connect_clicked(clone!(
        @strong combo_box_text_ports_map,
        @strong combo_box_text_ports,
        @strong combo_box_text_sensor_working_mode,
        @strong gui_tx,
        @strong modbus_master_tx,
        @strong platine,
        @strong spin_button_modbus_address
        => move |_| {
            // FIXME: Refactor das!
            match platine.lock() {
                Ok(platine) => {
                    match platine.as_ref() {
                        Some(platine) => {
                            let active_port = combo_box_text_ports.get_active().unwrap_or(0);
                            // Extrahiert den Namen der Schnittstelle aus der HashMap, Key ist die Nummer der Schnittstelle
                            let mut tty_path = None;
                            for (p, i) in &*combo_box_text_ports_map.borrow() {
                                if *i == active_port {
                                    tty_path = Some(p.to_owned());
                                    break;
                                }
                            }
                            let tty_path = match tty_path {
                                Some(tty_path) => tty_path,
                                None => {
                                    show_error(&gui_tx, "Keine Schnittstelle gefunden!");
                                    return
                                }
                            };

                            // Extract Lock Register und TTY Pfad
                            let reg_protection = platine.reg_protection();

                            // get modbus_address
                            let slave = spin_button_modbus_address.get_value() as u8;

                            // Sende Nachricht an Modbus Master und werte diese aus
                            match modbus_master_tx.clone()
                            .try_send(ModbusMasterMessage::Messgas{
                                tty_path,
                                slave,
                                reg_protection,
                                sensor_num: 2,
                            }) {
                                Ok(_) => {}
                                Err(error) => {
                                    show_error(&gui_tx, &format!("Modbus Master konnte nicht erreicht werden: {}!", error));
                                }
                            }
                        },
                        None => {
                            show_error(&gui_tx, &format!("Es wurde keine Platine ausgewählt!"));
                        }
                    }
                },
                Err(error) => {
                    show_error(&gui_tx, &format!("Platine Mutex Lock konnte nicht entfernt werden:\r\n{}!", error));
                }
            }
        }
    ));

    // Callback: Combo Box 'Auswahl Platine'
    //
    // Wird diese Auswahlbox selektiert werden die Anzeigen der Sensorwerte
    // entsprechend angepasst. Zudem wird die verwendete `Platine`
    // anwendungsweit festgelegt.
    combo_box_text_hw_version.connect_changed(clone!(
        @strong box_duo_sensor,
        @strong box_single_sensor,
        @strong button_duo_sensor1_messgas,
        @strong button_duo_sensor1_nullpunkt,
        @strong button_duo_sensor2_messgas,
        @strong button_duo_sensor2_nullpunkt,
        @strong button_messgas,
        @strong button_new_modbus_address,
        @strong button_nullpunkt,
        @strong check_button_mcs,
        @strong combo_box_text_hw_version,
        @strong combo_box_text_sensor_working_mode,
        @strong gui_tx,
        @strong label_sensor1_value_si,
        @strong modbus_master_tx,
        @strong notebook_sensor,
        @strong platine,
        @strong rreg_store,
        @strong rwreg_store,
        @strong spin_button_new_modbus_address,
        @strong stack_sensor,
        @strong toggle_button_connect
        => move |s| {
            match s.get_active_text().unwrap().as_str() {
                "Sensor-MB-CO2_O2_REV1_0" => {
                    // Lade Sensor Ansicht mit 2facher Messzelle
                    stack_sensor.set_visible_child_name("duo_sensor");
                    clean_notebook_tabs(&notebook_sensor);
                    match SensorMbCo2O2::new_from_csv() {
                        Ok(from_csv) => {
                            let from_csv = Box::new(from_csv);
                            // Setzt die Platine die in der GUI verwendet werden soll
                            set_platine(&platine, from_csv);
                            // Setzt den TreeStore der Lese Register
                            // Füllt den TreeStore mit Daten und zeigt die TreeViews der Hardware im Notebook-Sensor an
                            set_rreg_store(&rreg_store, platine.clone(), &notebook_sensor);

                            #[cfg(feature = "ra-gas")]
                            // Setzt den TreeStore der Schreib/Lese Register
                            // Füllt den TreeStore mit Daten und zeigt die TreeViews der Hardware im Notebook-Sensor an
                            set_rwreg_store(&rwreg_store, platine.clone(), &notebook_sensor, &gui_tx);

                            // SI einheit Sensor1 (Sauerstoff auf Vol%)
                            label_sensor1_value_si.set_text("Vol%");
                        },
                        Err(error) => {
                            show_error(&gui_tx, &format!("Sensor konnte nicht aus der CSV Datei erstellt werden!\r\n{}", error))
                        }
                    }
                },
                "Sensor-MB-NAP5X_REV1_0" => {
                    // Lade Sensor Ansicht mit einer Messzelle
                    stack_sensor.set_visible_child_name("single_sensor");
                    clean_notebook_tabs(&notebook_sensor);
                    match SensorMbNap5x::new_from_csv() {
                        Ok(from_csv) => {
                            let from_csv = Box::new(from_csv);
                            // Setzt die Platine die in der GUI verwendet werden soll
                            set_platine(&platine, from_csv);
                            // Setzt den TreeStore der Lese Register
                            // Füllt den TreeStore mit Daten und zeigt die TreeViews der Hardware im Notebook-Sensor an
                            set_rreg_store(&rreg_store, platine.clone(), &notebook_sensor);

                            #[cfg(feature = "ra-gas")]
                            // Setzt den TreeStore der Schreib/Lese Register
                            // Füllt den TreeStore mit Daten und zeigt die TreeViews der Hardware im Notebook-Sensor an
                            set_rwreg_store(&rwreg_store, platine.clone(), &notebook_sensor, &gui_tx);
                        },
                        Err(error) => {
                            show_error(&gui_tx, &format!("Sensor konnte nicht aus der CSV Datei erstellt werden!\r\n{}", error))
                        }
                    }
                },
                "Sensor-MB-NAP5xx_REV1_0" => {
                    // Lade Sensor Ansicht mit 2facher Messzelle
                    stack_sensor.set_visible_child_name("duo_sensor");
                    clean_notebook_tabs(&notebook_sensor);
                    match SensorMbNap5xx::new_from_csv() {
                        Ok(from_csv) => {
                            let from_csv = Box::new(from_csv);
                            // Setzt die Platine die in der GUI verwendet werden soll
                            set_platine(&platine, from_csv);
                            // Setzt den TreeStore der Lese Register
                            // Füllt den TreeStore mit Daten und zeigt die TreeViews der Hardware im Notebook-Sensor an
                            set_rreg_store(&rreg_store, platine.clone(), &notebook_sensor);

                            #[cfg(feature = "ra-gas")]
                            // Setzt den TreeStore der Schreib/Lese Register
                            // Füllt den TreeStore mit Daten und zeigt die TreeViews der Hardware im Notebook-Sensor an
                            set_rwreg_store(&rwreg_store, platine.clone(), &notebook_sensor, &gui_tx);

                            // SI einheit Sensor1 (ppm)
                            label_sensor1_value_si.set_text("ppm");

                        },
                        Err(error) => {
                            show_error(&gui_tx, &format!("Sensor konnte nicht aus der CSV Datei erstellt werden!\r\n{}", error))
                        }
                    }
                },
                "Sensor-MB-NE4_REV1_0" => {
                    // Lade Sensor Ansicht mit einer Messzelle
                    stack_sensor.set_visible_child_name("single_sensor");
                    clean_notebook_tabs(&notebook_sensor);
                    match SensorMbNe4::new_from_csv() {
                        Ok(from_csv) => {
                            let from_csv = Box::new(from_csv);
                            // Setzt die Platine die in der GUI verwendet werden soll
                            set_platine(&platine, from_csv);
                            // Setzt den TreeStore der Lese Register
                            // Füllt den TreeStore mit Daten und zeigt die TreeViews der Hardware im Notebook-Sensor an
                            set_rreg_store(&rreg_store, platine.clone(), &notebook_sensor);

                            #[cfg(feature = "ra-gas")]
                            // Setzt den TreeStore der Schreib/Lese Register
                            // Füllt den TreeStore mit Daten und zeigt die TreeViews der Hardware im Notebook-Sensor an
                            set_rwreg_store(&rwreg_store, platine.clone(), &notebook_sensor, &gui_tx);
                        },
                        Err(error) => {
                            show_error(&gui_tx, &format!("Sensor konnte nicht aus der CSV Datei erstellt werden!\r\n{}", error))
                        }
                    }
                },
                "Sensor-MB-NE4-V1.0" => {
                    // Lade Sensor Ansicht mit einer Messzelle
                    stack_sensor.set_visible_child_name("single_sensor");
                    clean_notebook_tabs(&notebook_sensor);
                    match SensorMbNe4Legacy::new_from_csv() {
                        Ok(from_csv) => {
                            let from_csv = Box::new(from_csv);
                            // Setzt die Platine die in der GUI verwendet werden soll
                            set_platine(&platine, from_csv);
                            // Setzt den TreeStore der Lese Register
                            // Füllt den TreeStore mit Daten und zeigt die TreeViews der Hardware im Notebook-Sensor an
                            set_rreg_store(&rreg_store, platine.clone(), &notebook_sensor);

                            #[cfg(feature = "ra-gas")]
                            // Setzt den TreeStore der Schreib/Lese Register
                            // Füllt den TreeStore mit Daten und zeigt die TreeViews der Hardware im Notebook-Sensor an
                            set_rwreg_store(&rwreg_store, platine.clone(), &notebook_sensor, &gui_tx);
                        },
                        Err(error) => {
                            show_error(&gui_tx, &format!("Sensor konnte nicht aus der CSV Datei erstellt werden!\r\n{}", error))
                        }
                    }
                },
                "Sensor-MB-SP42A_REV1_0" => {
                    // Lade Sensor Ansicht mit einer Messzelle
                    stack_sensor.set_visible_child_name("single_sensor");
                    clean_notebook_tabs(&notebook_sensor);
                    match SensorMbSp42a::new_from_csv() {
                        Ok(from_csv) => {
                            let from_csv = Box::new(from_csv);
                            // Setzt die Platine die in der GUI verwendet werden soll
                            set_platine(&platine, from_csv);
                            // Setzt den TreeStore der Lese Register
                            // Füllt den TreeStore mit Daten und zeigt die TreeViews der Hardware im Notebook-Sensor an
                            set_rreg_store(&rreg_store, platine.clone(), &notebook_sensor);

                            #[cfg(feature = "ra-gas")]
                            // Setzt den TreeStore der Schreib/Lese Register
                            // Füllt den TreeStore mit Daten und zeigt die TreeViews der Hardware im Notebook-Sensor an
                            set_rwreg_store(&rwreg_store, platine.clone(), &notebook_sensor, &gui_tx);
                        },
                        Err(error) => {
                            show_error(&gui_tx, &format!("Sensor konnte nicht aus der CSV Datei erstellt werden!\r\n{}", error))
                        }
                    }
                },
                _ => {
                    // Lade Sensor Ansicht mit einer Messzelle
                    stack_sensor.set_visible_child_name("single_sensor");
                },
            };

            // Aktiviere GUI Elemente die nur mit ausgewähler Platine funktionieren
            button_duo_sensor1_messgas.set_sensitive(true);
            button_duo_sensor1_nullpunkt.set_sensitive(true);
            button_duo_sensor2_messgas.set_sensitive(true);
            button_duo_sensor2_nullpunkt.set_sensitive(true);
            button_messgas.set_sensitive(true);
            button_new_modbus_address.set_sensitive(true);
            button_nullpunkt.set_sensitive(true);
            check_button_mcs.set_sensitive(true);
            combo_box_text_sensor_working_mode.set_sensitive(true);
            spin_button_new_modbus_address.set_sensitive(true);
            toggle_button_connect.set_sensitive(true);
        }
    ));

    // Callback: Button Arbeitsweise
    button_sensor_working_mode.connect_clicked(clone!(
        @strong platine,
        @strong combo_box_text_ports,
        @strong combo_box_text_ports_map,
        @strong modbus_master_tx,
        @strong spin_button_modbus_address,
        @strong combo_box_text_sensor_working_mode,
        @strong gui_tx
        => move |_| {
            // FIXME: Refactor das!
            match platine.lock() {
                Ok(platine) => {
                    match platine.as_ref() {
                        Some(platine) => {
                            let active_port = combo_box_text_ports.get_active().unwrap_or(0);
                            // Extrahiert den Namen der Schnittstelle aus der HashMap, Key ist die Nummer der Schnittstelle
                            let mut tty_path = None;
                            for (p, i) in &*combo_box_text_ports_map.borrow() {
                                if *i == active_port {
                                    tty_path = Some(p.to_owned());
                                    break;
                                }
                            }
                            let tty_path = match tty_path {
                                Some(tty_path) => tty_path,
                                None => {
                                    show_error(&gui_tx, "Keine Schnittstelle gefunden!");
                                    return
                                }
                            };

                            // Extract Lock Register und TTY Pfad
                            let reg_protection = platine.reg_protection();

                            // get modbus_address
                            let slave = spin_button_modbus_address.get_value() as u8;
                            info!("tty_path: {:?}, slave: {:?}", &tty_path, &slave);

                            // Extrahiere aus dem ComboBoxText ein u16
                            match combo_box_text_sensor_working_mode.get_active_text() {
                                Some(working_mode) => {
                                    let working_mode = working_mode.split_terminator(" - ").collect::<Vec<&str>>();
                                    let working_mode: u16 = working_mode.first().unwrap_or(&"0").parse::<u16>().unwrap_or(0);

                                    // Sende Nachricht an Modbus Master
                                    match modbus_master_tx.clone()
                                    .try_send(ModbusMasterMessage::SetNewWorkingMode(
                                        tty_path,
                                        slave,
                                        working_mode,
                                        reg_protection
                                    )) {
                                        Ok(_) => {
                                            show_info(&gui_tx, &format!("Arbeitsweise erfolgreich gesetzt."));
                                        }
                                        Err(error) => {
                                            show_error(&gui_tx, &format!("Modbus Master konnte nicht erreicht werden: {}!", error));
                                        }
                                    }
                                },
                                None => {
                                    show_error(&gui_tx, &format!("Bitte Arbeitsweise auswählen!"));
                                },
                            };
                        },
                        None => {
                            show_error(&gui_tx, &format!("Es wurde keine Platine ausgewählt!"));
                        }
                    }
                },
                Err(error) => {
                    show_error(&gui_tx, &format!("Platine Mutex Lock konnte nicht entfernt werden:\r\n{}!", error));
                }
            }
        }
    ));

    // Callback: Menu About Quit
    menu_item_quit.connect_activate(clone!(
        @weak application_window => move |_| {
            application_window.close()
        }
    ));

    // Callback: Menu About
    menu_item_about.connect_activate(clone!(
        @strong about_dialog => move |_| {
            about_dialog.show()
        }
    ));

    // Callback: Menu About Ok
    about_dialog_button_ok.connect_clicked(clone!(
        @strong about_dialog => move |_| {
            about_dialog.hide()
        }
    ));

    // Callback: Infobars
    // TODO: Infobar callbacks sehen komisch aus, refactore them!
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

    //
    // Ende Callbacks
    //

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
        label_sensor_value_value,
        label_sensor1_value_value,
        label_sensor1_value_si,
        label_sensor2_value_value,
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
        check_button_mcs,
        spin_button_new_modbus_address,
        button_new_modbus_address,
        combo_box_text_hw_version,
        combo_box_text_sensor_working_mode,
        button_sensor_working_mode,
        button_nullpunkt,
        button_messgas,
        button_duo_sensor1_nullpunkt,
        button_duo_sensor1_messgas,
        button_duo_sensor2_nullpunkt,
        button_duo_sensor2_messgas,
    };

    application_window.show_all();

    // future on main thread has access to UI
    let future = {
        use futures::stream::StreamExt;

        async move {
            while let Some(event) = gui_rx.next().await {
                match event {
                    GuiMessage::DisableUiElements => {
                        debug!("Disable UI Elements");
                        gui.disable_ui_elements();
                    }
                    GuiMessage::EnableUiElements => {
                        debug!("Enable UI Elements");
                        gui.enable_ui_elements();
                    }
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
                    GuiMessage::ModbusMasterUpdateRegister {
                        reg_nr,
                        new_value,
                        // modbus_master_tx2,
                    } => {
                        let tty_path = match gui.get_tty_path() {
                            Some(tty_path) => tty_path,
                            None => {
                                gui.show_infobar_error(&format!(
                                    "Keine gültige Schnittstelle gewählt"
                                ));
                                return;
                            }
                        };
                        let slave = spin_button_modbus_address.get_value() as u8;
                        let reg_nr = match reg_nr {
                            Ok(reg_nr) => reg_nr,
                            Err(_) => {
                                gui.show_infobar_error("Register Nummer nicht lesbar!");
                                return;
                            }
                        };
                        let new_value = match new_value.parse::<u16>() {
                            Ok(new_value) => new_value,
                            Err(error) => {
                                gui.show_infobar_error(&format!(
                                    "Konnte neuen Wert nicht lesen: {}",
                                    error
                                ));
                                return;
                            }
                        };
                        let reg_protection: u16 = gui.platine_reg_protection();
                        let _ = modbus_master_tx.clone().try_send(
                            ModbusMasterMessage::UpdateRegister {
                                tty_path,
                                slave,
                                reg_nr,
                                reg_protection,
                                new_value,
                            },
                        );
                        debug!("ModbusMaster Update One Register:");
                    }
                    GuiMessage::UpdateSensorValues(results) => {
                        debug!("Update sensor values with: {:?}", &results);
                        gui.update_sensor_values(results);
                    }
                    GuiMessage::UpdateSerialPorts(ports) => {
                        debug!("Update Serial Ports with: {:?}", &ports);
                        gui.update_serial_ports(ports);
                    }
                    GuiMessage::UpdateRregs(results) => {
                        debug!("Update Rregs with: {:?}", &results);
                        gui.update_rreg_store(results);
                    }
                    GuiMessage::UpdateRwregs(results) => {
                        debug!("Update Rwregs with: {:?}", &results);
                        gui.update_rwreg_store(results);
                    }
                }
            }
        }
    };

    let c = glib::MainContext::default();
    c.spawn_local(future);
}

impl Gui {
    /// Disable UI elements
    ///
    /// Helper function disable User Interface elements
    fn disable_ui_elements(&self) {
        self.button_duo_sensor1_messgas.set_sensitive(false);
        self.button_duo_sensor1_nullpunkt.set_sensitive(false);
        self.button_duo_sensor2_messgas.set_sensitive(false);
        self.button_duo_sensor2_nullpunkt.set_sensitive(false);
        self.button_messgas.set_sensitive(false);
        self.button_new_modbus_address.set_sensitive(false);
        self.button_nullpunkt.set_sensitive(false);
        self.button_sensor_working_mode.set_sensitive(false);
        self.check_button_mcs.set_sensitive(false);
        self.combo_box_text_ports.set_sensitive(false);
        self.combo_box_text_sensor_working_mode.set_sensitive(false);
        self.spin_button_new_modbus_address.set_sensitive(false);
    }

    /// Enable UI elements
    ///
    /// Helper function enable User Interface elements
    fn enable_ui_elements(&self) {
        self.combo_box_text_ports.set_sensitive(true);
        #[cfg(feature = "ra-gas")]
        {
            // self.button_new_modbus_address.set_sensitive(true);
            // self.button_sensor_working_mode.set_sensitive(true);
            // self.check_button_mcs.set_sensitive(true);
            // self.combo_box_text_sensor_working_mode.set_sensitive(true);
            // self.spin_button_new_modbus_address.set_sensitive(true);
        }
        // self.button_nullpunkt.set_sensitive(true);
        // self.button_messgas.set_sensitive(true);
        // self.button_duo_sensor1_nullpunkt.set_sensitive(true);
        // self.button_duo_sensor1_messgas.set_sensitive(true);
        // self.button_duo_sensor2_nullpunkt.set_sensitive(true);
        // self.button_duo_sensor2_messgas.set_sensitive(true);
    }

    // Setzt die Serielle Schnittstelle
    fn select_port(&self, num: u32) {
        // Block signal handler
        signal::signal_handler_block(
            &self.combo_box_text_ports,
            &self.combo_box_text_ports_changed_signal,
        );
        // set serial interface
        &self.combo_box_text_ports.set_active(Some(num));
        // unblock signal handler
        signal::signal_handler_unblock(
            &self.combo_box_text_ports,
            &self.combo_box_text_ports_changed_signal,
        );
        // activate combo field and connect button
        &self.combo_box_text_ports.set_sensitive(true);
        if let Ok(platine) = &self.platine.lock() {
            if platine.is_some() {
                &self.toggle_button_connect.set_sensitive(true);
            }
        }
    }

    /// Update verfügbare serielle Schnittstellen
    ///
    /// Diese Funktion wird unter Anderem vom `SerialThread` aufgerufen wenn
    /// sich die Anzahl der gefunden seriellen Schnittstellen ändert.
    fn update_serial_ports(&self, ports: Vec<String>) {
        debug!(
            "ports: {:?}, active port: {:?}",
            &ports,
            self.combo_box_text_ports.get_active()
        );
        let active_port = self.combo_box_text_ports.get_active().unwrap_or(0);
        let old_num_ports = self.combo_box_text_ports_map.borrow().len();
        // Update the port listing and other UI elements
        self.combo_box_text_ports.remove_all();
        self.combo_box_text_ports_map.borrow_mut().clear();
        // no serial interface found
        if ports.is_empty() {
            debug!("kein Port gefunden",);
            self.combo_box_text_ports
                .append(None, "Keine Schnittstelle gefunden");
            self.combo_box_text_ports.set_active(Some(0));

            // Disable UI elements
            self.disable_ui_elements();
        // one or more serial ports found
        } else {
            for (i, p) in (0u32..).zip(ports.clone().into_iter()) {
                self.combo_box_text_ports.append(None, &p);
                self.combo_box_text_ports_map.borrow_mut().insert(p, i);
            }
            let num_ports = self.combo_box_text_ports_map.borrow().len();
            // lost one or more serial ports
            if num_ports < old_num_ports {
                debug!(
                    "lost one or more serial ports: active_port:{:?}, num_ports:{:?}, old_num_ports:{:?}",
                    active_port, num_ports, old_num_ports
                );
                // Restore selected serial interface
                let active_port = if active_port > 0 {
                    active_port - 1
                } else {
                    active_port
                };
                self.select_port(active_port);

                // Enable UI elements
                self.enable_ui_elements();

                // Statusbar message
                self.log_status(
                    StatusBarContext::PortOperation,
                    &format!(
                        "Schnittstelle verloren! Aktuelle Schnittstellen: {:?}",
                        ports
                    ),
                );
            // Additional serial port found
            } else if num_ports > old_num_ports {
                debug!(
                    "Port gefunden: active_port:{:?} num_ports:{:?} old_num_ports:{:?}",
                    active_port, num_ports, old_num_ports
                );
                // Enable UI elements
                self.enable_ui_elements();

                // Restore selected serial interface
                self.select_port(num_ports as u32 - 1);

                // Statusbar message
                self.log_status(
                    StatusBarContext::PortOperation,
                    &format!("Neue Schnittstelle gefunden: {:?}", ports),
                );
            // same serial ports as last time
            } else if num_ports == old_num_ports {
                debug!(
                    "no new serial ports found: active_port:{:?} num_ports:{:?} old_num_ports:{:?}",
                    active_port, num_ports, old_num_ports
                );
                // Restore selected serial interface
                self.select_port(active_port);
            }
        }
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
        label.set_markup(message);

        &self.infobar_info.show_all();
        &self.revealer_infobar_info.set_reveal_child(true);
    }

    /// Show InfoBar Warning
    ///
    fn show_infobar_warning(&self, message: &str) {
        let label = &self.label_infobar_warning_text;
        label.set_line_wrap(true);
        label.set_markup(message);

        &self.infobar_warning.show_all();
        &self.revealer_infobar_warning.set_reveal_child(true);
    }

    /// Show InfoBar Error> {
    /// }
    ///
    fn show_infobar_error(&self, message: &str) {
        let label = &self.label_infobar_error_text;
        label.set_line_wrap(true);
        label.set_markup(message);

        &self.infobar_error.show_all();
        &self.revealer_infobar_error.set_reveal_child(true);
    }

    /// Show InfoBar Question
    ///
    fn show_infobar_question(&self, message: &str) {
        let label = &self.label_infobar_question_text;
        label.set_line_wrap(true);
        label.set_markup(message);

        &self.infobar_question.show_all();
        &self.revealer_infobar_question.set_reveal_child(true);
    }

    /// Update SensorValues
    fn update_sensor_values(&self, result: Vec<(u16, u16)>) {
        if let Ok(platine) = self.platine.lock() {
            if let Some(platine) = &*platine {
                match platine.name() {
                "Sensor-MB-CO2_O2_REV1_0" => {
                    // Messzelle 1
                    if let Some((_, value)) = result.get(2) {
                        self.label_sensor1_value_value.set_text(&value.to_string());
                    }
                    // Messzelle 2
                    if let Some((_, value)) = result.get(6) {
                        self.label_sensor2_value_value.set_text(&value.to_string());
                    }
                },
                "Sensor-MB-NAP5x_REV1_0" => {
                    // Messzelle 1
                    if let Some((_, value)) = result.get(2) {
                        self.label_sensor_value_value.set_text(&value.to_string());
                    }
                },
                "Sensor-MB-NAP5xx_REV1_0" => {
                    // Messzelle 1
                    if let Some((_, value)) = result.get(2) {
                        self.label_sensor1_value_value.set_text(&value.to_string());
                    }
                    // Messzelle 2
                    if let Some((_, value)) = result.get(6) {
                        self.label_sensor2_value_value.set_text(&value.to_string());
                    }
                },
                "Sensor-MB-NE4_REV1_0" => {
                    // Messzelle 1
                    if let Some((_, value)) = result.get(2) {
                        self.label_sensor_value_value.set_text(&value.to_string());
                    }
                },
                "Sensor-MB-NE4-V1.0" => {
                    // Messzelle 1
                    if let Some((_, value)) = result.get(2) {
                        self.label_sensor_value_value.set_text(&value.to_string());
                    }
                },
                "Sensor-MB-SP42A_REV1_0" => {
                    // Messzelle 1
                    if let Some((_, value)) = result.get(2) {
                        self.label_sensor_value_value.set_text(&value.to_string());
                    }
                },
                _ => self.show_infobar_error("Nicht unterstützte Platine, Sensorwerte konnten nicht aktualisiert werden."),
                };
                if let Some((_, value)) = result.get(1) {
                    self.combo_box_text_sensor_working_mode.set_active_id(Some(&format!("{}", value)));
                }
            }
        }
    }

    /// Update RregStore
    fn update_rreg_store(&self, result: Vec<(u16, u16)>) {
        if let Ok(lock) = self.rreg_store.lock() {
            match *lock {
                Some(ref store) => store.update_treestore(result),
                None => {}
            }
        }
    }

    /// Update RwregStore
    fn update_rwreg_store(&self, result: Vec<(u16, u16)>) {
        if let Ok(lock) = self.rwreg_store.lock() {
            match *lock {
                Some(ref store) => store.update_treestore(result),
                None => {}
            }
        }
    }

    /// Register Nummer der Schreibschutzes
    ///
    /// Diese Funktion versucht aus dem Trait Objekt den Schreibschutz Register zu entpacken.
    fn platine_reg_protection(&self) -> u16 {
        let reg_protection = match self.platine.lock() {
            Ok(platine) => match platine.as_ref() {
                Some(platine) => platine.reg_protection(),
                None => platine::DEFAULT_REG_PROTECTION,
            },
            Err(_) => platine::DEFAULT_REG_PROTECTION,
        };
        reg_protection
    }

    /// Liefert die Schnittstelle in einem Result
    fn get_tty_path(&self) -> Option<String> {
        // tty path
        let active_port = self.combo_box_text_ports.get_active().unwrap_or(0);
        // Extrahiert den Namen der Schnittstelle aus der HashMap, Key ist die Nummer der Schnittstelle
        let mut tty_path = None;
        for (p, i) in &*self.combo_box_text_ports_map.borrow() {
            if *i == active_port {
                tty_path = Some(p.to_owned());
                break;
            }
        }
        tty_path
    }
} // Ende Gui Implementation

// Lösche Notebook alle bis auf den ersten Tab
//
// Diese Funktion löscht so lange die Tabs bis nur noch einer
// übrig ist.
fn clean_notebook_tabs(notebook: &gtk::Notebook) {
    while notebook.get_n_pages() > 1 {
        if let Some(child) = notebook.get_nth_page(None) {
            notebook.detach_tab(&child);
        }
    }
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
/// Bildet danach den Treestore und zeigt diesen im Notebook Widget an.
pub fn set_rreg_store(
    rreg_store: &BoxedRregStore,
    platine: BoxedPlatine,
    notebook: &gtk::Notebook,
) {
    let store = RregStore::new(platine);
    if let Ok(mut ptr) = rreg_store.lock() {
        let widget = store.fill_and_build_ui();
        notebook.add(&widget);
        notebook.set_tab_label_text(&widget, registers::REGISTER_TYPES[0].1);
        notebook.show_all();
        *ptr = Some(store);
    }
}

#[cfg(feature = "ra-gas")]
/// Setzt die Schreib/Lese Register TreeStore der in der GUI verwendet wird.
///
/// Bildet danach den Treestore und zeigt diesen im Notebook Widget an.
pub fn set_rwreg_store(
    rwreg_store: &BoxedRwregStore,
    platine: BoxedPlatine,
    notebook: &gtk::Notebook,
    gui_tx: &futures::channel::mpsc::Sender<GuiMessage>,
) {
    let store = RwregStore::new(platine);
    if let Ok(mut ptr) = rwreg_store.lock() {
        let widget = store.fill_and_build_ui(&gui_tx);
        notebook.add(&widget);
        notebook.set_tab_label_text(&widget, registers::REGISTER_TYPES[1].1);
        notebook.show_all();
        *ptr = Some(store);
    }
}

/// Info Infobar für Aufruf in Callbacks
///
/// In den Callbacks steht die Ui Struktur noch nicht zur Verfügung. So dass
/// deren Funktionen wie `Gui::show_infobar_info` in den Callbacks nicht
/// aufrufbar sind.
/// Diese Funktion sended über den gui_tx Channel eine Nachricht an die InfoBar.
pub fn show_info(tx: &mpsc::Sender<GuiMessage>, msg: &str) {
    let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    tx.clone()
        .try_send(GuiMessage::ShowInfo(format!(
            "<b>{}:</b> {}",
            timestamp,
            msg.to_string()
        )))
        .expect(r#"Failed to send Message"#);
}

/// Warning Infobar für Aufruf in Callbacks
///
/// In den Callbacks steht die Ui Struktur noch nicht zur Verfügung. So dass
/// deren Funktionen wie `Gui::show_infobar_info` in den Callbacks nicht
/// aufrufbar sind.
/// Diese Funktion sended über den gui_tx Channel eine Nachricht an die InfoBar.
pub fn show_warning(tx: &mpsc::Sender<GuiMessage>, msg: &str) {
    let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    tx.clone()
        .try_send(GuiMessage::ShowWarning(format!(
            "<b>{}:</b> {}",
            timestamp,
            msg.to_string()
        )))
        .expect(r#"Failed to send Message"#);
}

/// Error> {
/// } Infobar für Aufruf in Callbacks
///
/// In den Callbacks steht die Ui Struktur noch nicht zur Verfügung. So dass
/// deren Funktionen wie `Gui::show_infobar_info` in den Callbacks nicht
/// aufrufbar sind.
/// Diese Funktion sended über den gui_tx Channel eine Nachricht an die InfoBar.
pub fn show_error(tx: &mpsc::Sender<GuiMessage>, msg: &str) {
    let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    tx.clone()
        .try_send(GuiMessage::ShowError(format!(
            "<b>{}:</b> {}",
            timestamp,
            msg.to_string()
        )))
        .expect(r#"Failed to send Message"#);
}

/// Question Infobar für Aufruf in Callbacks
///
/// In den Callbacks steht die Ui Struktur noch nicht zur Verfügung. So dass
/// deren Funktionen wie `Gui::show_infobar_info` in den Callbacks nicht
/// aufrufbar sind.
/// Diese Funktion sended über den gui_tx Channel eine Nachricht an die InfoBar.
pub fn _show_question(tx: &mpsc::Sender<GuiMessage>, msg: &str) {
    let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    tx.clone()
        .try_send(GuiMessage::ShowQuestion(format!(
            "<b>{}:</b> {}",
            timestamp,
            msg.to_string()
        )))
        .expect(r#"Failed to send Message"#);
}
