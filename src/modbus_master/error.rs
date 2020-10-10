use std::{fmt, io::Error};

/// Fehler die bei der Komunikation mit den Modbus Servern auftreten können.
#[derive(Debug)]
pub enum ModbusMasterError {
    /// Eingabe/ Ausgabe Fehler
    IoError(Error),
    /// Ein Fehler bei Auslesen der Lese Register ist aufgetreten
    ReadRreg,
    /// Fehler bei der Mobus Komunikation, ein Lese Register konnte nicht gelesen werden
    ReadInputRegister,
}

impl fmt::Display for ModbusMasterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            ModbusMasterError::IoError(ref _error) => write!(f, "Io Error"),
            ModbusMasterError::ReadRreg => write!(f, "Fehler beim Lesen der Lese Register"),
            ModbusMasterError::ReadInputRegister => {
                write!(f, "Modbus Fehler beim Lesen der Lese Input Register")
            }
        }
    }
}

impl From<Error> for ModbusMasterError {
    fn from(error: Error) -> Self {
        ModbusMasterError::IoError(error)
    }
}

impl std::error::Error for ModbusMasterError {}
