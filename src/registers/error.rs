use std::error::Error;
use std::fmt;


/// Register Error
#[derive(Debug)]
pub struct RegisterError(String);

impl fmt::Display for RegisterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Register Error: {}", self.0)
    }
}

impl Error for RegisterError {}
