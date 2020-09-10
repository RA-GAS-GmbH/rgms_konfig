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
