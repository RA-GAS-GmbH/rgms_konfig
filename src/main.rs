#![windows_subsystem = "windows"]

#[macro_use]
extern crate log;

fn main() {
    pretty_env_logger::init();

    info!("Starte Grafische Oberfläche");
    rgms_konfig::gui::gtk3::launch();
}
