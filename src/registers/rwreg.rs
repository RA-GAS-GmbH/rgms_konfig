use crate::registers::Register;
use serde::Deserialize;
/// Schreib/ Lese Register
#[derive(Clone, Debug, Default, Deserialize)]
pub struct Rwreg {
    #[serde(rename = "Rwreg Nr.
(Fcode: 0x03, 0x06)")]
    reg_nr: Option<u32>,
    #[serde(rename = "Wertebereich")]
    range: String,
    #[serde(rename = "Zugeordnete Größe und Einheit")]
    value: Option<String>,
    #[serde(rename = "Messwerteigenschaft")]
    description: String,
}

impl Rwreg {
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

    /// Ist dieses Register schreibgeschützt?
    ///
    /// Diese Funktion wird u.a. bei der Erstellung des gtk::TreeStores verwendet.
    pub fn is_protected(&self) -> bool {
        self.description.contains('*')
    }
}

impl Register for Rwreg {
    fn is_empty(&self) -> bool {
        self.reg_nr.is_none()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rreg_nr() {
        let rwreg = Rwreg::default();
        assert_eq!(rwreg.reg_nr(), 0);
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

    #[test]
    fn is_protected() {
        let rwreg = Rwreg {
            description: "Some description".to_string(),
            ..Default::default()
        };
        assert_eq!(rwreg.is_protected(), false);

        let rwreg = Rwreg {
            description: "Some protected description *".to_string(),
            ..Default::default()
        };
        assert_eq!(rwreg.is_protected(), true);
    }
}
