use crate::modbus_master::context_error;
use std::{fmt, io};

/// Fehler die bei der Komunikation mit den Modbus Servern auftreten können.
#[derive(Debug)]
pub enum ModbusMasterError {
    /// Fehler im ModbusContext
    ContextError(context_error::ContextError),
    /// Eingabe/ Ausgabe Fehler
    IoError(io::Error),
    /// Ein Fehler bei Auslesen der Lese Register ist aufgetreten
    ReadRreg,
    /// Fehler bei der Modbus Kommunikation, ein Lese Register konnte nicht gelesen werden
    ReadInputRegister,
    /// Fehler bei der Modbus Kommunikation, ein Schreib/Lese Register konnte nicht gelesen werden
    ReadHoldingRegister(u16, io::Error),
}

impl fmt::Display for ModbusMasterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            ModbusMasterError::ContextError(ref error) => {
                write!(f, "Modbus Context Fehler: {}", error)
            }
            ModbusMasterError::IoError(ref error) => write!(f, "Io Fehler: {:?}", error),
            ModbusMasterError::ReadRreg => write!(f, "Fehler beim Lesen der Lese Register"),
            ModbusMasterError::ReadInputRegister => {
                write!(f, "Modbus Fehler beim Lesen der Lese Input Register")
            }
            ModbusMasterError::ReadHoldingRegister(reg_nr, ref error) => write!(
                f,
                "Modbus Fehler beim Lesen der Schreib/Lese Input Registers {}: {:?}",
                reg_nr, error
            ),
        }
    }
}

impl From<io::Error> for ModbusMasterError {
    fn from(error: io::Error) -> Self {
        ModbusMasterError::IoError(error)
    }
}

impl From<context_error::ContextError> for ModbusMasterError {
    fn from(error: context_error::ContextError) -> Self {
        ModbusMasterError::ContextError(error)
    }
}

impl std::error::Error for ModbusMasterError {}
