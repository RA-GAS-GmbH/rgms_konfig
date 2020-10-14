use crate::gui::gtk3::GuiMessage;
use futures::channel::mpsc;
use futures::SinkExt;
use std::time::Duration;
use tokio::{runtime::Runtime, time::interval};

/// Datenstruktur für den SerialInterface Thread
pub struct SerialInterface {}

impl SerialInterface {
    /// Erzeugt den SerialInterface Thread
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

/// Liste der verfügbaren seriellen Schnittstellen
pub fn list_ports() -> tokio_serial::Result<Vec<String>> {
    match tokio_serial::available_ports() {
        Ok(ports) => Ok(ports.into_iter().map(|x| x.port_name).collect()),
        Err(e) => Err(e),
    }
}

/// Filtert die verfügbaren seiellen Schnittstellen
///
/// Diese Funktion wird im Gui Thread aufgerufen.
/// **Unter Linux wird die, nicht nutzbare, Schnittstelle `/dev/ttyS0` entfernt!**
pub fn get_ports() -> Vec<String> {
    let mut ports = list_ports().expect("Scanning for ports should never fail");
    ports.sort();
    // Remove unwanted ports under linux
    ports.retain(|p| p != "/dev/ttyS0");

    ports
}
