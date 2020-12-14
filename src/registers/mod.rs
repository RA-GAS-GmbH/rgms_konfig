//! Es git 2 verschiedene Arten von Registern
//!
//! * Rregs   -> Lese Register
//! * Rwregs  -> Schreib/ Lese Register
//!
use serde::de::DeserializeOwned;
use std::fs::File;

mod error;
mod rreg;
mod rwreg;

// Reexports
pub use error::RegisterError;
pub use rreg::Rreg;
pub use rwreg::Rwreg;

/// Traits to handle Register Data while parsing CSV
pub trait Register {
    /// Erkenne leere Register
    ///
    /// Beim parsen der CSV Dateien können leere Register erstellt werden.
    /// Diese Funktion checkt ob die Register Nummer leer ist und liefert
    /// in diesem Fall `true` zurück.
    fn is_empty(&self) -> bool {
        false
    }
}

/// Mögliche Register Typen
pub const REGISTER_TYPES: &[(i32, &str)] = &[
    (0, "Rreg (Lese Register)"),
    (1, "Rwreg (Schreib/ Lese Register)"),
];

#[allow(clippy::needless_doctest_main)]
/// Generische Funktion um ein Vec von `Deserializable` Typen zu erstellen
///
/// # Examples
/// Dieses Beispiel sucht eine CSV Datei, mit Header (`field` im Beispiel),
/// unter /tmp!
///
/// Erstelle eine CSV Datei z.B. mit: `echo "field\n1337">/tmp/test.csv`
///
/// ```rust,no_run
/// use rgms_konfig::registers::*;
/// use serde::{de::DeserializeOwned, Deserialize};
///
/// #[derive(Debug, Deserialize)]
/// struct Foo {
///     field: usize,
/// }
/// // Die Struktur die geparsed werden soll muss das [`Register`] Trait
/// // implementieren.
/// impl Register for Foo {}
///
/// fn main() {
///     let file_path = "/tmp/test.csv";
///     let res: Result<Vec<Foo>, RegisterError> = vec_from_csv(&file_path);
///     assert!(res.is_ok());
///     assert_eq!(res.unwrap().len(), 1)
/// }
pub fn vec_from_csv<T>(file_path: &str) -> Result<Vec<T>, RegisterError>
where
    T: DeserializeOwned + Register + std::fmt::Debug,
{
    let file_path = std::path::Path::new(file_path);
    let file = File::open(file_path)?;
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader(file);
    let mut res: Vec<T> = vec![];
    for result in rdr.deserialize() {
        let record: T = result?;
        if !record.is_empty() {
            res.push(record);
        };
    }
    Ok(res)
}

#[cfg(test)]
mod tests {
    const CSV_RREG: &str = "resources/Sensor-MB-NE4_REV1_0-Rreg.csv";
    const CSV_RWREG: &str = "resources/Sensor-MB-NE4_REV1_0-Rwreg.csv";

    use super::*;

    #[test]
    fn test_vec_from_csv_rreg() {
        let file_path = CSV_RREG;
        let res: Result<Vec<Rreg>, RegisterError> = vec_from_csv(&file_path);
        assert!(res.is_ok());
        assert_eq!(res.unwrap().len(), 14)
    }

    #[test]
    fn test_vec_from_csv_rwreg() {
        let file_path = CSV_RWREG;
        let res: Result<Vec<Rwreg>, RegisterError> = vec_from_csv(&file_path);
        assert!(res.is_ok());
        assert_eq!(res.unwrap().len(), 35)
    }
}
