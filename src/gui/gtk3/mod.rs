#[macro_use]
mod macros;
mod rreg_store;
mod rwreg_store;
// Reexports
pub use rreg_store::RregStore;
pub use rwreg_store::RwregStore;

use crate::{
    modbus_master::ModbusMaster,
    platine::{self, *},
    registers,
    serial_interface::SerialInterface,
};
use futures::channel::mpsc;
use gio::prelude::*;
use glib::clone;
use gtk::{prelude::*, Application, NotebookExt};
use std::collections::HashMap;

const PKG_VERSION: &'static str = env!("CARGO_PKG_VERSION");
const PKG_NAME: &'static str = env!("CARGO_PKG_NAME");
const PKG_DESCRIPTION: &'static str = env!("CARGO_PKG_DESCRIPTION");

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum StatusContext {
    PortOperation,
    _Error,
}

/// Kommandos an die Grafische Schnittstelle
#[derive(Debug)]
pub enum GuiMessage {
    /// Zeige Infobar mit Fehlermeldung
    ShowError(String),
    /// Zeige Infobar mit Information an den Benutzer
    ShowInfo(String),
}

/// Representation der Grafischen Schnittstelle
pub struct Gui {
    infobar_info: gtk::InfoBar,
    revealer_infobar_info: gtk::Revealer,
    label_infobar_info_text: gtk::Label,
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
    let modbus_master = ModbusMaster::new();
    let _modbus_master_tx = modbus_master.tx;
    // Serial Interface Thread
    let _serial_interface = SerialInterface::new();

    let glade_str = include_str!("rgms_konfig.ui");
    let builder = gtk::Builder::from_string(glade_str);
    let application_window: gtk::ApplicationWindow = build!(builder, "application_window");
    // Infobars
    let infobar_info: gtk::InfoBar = build!(builder, "infobar_info");
    let infobar_warning: gtk::InfoBar = build!(builder, "infobar_warning");
    let infobar_error: gtk::InfoBar = build!(builder, "infobar_error");
    let infobar_question: gtk::InfoBar = build!(builder, "infobar_question");
    let revealer_infobar_info: gtk::Revealer = build!(builder, "revealer_infobar_info");
    let label_infobar_info_text: gtk::Label = build!(builder, "label_infobar_info_text");

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

    // Statusbar
    let statusbar_application: gtk::Statusbar = build!(builder, "statusbar_application");
    let context_id_port_ops = statusbar_application.get_context_id("port operations");
    let _context_map: HashMap<StatusContext, u32> =
        [(StatusContext::PortOperation, context_id_port_ops)]
            .iter()
            .cloned()
            .collect();

    let combo_box_text_hw_version: gtk::ComboBoxText = build!(builder, "combo_box_text_hw_version");
    for (id, name, _desc) in platine::HW_VERSIONS {
        combo_box_text_hw_version.append(Some(&id.to_string()), name);
    }

    let combo_box_text_sensor_working_mode: gtk::ComboBoxText =
        build!(builder, "combo_box_text_sensor_working_mode");
    for (id, name) in platine::WORKING_MODES {
        combo_box_text_sensor_working_mode.append(Some(&id.to_string()), &name);
    }

    let _toggle_button_connect: gtk::ToggleButton = build!(builder, "toggle_button_connect");

    let menu_item_quit: gtk::MenuItem = build!(builder, "menu_item_quit");
    let menu_item_about: gtk::MenuItem = build!(builder, "menu_item_about");

    let header_bar: gtk::HeaderBar = build!(builder, "header_bar");
    let about_dialog: gtk::AboutDialog = build!(builder, "about_dialog");
    let about_dialog_button_ok: gtk::Button = build!(builder, "about_dialog_button_ok");

    header_bar.set_title(Some(PKG_NAME));
    #[cfg(feature = "ra-gas")]
    header_bar.set_title(Some(&format!("{} - RA-GAS intern!", PKG_NAME)));
    header_bar.set_subtitle(Some(PKG_VERSION));

    about_dialog.set_program_name(PKG_NAME);
    #[cfg(feature = "ra-gas")]
    about_dialog.set_program_name(&format!("{} - RA-GAS intern!", PKG_NAME));
    about_dialog.set_version(Some(PKG_VERSION));
    about_dialog.set_comments(Some(PKG_DESCRIPTION));

    let _check_button_mcs: gtk::CheckButton = build!(builder, "check_button_mcs");

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
    button_nullpunkt.connect_clicked(clone!(
        @strong gui_tx
        => move |_| {
            // Test Send Message an Infobar::Infor
            // gui_tx.clone().try_send(GuiMessage::ShowInfo("Lorem ipsum dolor sit amet consectetur, adipisicing elit. Aperiam eveniet nulla quam ea, saepe ut a quia blanditiis veniam voluptate expedita quidem at rerum est! Quaerat ratione incidunt sunt nisi.".to_string())).expect(r#"Failed to send Message"#);
        }
    ));

    button_messgas.connect_clicked(clone!(
        @strong gui_tx
        => move |_| {
        }
    ));

    // Wird diese Auswahlbox selectiert werden die Anzeigen der Sensorwerte
    // entsprechend angepasst.
    combo_box_text_hw_version.connect_changed(clone!(
        @strong notebook_sensor,
        @strong stack_sensor,
        @strong box_single_sensor,
        @strong box_duo_sensor,
        @strong combo_box_text_hw_version
        => move |s| {
            match s.get_active_text().unwrap().as_str() {
                "Sensor-MB-CO2_O2_REV1_0" => {
                    // Load Sensor View mit 2facher Messzelle
                    stack_sensor.set_visible_child_name("duo_sensor");
                    // Lösche Notebook Tabs wenn schon 3 angezeigt werden
                    if notebook_sensor.get_n_pages() == 3 {
                        let child = notebook_sensor.get_nth_page(None).unwrap();
                        notebook_sensor.detach_tab(&child);
                        let child = notebook_sensor.get_nth_page(None).unwrap();
                        notebook_sensor.detach_tab(&child);
                    };
                    // TODO: implement Gui struct and add member rreg: Option<dyn Platine>
                    let platine = Box::new(SensorMbCo2O2::new_from_csv().unwrap());
                    let rreg_store = RregStore::new();
                    let rreg_store_ui = rreg_store.build_ui(platine);
                    notebook_sensor.add(&rreg_store_ui);
                    notebook_sensor.set_tab_label_text(&rreg_store_ui, registers::REGISTER_TYPES[0].1);

                    // TODO: implement Gui struct and add member rwreg: Option<dyn Platine>
                    let platine = Box::new(SensorMbCo2O2::new_from_csv().unwrap());
                    let rwreg_store = RwregStore::new();
                    let rwreg_store_ui = rwreg_store.build_ui(platine);
                    notebook_sensor.add(&rwreg_store_ui);
                    notebook_sensor.set_tab_label_text(&rwreg_store_ui, registers::REGISTER_TYPES[1].1);

                    notebook_sensor.show_all();
                }
                "Sensor-MB-NAP5X_REV1_0" => {
                    stack_sensor.set_visible_child_name("single_sensor");

                    // Lösche Notebook Tabs wenn schon 3 angezeigt werden
                    if notebook_sensor.get_n_pages() == 3 {
                        let child = notebook_sensor.get_nth_page(None).unwrap();
                        notebook_sensor.detach_tab(&child);
                        let child = notebook_sensor.get_nth_page(None).unwrap();
                        notebook_sensor.detach_tab(&child);
                    };
                    // TODO: implement Gui struct and add member rreg: Option<dyn Platine>
                    let platine = Box::new(SensorMbNap5x::new_from_csv().unwrap());
                    let rreg_store = RregStore::new();
                    let rreg_store_ui = rreg_store.build_ui(platine);
                    notebook_sensor.add(&rreg_store_ui);
                    notebook_sensor.set_tab_label_text(&rreg_store_ui, registers::REGISTER_TYPES[0].1);

                    // TODO: implement Gui struct and add member rwreg: Option<dyn Platine>
                    let platine = Box::new(SensorMbNap5x::new_from_csv().unwrap());
                    let rwreg_store = RwregStore::new();
                    let rwreg_store_ui = rwreg_store.build_ui(platine);
                    notebook_sensor.add(&rwreg_store_ui);
                    notebook_sensor.set_tab_label_text(&rwreg_store_ui, registers::REGISTER_TYPES[1].1);

                    notebook_sensor.show_all();
                }
                "Sensor-MB-NAP5xx_REV1_0" => {
                    // Load Sensor View mit 2facher Messzelle
                    stack_sensor.set_visible_child_name("duo_sensor");

                    // Lösche Notebook Tabs wenn schon 3 angezeigt werden
                    if notebook_sensor.get_n_pages() == 3 {
                        let child = notebook_sensor.get_nth_page(None).unwrap();
                        notebook_sensor.detach_tab(&child);
                        let child = notebook_sensor.get_nth_page(None).unwrap();
                        notebook_sensor.detach_tab(&child);
                    };
                    // TODO: implement Gui struct and add member rreg: Option<dyn Platine>
                    let platine = Box::new(SensorMbNap5xx::new_from_csv().unwrap());
                    let rreg_store = RregStore::new();
                    let rreg_store_ui = rreg_store.build_ui(platine);
                    notebook_sensor.add(&rreg_store_ui);
                    notebook_sensor.set_tab_label_text(&rreg_store_ui, registers::REGISTER_TYPES[0].1);

                    // TODO: implement Gui struct and add member rwreg: Option<dyn Platine>
                    let platine = Box::new(SensorMbNap5xx::new_from_csv().unwrap());
                    let rwreg_store = RwregStore::new();
                    let rwreg_store_ui = rwreg_store.build_ui(platine);
                    notebook_sensor.add(&rwreg_store_ui);
                    notebook_sensor.set_tab_label_text(&rwreg_store_ui, registers::REGISTER_TYPES[1].1);

                    notebook_sensor.show_all();
                }
                "Sensor-MB-NE4_REV1_0" => {
                    stack_sensor.set_visible_child_name("single_sensor");

                    // Lösche Notebook Tabs wenn schon 3 angezeigt werden
                    if notebook_sensor.get_n_pages() == 3 {
                        let child = notebook_sensor.get_nth_page(None).unwrap();
                        notebook_sensor.detach_tab(&child);
                        let child = notebook_sensor.get_nth_page(None).unwrap();
                        notebook_sensor.detach_tab(&child);
                    };
                    // TODO: implement Gui struct and add member rreg: Option<dyn Platine>
                    let platine = Box::new(SensorMbNe4::new_from_csv().unwrap());
                    let rreg_store = RregStore::new();
                    let rreg_store_ui = rreg_store.build_ui(platine);
                    notebook_sensor.add(&rreg_store_ui);
                    notebook_sensor.set_tab_label_text(&rreg_store_ui, registers::REGISTER_TYPES[0].1);

                    // TODO: implement Gui struct and add member rwreg: Option<dyn Platine>
                    let platine = Box::new(SensorMbNe4::new_from_csv().unwrap());
                    let rwreg_store = RwregStore::new();
                    let rwreg_store_ui = rwreg_store.build_ui(platine);
                    notebook_sensor.add(&rwreg_store_ui);
                    notebook_sensor.set_tab_label_text(&rwreg_store_ui, registers::REGISTER_TYPES[1].1);

                    notebook_sensor.show_all();
                }
                "Sensor-MB-NE4-V1.0" => {
                    stack_sensor.set_visible_child_name("single_sensor");

                    // Lösche Notebook Tabs wenn schon 3 angezeigt werden
                    if notebook_sensor.get_n_pages() == 3 {
                        let child = notebook_sensor.get_nth_page(None).unwrap();
                        notebook_sensor.detach_tab(&child);
                        let child = notebook_sensor.get_nth_page(None).unwrap();
                        notebook_sensor.detach_tab(&child);
                    };
                    // TODO: implement Gui struct and add member rreg: Option<dyn Platine>
                    let platine = Box::new(SensorMbNe4Legacy::new_from_csv().unwrap());
                    let rreg_store = RregStore::new();
                    let rreg_store_ui = rreg_store.build_ui(platine);
                    notebook_sensor.add(&rreg_store_ui);
                    notebook_sensor.set_tab_label_text(&rreg_store_ui, registers::REGISTER_TYPES[0].1);

                    // TODO: implement Gui struct and add member rwreg: Option<dyn Platine>
                    let _platine = Box::new(SensorMbNe4Legacy::new_from_csv().unwrap());
                    let platine = Box::new(SensorMbNe4::new_from_csv().unwrap());
                    let rwreg_store = RwregStore::new();
                    let rwreg_store_ui = rwreg_store.build_ui(platine);
                    notebook_sensor.add(&rwreg_store_ui);
                    notebook_sensor.set_tab_label_text(&rwreg_store_ui, registers::REGISTER_TYPES[1].1);

                    notebook_sensor.show_all();
                }
                "Sensor-MB-SP42A_REV1_0" => {
                    stack_sensor.set_visible_child_name("single_sensor");

                    // Lösche Notebook Tabs wenn schon 3 angezeigt werden
                    if notebook_sensor.get_n_pages() == 3 {
                        let child = notebook_sensor.get_nth_page(None).unwrap();
                        notebook_sensor.detach_tab(&child);
                        let child = notebook_sensor.get_nth_page(None).unwrap();
                        notebook_sensor.detach_tab(&child);
                    };
                    // TODO: implement Gui struct and add member rreg: Option<dyn Platine>
                    let platine = Box::new(SensorMbSp42a::new_from_csv().unwrap());
                    let rreg_store = RregStore::new();
                    let rreg_store_ui = rreg_store.build_ui(platine);
                    notebook_sensor.add(&rreg_store_ui);
                    notebook_sensor.set_tab_label_text(&rreg_store_ui, registers::REGISTER_TYPES[0].1);

                    // TODO: implement Gui struct and add member rwreg: Option<dyn Platine>
                    let platine = Box::new(SensorMbSp42a::new_from_csv().unwrap());
                    let rwreg_store = RwregStore::new();
                    let rwreg_store_ui = rwreg_store.build_ui(platine);
                    notebook_sensor.add(&rwreg_store_ui);
                    notebook_sensor.set_tab_label_text(&rwreg_store_ui, registers::REGISTER_TYPES[1].1);

                    notebook_sensor.show_all();
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

    let gui = Gui {
        infobar_info,
        revealer_infobar_info,
        label_infobar_info_text,
    };

    application_window.show_all();

    // future on main thread has access to UI
    let future = {
        use futures::stream::StreamExt;

        async move {
            while let Some(event) = gui_rx.next().await {
                match event {
                    GuiMessage::ShowInfo(msg) => {
                        show_info(&gui, &msg);
                    }
                    GuiMessage::ShowError(msg) => {
                        println!("{}", msg);
                    }
                }
            }
        }
    };

    let c = glib::MainContext::default();
    c.spawn_local(future);
}

/// Show InfoBar Info
///
fn show_info(gui: &Gui, message: &str) {
    let label = &gui.label_infobar_info_text;
    label.set_line_wrap(true);
    label.set_text(message);

    &gui.infobar_info.show_all();
    &gui.revealer_infobar_info.set_reveal_child(true);
}
