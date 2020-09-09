use crate::{modbus_master::ModbusMaster, platine};
use futures::channel::mpsc;
use gio::prelude::*;
use glib::clone;
use gtk::{prelude::*, Application};
use std::collections::HashMap;

#[macro_use]
mod macros;
mod rreg_store;
mod rwreg_store;

// Reexport
pub use rreg_store::RregStore;
pub use rwreg_store::RwregStore;

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
pub struct Gui {}

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
    let (_gui_tx, mut gui_rx) = mpsc::channel(0);
    let modbus_master = ModbusMaster::new();
    let _modbus_master_tx = modbus_master.tx;

    let glade_str = include_str!("rgms_konfig.ui");
    let builder = gtk::Builder::from_string(glade_str);
    let application_window: gtk::ApplicationWindow = build!(builder, "application_window");
    // Infobars
    let _revealer_infobar_info: gtk::Revealer = build!(builder, "revealer_infobar_info");
    let infobar_info: gtk::InfoBar = build!(builder, "infobar_info");
    let infobar_warning: gtk::InfoBar = build!(builder, "infobar_warning");
    let infobar_error: gtk::InfoBar = build!(builder, "infobar_error");
    let infobar_question: gtk::InfoBar = build!(builder, "infobar_question");
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
    combo_box_text_hw_version.connect_changed(clone!(
        @strong combo_box_text_hw_version
        => move |s| {
            println!("Signal: {:?}", s.get_active_text().unwrap().to_string());
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

    application_window.show_all();

    // future on main thread has access to UI
    let future = {
        use futures::stream::StreamExt;

        async move {
            while let Some(event) = gui_rx.next().await {
                match event {
                    GuiMessage::ShowInfo(_) => {}
                    GuiMessage::ShowError(_) => {}
                }
            }
        }
    };

    let c = glib::MainContext::default();
    c.spawn_local(future);
}
