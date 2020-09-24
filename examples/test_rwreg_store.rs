//! # Test gui::gtk3::rwreg_store::build_ui()
//!
//! Da das Crate 'gtk-test' nicht richtig funktioniert wurde dieses Beispiel als
//! Integrationstest verwendet.

extern crate gio;
extern crate gtk;
extern crate rgms_konfig;

use gio::prelude::*;
use gtk::prelude::*;
use rgms_konfig::{gui::gtk3::RwregStore, platine::SensorMbCo2O2};

use std::env::args;

fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);
    window.set_border_width(10);
    window.set_position(gtk::WindowPosition::Center);
    window.set_default_size(800, 600);

    let platine = Box::new(SensorMbCo2O2::new_from_csv().unwrap());
    let rwreg_store = RwregStore::new();
    let rwreg_store_ui = rwreg_store.build_ui(platine);

    window.add(&rwreg_store_ui);

    window.show_all();
}

fn main() {
    let application =
        gtk::Application::new(Some("com.ra-gas.test.rwreg_store"), Default::default())
            .expect("Initialization failed...");

    application.connect_activate(|app| {
        build_ui(app);
    });

    application.run(&args().collect::<Vec<_>>());
}