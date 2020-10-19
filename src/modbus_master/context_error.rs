use std::{fmt, io::Error};

/// Mögliche Fehler die im Modbus RTU Context auftreten können
#[derive(Debug)]
pub enum ContextError {
    /// Eingabe/ Ausgabe Fehler
    IoError(Error),
}

impl fmt::Display for ContextError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
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
