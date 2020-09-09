use serde::Deserialize;
/// Lese Register
#[derive(Clone, Debug, Default, Deserialize)]
pub struct Rreg {
    #[serde(rename = "Rreg Nr.\n(Fcode 0x04)")]
    rreg_nr: Option<u32>,
    #[serde(rename = "Wertebereich")]
    range: String,
    #[serde(rename = "Zugeordnete Größe und teilw. Einheit")]
    value: Option<String>,
    #[serde(rename = "Messwerteigenschaft")]
    description: String,
}

impl Rreg {
    /// Register Nummer als u32
    ///
    /// Diese Funktion wird bei der Erstellung des gtk::TreeStores verwendet.
    pub fn rreg_nr(&self) -> u32 {
        match self.rreg_nr {
            Some(num) => num,
            None => 0,
        }
    }
    /// Range Nummer als String
    ///
    /// Diese Funktion wird bei der Erstellung des gtk::TreeStores verwendet.
    pub fn range(&self) -> String {
        self.range.to_string()
    }
    /// Value Nummer als String
    ///
    /// Diese Funktion wird bei der Erstellung des gtk::TreeStores verwendet.
    pub fn value(&self) -> String {
        "".to_string()
    }
    /// Description Nummer als String
    ///
    /// Diese Funktion wird bei der Erstellung des gtk::TreeStores verwendet.
    pub fn description(&self) -> String {
        self.description.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rreg_nr() {
        let rreg = Rreg::default();
        assert_eq!(rreg.rreg_nr(), 0);
    }

    #[test]
    fn range() {
        let rreg = Rreg::default();
        assert_eq!(rreg.range(), "".to_string());
    }

    #[test]
    fn values() {
        let rreg = Rreg::default();
        assert_eq!(rreg.value(), "".to_string());
    }

    #[test]
    fn description() {
        let rreg = Rreg::default();
        assert_eq!(rreg.description(), "".to_string());
    }
}
