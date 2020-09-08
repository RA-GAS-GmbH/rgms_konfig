#![windows_subsystem = "windows"]
use rgms_konfig;
#[macro_use]
extern crate log;

fn main() {
    pretty_env_logger::init();

    info!("Launch GUI");
    rgms_konfig::gui::gtk3::launch();
}
