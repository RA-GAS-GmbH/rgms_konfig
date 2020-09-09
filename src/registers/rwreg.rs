use serde::Deserialize;

/// Schreib/ Lese Register
#[derive(Clone, Debug, Default, Deserialize)]
pub struct Rwreg {
    #[serde(rename = "Rwreg Nr.\n(Fcode: 0x03, 0x06)")]
    rwreg_nr: Option<u32>,
    #[serde(rename = "Wertebereich")]
    range: String,
    #[serde(rename = "Zugeordnete Größe\nund Einheit")]
    value: Option<String>,
    #[serde(rename = "Messwerteigenschaft")]
    description: String,
    protected: Option<String>,
}

impl Rwreg {
    /// Register Nummer als u32
    ///
    /// Diese Funktion wird bei der Erstellung des gtk::TreeStores verwendet.
    pub fn rwreg_nr(&self) -> u32 {
        match self.rwreg_nr {
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

    /// Description Nummer als Boolean
    ///
    /// Diese Funktion wird bei der Erstellung des gtk::TreeStores verwendet.
    pub fn protected(&self) -> bool {
        match self.protected {
            Some(_) => true,
            None => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rreg_nr() {
        let rwreg = Rwreg::default();
        assert_eq!(rwreg.rwreg_nr(), 0);
    }

    #[test]
    fn range() {
        let rwreg = Rwreg::default();
        assert_eq!(rwreg.range(), "".to_string());
    }

    #[test]
    fn value() {
        let rwreg = Rwreg::default();
        assert_eq!(rwreg.value(), "".to_string());
    }

    #[test]
    fn description() {
        let rwreg = Rwreg::default();
        assert_eq!(rwreg.description(), "".to_string());
    }
}
