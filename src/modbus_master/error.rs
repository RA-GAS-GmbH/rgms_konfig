use std::{fmt, io};

/// Fehler die bei der Komunikation mit den Modbus Servern auftreten können.
#[derive(Debug)]
pub enum ModbusMasterError {
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
            ModbusMasterError::IoError(ref _error) => write!(f, "Io Error"),
            ModbusMasterError::ReadRreg => write!(f, "Fehler beim Lesen der Lese Register"),
            ModbusMasterError::ReadInputRegister => {
                write!(f, "Modbus Fehler beim Lesen der Lese Input Register")
            }
            ModbusMasterError::ReadHoldingRegister(reg_nr, ref e) => write!(
                f,
                "Modbus Fehler beim Lesen der Schreib/Lese Input Registers {}: {:?}",
                reg_nr, e
            ),
        }
    }
}

impl From<io::Error> for ModbusMasterError {
    fn from(error: io::Error) -> Self {
        ModbusMasterError::IoError(error)
    }
}

impl std::error::Error for ModbusMasterError {}
