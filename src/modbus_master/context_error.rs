use std::{fmt, io::Error};

#[derive(Debug)]
pub enum ContextError {
    /// Fehler beim lesen der Lese Register
    ReadRRegs { source: std::io::Error },
    // ReadRwRegs { source: std::io::Error },
    // InitFailure,
    /// Eingabe/ Ausgabe Fehler
    IoError(Error),
    // NoSharedContext,
}

impl fmt::Display for ContextError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            ContextError::ReadRRegs { source: _ } => write!(f, "Could not read Read Register"),
            // ContextError::ReadRwRegs { ref source } => write!(f, "Could not read Read/ Write Register"),
            // ContextError::InitFailure => write!(f, "ModbusRtuMaster could not initalized"),
            ContextError::IoError(ref _error) => write!(f, "Io Error"),
            // ContextError::NoSharedContext => write!(f, "Could not create shared context."),
        }
    }
}

impl From<Error> for ContextError {
    fn from(error: Error) -> Self {
        ContextError::IoError(error)
    }
}

impl std::error::Error for ContextError {}
