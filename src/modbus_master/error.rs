use libmodbus::prelude::Error as LibModbusError;
use std::{fmt, io};

/// Fehler die bei der Komunikation mit den Modbus Servern auftreten können.
#[derive(Debug)]
pub enum ModbusMasterError {
    /// Eingabe/ Ausgabe Fehler
    IoError(io::Error),
    /// Libmodbus Fehler
    LibModbusError(LibModbusError),
    /// Ein Fehler bei Auslesen der Lese Register ist aufgetreten
    ReadRreg,
    /// Fehler bei der Modbus Kommunikation, ein Lese Register konnte nicht gelesen werden
    ReadInputRegister {
        /// Register Nummer
        reg_nr: u16,
        /// Libmodbus Error
        source: LibModbusError,
    },
    /// Fehler bei der Modbus Kommunikation, ein Schreib/Lese Register konnte nicht gelesen werden
    ReadHoldingRegister {
        /// Register Nummer
        reg_nr: u16,
        /// Libmodbus Error
        source: LibModbusError,
    },
}

impl fmt::Display for ModbusMasterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            ModbusMasterError::IoError(ref error) => write!(f, "Io Fehler: {:?}", error),
            ModbusMasterError::LibModbusError(ref error) => {
                write!(f, "Libmodbus Fehler: {:?}", error)
            }
            ModbusMasterError::ReadRreg => write!(f, "Fehler beim Lesen der Lese Register"),
            ModbusMasterError::ReadInputRegister { reg_nr, source: _ } => write!(
                f,
                "Modbus Fehler beim Lesen des Input Registers: {}",
                reg_nr
            ),
            ModbusMasterError::ReadHoldingRegister { reg_nr, source: _ } => write!(
                f,
                "Modbus Fehler beim Lesen der Schreib/Lese Input Registers {}",
                reg_nr
            ),
        }
    }
}

impl From<io::Error> for ModbusMasterError {
    fn from(error: io::Error) -> Self {
        ModbusMasterError::IoError(error)
    }
}

impl From<LibModbusError> for ModbusMasterError {
    fn from(error: LibModbusError) -> Self {
        ModbusMasterError::LibModbusError(error)
    }
}

impl std::error::Error for ModbusMasterError {}
