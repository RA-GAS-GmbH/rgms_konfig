use std::error::Error;
use std::fmt;


/// Register Error
#[derive(Debug)]
pub enum RegisterError {
    /// Fehler beim Import einer CSV Datei
    CsvError(csv::Error),
}

impl fmt::Display for RegisterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RegisterError::CsvError(ref e) => write!(f, "CSV Error: {}", e),
        }
    }
}

impl From<csv::Error> for RegisterError {
    fn from(error: csv::Error) -> Self {
        RegisterError::CsvError(error)
    }
}

impl Error for RegisterError {}
