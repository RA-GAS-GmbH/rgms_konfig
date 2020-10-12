use std::{fmt, io::Error};

#[derive(Debug)]
pub enum ContextError {
    /// Eingabe/ Ausgabe Fehler
    IoError(Error),
}

impl fmt::Display for ContextError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            // ContextError::InitFailure => write!(f, "ModbusRtuMaster could not initalized"),
            ContextError::IoError(ref _error) => write!(f, "Io Error"),
        }
    }
}

impl From<Error> for ContextError {
    fn from(error: Error) -> Self {
        ContextError::IoError(error)
    }
}

impl std::error::Error for ContextError {}
