/// Sensor-MB-SP42A_REV1_0
///
/// Sensorplatine der Firma 'RA-GAS GmbH Kernen'
use crate::{
    platine::{Platine, HW_VERSIONS},
    registers::{vec_from_csv, RegisterError, Rreg, Rwreg},
};

#[cfg(target_os = "windows")]
const CSV_RREG: &str = ".\\resources\\Sensor-MB-SP42A_REV1_0-Rreg.csv";
#[cfg(target_os = "windows")]
const CSV_RWREG: &str = ".\\resources\\Sensor-MB-SP42A_REV1_0-Rwreg.csv";
#[cfg(target_os = "linux")]
const CSV_RREG: &str = "resources/Sensor-MB-SP42A_REV1_0-Rreg.csv";
#[cfg(target_os = "linux")]
const CSV_RWREG: &str = "resources/Sensor-MB-SP42A_REV1_0-Rwreg.csv";

const REG_PROTECTION: u16 = 79;

/// Sensor-MB-SP42A_REV1_0
#[derive(Clone, Debug, Default)]
pub struct SensorMbSp42a {
    /// Lese Register
    pub rregs: Vec<Rreg>,
    /// Schreib/ Lese Register
    pub rwregs: Vec<Rwreg>,
}

impl SensorMbSp42a {
    /// Erstellt ein "leere" Instanz des Sensors
    ///
    /// Diese wird nur in den Tests verwendete.
    ///
    /// # Examples
    /// ```rust
    /// use rgms_konfig::platine::{SensorMbSp42a};
    ///
    /// let platine = SensorMbSp42a::new();
    /// assert_eq!(platine.rregs.len(), 0);
    /// assert_eq!(platine.rwregs.len(), 0);
    /// ```
    pub fn new() -> Self {
        Default::default()
    }

    /// Erstellt den Sensor aus den CSV Dateien
    ///
    /// # Examples
    /// ```rust
    /// use rgms_konfig::platine::{SensorMbSp42a};
    ///
    /// let sensor = SensorMbSp42a::new_from_csv();
    /// assert!(sensor.is_ok());
    /// let sensor = sensor.unwrap();
    /// assert_eq!(sensor.rregs.len(), 14);
    /// assert_eq!(sensor.rwregs.len(), 51);
    /// ```
    pub fn new_from_csv() -> Result<Self, Box<dyn std::error::Error>> {
        let file_path = CSV_RREG;
        let rregs: Result<Vec<Rreg>, RegisterError> = vec_from_csv(&file_path);
        let file_path = CSV_RWREG;
        let rwregs: Result<Vec<Rwreg>, RegisterError> = vec_from_csv(&file_path);

        Ok(SensorMbSp42a {
            rregs: rregs?,
            rwregs: rwregs?,
        })
    }
}

impl Platine for SensorMbSp42a {
    fn name(&self) -> &str {
        let (_id, name, _desc) = HW_VERSIONS[5];
        name
    }

    fn description(&self) -> &str {
        let (_id, _name, desc) = HW_VERSIONS[5];
        desc
    }

    fn rregs(&self) -> &[Rreg] {
        &self.rregs
    }

    fn rwregs(&self) -> &[Rwreg] {
        &self.rwregs
    }

    fn reg_protection(&self) -> u16 {
        REG_PROTECTION
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn csv_files_exists() {
        assert!( Path::new(CSV_RREG).exists() );
        assert!( Path::new(CSV_RWREG).exists() );
    }

    #[test]
    fn name() {
        let platine = SensorMbSp42a::new();
        assert_eq!(platine.name(), "Sensor-MB-SP42A_REV1_0");
    }

    #[test]
    fn description() {
        let platine = SensorMbSp42a::new();
        assert_eq!(platine.description(), "Platine für SP42 Messzellen");
    }

    #[test]
    fn new() {
        let platine = SensorMbSp42a::new();
        assert_eq!(platine.rregs.len(), 0);
        assert_eq!(platine.rwregs.len(), 0);
    }

    #[test]
    fn test_new_from_csv_rregs() {
        let platine = SensorMbSp42a::new_from_csv();
        assert!(platine.is_ok());
        let platine = platine.unwrap();
        assert_eq!(platine.rregs.len(), 14);
    }

    #[test]
    fn test_new_from_csv_rwregs() {
        let platine = SensorMbSp42a::new_from_csv();
        assert!(platine.is_ok());
        let platine = platine.unwrap();
        assert_eq!(platine.rwregs.len(), 51);
    }

    #[test]
    fn reg_protection() {
        let platine = SensorMbSp42a::new();
        assert_eq!(platine.reg_protection(), 79);
    }
}
