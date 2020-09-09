use serde::Deserialize;
/// Lese Register
#[derive(Debug, Default, Deserialize)]
pub struct Rreg {
    #[serde(rename = "Rreg Nr.\n(Fcode 0x04)")]
    rreg_nr: Option<usize>,
    #[serde(rename = "Wertebereich")]
    range: String,
    #[serde(rename = "Zugeordnete Größe und teilw. Einheit")]
    value: Option<String>,
    #[serde(rename = "Messwerteigenschaft")]
    description: String,
}

impl Rreg {
    /// Register Nummer als String
    ///
    /// Diese Funktion wird bei der Erstellung des gtk::TreeStores verwendet.
    pub fn rreg_nr(&self) -> String {
        match self.rreg_nr {
            Some(reg_nr) => reg_nr.to_string(),
            None => "".to_string(),
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
        assert_eq!(rreg.rreg_nr(), "".to_string());
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
