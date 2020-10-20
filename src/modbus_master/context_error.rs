use std::{fmt, io};

/// Mögliche Fehler die im Modbus RTU Context auftreten können
#[derive(Debug)]
pub enum ContextError {
    /// Eingabe/ Ausgabe Fehler
    IoError(io::Error),
    /// Tokio Serial Fehler
    TokioSerialError(tokio_serial::Error),
}

impl fmt::Display for ContextError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            ContextError::IoError(ref error) => write!(f, "Io Error: {}", error),
            ContextError::TokioSerialError(ref error) => write!(f, "Tokio Serial Error: {}", error),
        }
    }
}

impl From<io::Error> for ContextError {
    fn from(error: io::Error) -> Self {
        ContextError::IoError(error)
    }
}

impl From<tokio_serial::Error> for ContextError {
    fn from(error: tokio_serial::Error) -> Self {
        ContextError::TokioSerialError(error)
    }
}



impl std::error::Error for ContextError {}
