//! GUI für die Konfiguration der Sensoren vom Typ 'RA-GAS Modbus System'
//!
#![deny(missing_docs)]

#[macro_use]
extern crate log;

/// Grafische Benutzer Oberfläche
pub mod gui {
    /// Gtk3+ Benutzer Schnittstelle
    pub mod gtk3;
}
/// Modbus Register
pub mod registers;

/// Unterstützte Platinen
pub mod platine;

/// Modbus Master
///
/// Der ModbusMaster dient der Kommunikation mit den Modbus Servern.
pub mod modbus_master;

/// Serial Interface Thread
///
/// Ein separater Thread der ständig, im Hintergrund, die verfügbaren seriellen
/// Schnittstellen überprüft.
/// Werden neue Schnittstellen gefunden oder werden Schnittstellen vom System
/// entfernt dann sendet dieser Thread Nachrichten an die Gui.
pub mod serial_interface;
