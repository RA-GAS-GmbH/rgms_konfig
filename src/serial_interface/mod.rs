use crate::gui::gtk3::GuiMessage;
use futures::channel::mpsc;
use futures::SinkExt;
use std::time::Duration;
use tokio::{runtime::Runtime, time::interval};

pub struct SerialInterface {}

impl SerialInterface {
    pub fn new(gui_tx: mpsc::Sender<GuiMessage>) -> Self {
        std::thread::spawn(move || {
            let mut rt = Runtime::new().expect("create tokio runtime");

            rt.block_on(async {
                let mut ports: Vec<String> = vec![];
                let mut interval = interval(Duration::from_millis(100));

                let available_ports = get_ports();
                let _ = gui_tx
                    .clone()
                    .send(GuiMessage::UpdateSerialPorts(available_ports.clone()))
                    .await;

                loop {
                    let available_ports = get_ports();
                    // Hier könnte die Logic stehen für "neuen Port gefunden", "Port wurde entfernt"
                    if available_ports.len() != ports.len() {
                        let _ = gui_tx
                            .clone()
                            .send(GuiMessage::UpdateSerialPorts(available_ports.clone()))
                            .await;
                    };
                    ports = available_ports;
                    interval.tick().await;
                }
            });
        });

        SerialInterface {}
    }
}

/// List available serial ports
pub(crate) fn list_ports() -> tokio_serial::Result<Vec<String>> {
    match tokio_serial::available_ports() {
        Ok(ports) => Ok(ports.into_iter().map(|x| x.port_name).collect()),
        Err(e) => Err(e),
    }
}

/// Get and filter available serial ports
///
/// This function is called from the gui thread.
pub(crate) fn get_ports() -> Vec<String> {
    let mut ports = list_ports().expect("Scanning for ports should never fail");
    ports.sort();
    // Remove unwanted ports under linux
    ports.retain(|p| p != "/dev/ttyS0");

    ports
}
