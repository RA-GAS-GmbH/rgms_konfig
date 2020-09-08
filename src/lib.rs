//! GUI für die Konfiguration der Sensoren vom Typ 'RA-GAS Modbus System'
//!
#![deny(missing_docs)]

/// Graphical User Interface
pub mod gui {
    /// Gtk3+ User Interface
    pub mod gtk3;
}
/// Modbus Registers
pub mod registers;

/// Unterstützte Sensoren
pub mod sensors;

/// Modbus Master
pub mod modbus_master;