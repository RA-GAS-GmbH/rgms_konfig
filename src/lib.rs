//! GUI für die Konfiguration der Sensoren vom Typ 'RA-GAS Modbus System'

#![deny(missing_docs)]

#[macro_use]
extern crate log;

/// GUI Komponenten
pub mod gui {
    pub mod gtk3;
}
pub mod registers;

pub mod platine;

pub mod modbus_master;

pub mod serial_interface;
