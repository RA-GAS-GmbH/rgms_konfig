use std::{fmt, io};
use libmodbus::ModbusRtuError as LibModbusRtuError;
use libmodbus::prelude::Error as LibModbusError;

/// Fehler die bei der Komunikation mit den Modbus Servern auftreten können.
#[derive(Debug)]
pub enum ModbusMasterError {
    /// Eingabe/ Ausgabe Fehler
    IoError(io::Error),
    /// Libmodbus Fehler
    LibModbusError(LibModbusError),
    /// Libmodbus Rtu Fehler
    LibModbusRtuError(LibModbusRtuError),
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
            ModbusMasterError::IoError(ref error) => write!(f, "Io Fehler: {:?}", error),
            ModbusMasterError::LibModbusError(ref error) => write!(f, "Libmodbus Fehler: {:?}", error),
            ModbusMasterError::LibModbusRtuError(ref error) => write!(f, "Libmodbus RTU Fehler: {:?}", error),
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

impl From<LibModbusRtuError> for ModbusMasterError {
    fn from(error: LibModbusRtuError) -> Self {
        ModbusMasterError::LibModbusRtuError(error)
    }
}

impl From<LibModbusError> for ModbusMasterError {
    fn from(error: LibModbusError) -> Self {
        ModbusMasterError::LibModbusError(error)
    }
}

impl std::error::Error for ModbusMasterError {}
