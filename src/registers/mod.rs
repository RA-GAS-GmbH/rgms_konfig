use serde::{de::DeserializeOwned, Deserialize};
use std::fs::File;

/// Lese Register
#[derive(Debug, Default, Deserialize)]
pub struct Rreg {
    #[serde(rename = "Rreg Nr.\n(Fcode 0x04)")]
    rreg_nr: Option<usize>,
    #[serde(rename = "Wertebereich")]
    range: String,
    #[serde(rename = "Zugeordnete Größe und teilw. Einheit")]
    values: Option<String>,
    #[serde(rename = "Messwerteigenschaft")]
    description: String,
}

/// Schreib/ Lese Register
#[derive(Debug, Default, Deserialize)]
pub struct Rwreg {
    #[serde(rename = "Rwreg Nr.\n(Fcode: 0x03, 0x06)")]
    rweg_nr: Option<usize>,
    #[serde(rename = "Wertebereich")]
    range: String,
    #[serde(rename = "Zugeordnete Größe\nund Einheit")]
    values: Option<String>,
    #[serde(rename = "Messwerteigenschaft")]
    description: String,
    protected: String,
}

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
    const CSV_RREG: &str = "resources/sensor_mb_ne4_legacy-rregs.csv";
    const CSV_RWREG: &str = "resources/sensor_mb_ne4_legacy-rwregs.csv";

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
