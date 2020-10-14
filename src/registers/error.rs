use std::error::Error;
use std::{fmt, io};

/// Register Error
#[derive(Debug)]
pub enum RegisterError {
    /// Fehler beim Import einer CSV Datei
    CsvError(csv::Error),
    /// IO Fehler
    IoError(io::Error),
}

impl fmt::Display for RegisterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RegisterError::CsvError(ref e) => write!(f, "CSV Error: {}", e),
            RegisterError::IoError(ref e) => write!(f, "IO Error: {}", e),
        }
    }
}

impl From<csv::Error> for RegisterError {
    fn from(error: csv::Error) -> Self {
        RegisterError::CsvError(error)
    }
}

impl From<io::Error> for RegisterError {
    fn from(error: io::Error) -> Self {
        RegisterError::IoError(error)
    }
}

impl Error for RegisterError {}
