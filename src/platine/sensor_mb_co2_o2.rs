/// Sensor-MB-CO2_O2_REV1_0
///
/// Sensorplatine der Firma 'RA-GAS GmbH Kernen'
use crate::{
    platine::{Platine, HW_VERSIONS},
    registers::{vec_from_csv, RegisterError, Rreg, Rwreg},
};

const CSV_RREG: &str = "resources/sensor_mb_co2_o2-rregs.csv";
const CSV_RWREG: &str = "resources/sensor_mb_co2_o2-rwregs.csv";
const REG_PROTECTION: u16 = 79;

/// Sensor-MB-CO2_O2_REV1_0
#[derive(Clone, Debug, Default)]
pub struct SensorMbCo2O2 {
    /// Lese Register
    pub rregs: Vec<Rreg>,
    /// Schreib/ Lese Register
    pub rwregs: Vec<Rwreg>,
}

impl SensorMbCo2O2 {
    /// Erstellt ein "leere" Instanz des Sensors
    ///
    /// Diese wird nur in den Tests verwendete.
    ///
    /// # Examples
    /// ```rust
    /// use rgms_konfig::platine::{SensorMbCo2O2};
    ///
    /// let platine = SensorMbCo2O2::new();
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
    /// use rgms_konfig::platine::{SensorMbCo2O2};
    ///
    /// let sensor = SensorMbCo2O2::new_from_csv();
    /// assert!(sensor.is_ok());
    /// let sensor = sensor.unwrap();
    /// assert_eq!(sensor.rregs.len(), 16);
    /// assert_eq!(sensor.rwregs.len(), 41);
    /// ```
    pub fn new_from_csv() -> Result<Self, Box<dyn std::error::Error>> {
        let file_path = CSV_RREG;
        let rregs: Result<Vec<Rreg>, RegisterError> = vec_from_csv(&file_path);
        let file_path = CSV_RWREG;
        let rwregs: Result<Vec<Rwreg>, RegisterError> = vec_from_csv(&file_path);

        Ok(SensorMbCo2O2 {
            rregs: rregs?,
            rwregs: rwregs?,
        })
    }
}

impl Platine for SensorMbCo2O2 {
    fn name(&self) -> &str {
        let (_id, name, _desc) = HW_VERSIONS[4];
        name
    }

    fn description(&self) -> &str {
        let (_id, _name, desc) = HW_VERSIONS[4];
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

    #[test]
    fn name() {
        let platine = SensorMbCo2O2::new();
        assert_eq!(platine.name(), "Sensor-MB-CO2_O2_REV1_0");
    }

    #[test]
    fn description() {
        let platine = SensorMbCo2O2::new();
        assert_eq!(
            platine.description(),
            "Kombisensor Platine für CO2 und O2 Messzellen"
        );
    }

    #[test]
    fn new() {
        let platine = SensorMbCo2O2::new();
        assert_eq!(platine.rregs.len(), 0);
        assert_eq!(platine.rwregs.len(), 0);
    }

    #[test]
    fn test_new_from_csv_rregs() {
        let platine = SensorMbCo2O2::new_from_csv();
        assert!(platine.is_ok());
        let platine = platine.unwrap();
        assert_eq!(platine.rregs.len(), 16);
    }

    #[test]
    fn test_new_from_csv_rwregs() {
        let platine = SensorMbCo2O2::new_from_csv();
        assert!(platine.is_ok());
        let platine = platine.unwrap();
        assert_eq!(platine.rwregs.len(), 41);
    }

    #[test]
    fn reg_protection() {
        let platine = SensorMbCo2O2::new();
        assert_eq!(platine.reg_protection(), 79);
    }
}
