//! # Basic Sample
//!
//! This sample demonstrates how to create a toplevel `window`, set its title, size and
//! position, how to add a `button` to this `window` and how to connect signals with
//! actions.

extern crate gio;
extern crate gtk;
extern crate rgms_konfig;

use gio::prelude::*;
use gtk::prelude::*;
use rgms_konfig::{gui::gtk3::RregStore, platine::SensorMbCo2O2};

use std::env::args;

fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);
    window.set_border_width(10);
    window.set_position(gtk::WindowPosition::Center);
    window.set_default_size(800, 600);

    let platine = Box::new(SensorMbCo2O2::new_from_csv().unwrap());
    let rreg_store = RregStore::new();
    let rreg_store_ui = rreg_store.build_ui(platine);

    window.add(&rreg_store_ui);

    window.show_all();
}

fn main() {
    let application = gtk::Application::new(Some("com.ra-gas.test.rreg_store"), Default::default())
        .expect("Initialization failed...");

    application.connect_activate(|app| {
        build_ui(app);
    });

    application.run(&args().collect::<Vec<_>>());
}
