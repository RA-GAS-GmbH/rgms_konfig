use serde::de::DeserializeOwned;
use std::fs::File;

mod rreg;
mod rwreg;

// Reexports
pub use rreg::Rreg;
pub use rwreg::Rwreg;

/// Generische Funktion um ein Vec von `Deserializable` Typen zu erstellen
///     
/// # Examples
/// Dieses Beispiel sucht eine CSV Datei unter /tmp!
/// Erstelle eine z.B. mit: `echo "field\n1337">/tmp/test.csv`
///
/// ```rust,no_run
/// use rgms_konfig::registers::vec_from_csv;
/// use serde::{de::DeserializeOwned, Deserialize};
///
/// #[derive(Deserialize)]
/// struct Foo {
///     field: usize,
/// }
///
/// fn main() {
///     let file_path = "/tmp/test.csv";
///     let res: Result<Vec<Foo>, Box<dyn std::error::Error>> = vec_from_csv(&file_path);
///     assert!(res.is_ok());
///     assert_eq!(res.unwrap().len(), 1)
/// }
pub fn vec_from_csv<'a, T>(file_path: &str) -> Result<Vec<T>, Box<dyn std::error::Error>>
where
    T: DeserializeOwned,
{
    let file = File::open(file_path).unwrap();
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader(file);
    let mut res: Vec<T> = vec![];
    for result in rdr.deserialize() {
        let record: T = result?;
        res.push(record);
    }
    Ok(res)
}

#[cfg(test)]
mod tests {
    const CSV_RREG: &str = "resources/sensor_mb_ne4-rregs.csv";
    const CSV_RWREG: &str = "resources/sensor_mb_ne4-rwregs.csv";

    use super::*;

    #[test]
    fn test_vec_from_csv_rreg() {
        let file_path = CSV_RREG;
        let res: Result<Vec<Rreg>, Box<dyn std::error::Error>> = vec_from_csv(&file_path);
        assert!(res.is_ok());
        assert_eq!(res.unwrap().len(), 16)
    }

    #[test]
    fn test_vec_from_csv_rwreg() {
        let file_path = CSV_RWREG;
        let res: Result<Vec<Rwreg>, Box<dyn std::error::Error>> = vec_from_csv(&file_path);
        assert!(res.is_ok());
        assert_eq!(res.unwrap().len(), 44)
    }
}
