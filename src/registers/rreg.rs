use crate::registers::Register;
use serde::Deserialize;
/// Lese Register
#[derive(Clone, Debug, Default, Deserialize)]
pub struct Rreg {
    #[serde(rename = "Rreg Nr. (Fcode 0x04)")]
    reg_nr: Option<u32>,
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
    pub fn reg_nr(&self) -> u32 {
        match self.reg_nr {
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

impl Register for Rreg {
    fn is_empty(&self) -> bool {
        self.reg_nr.is_none()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reg_nr() {
        let rreg = Rreg::default();
        assert_eq!(rreg.reg_nr(), 0);
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

    #[test]
    fn is_empty() {
        let rreg = Rreg::default();
        assert_eq!(rreg.is_empty(), true);
    }
}
