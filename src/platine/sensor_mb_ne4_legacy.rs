/// Sensor-MB-NE4_REV1_0
///
/// Sensorplatine der Firma 'RA-GAS GmbH Kernen'
use crate::{
    platine::Platine,
    registers::{vec_from_csv, RegisterError, Rreg, Rwreg},
};

const CSV_RREG: &str = "resources/sensor_mb_ne4_legacy-rregs.csv";
const CSV_RWREG: &str = "resources/sensor_mb_ne4_legacy-rwregs.csv";
const REG_PROTECTION: u16 = 49;

/// Sensor-MB-NE4_REV1_0
#[derive(Clone, Debug, Default)]
pub struct SensorMbNe4Legacy {
    /// Lese Register
    pub rregs: Vec<Rreg>,
    /// Schreib/ Lese Register
    pub rwregs: Vec<Rwreg>,
}

impl SensorMbNe4Legacy {
     /// Erstellt ein "leere" Instanz des Sensors
    ///
    /// Diese wird nur in den Tests verwendete.
    ///
    /// # Examples
    /// ```rust
    /// use rgms_konfig::platine::{SensorMbNe4Legacy};
    ///
    /// let platine = SensorMbNe4Legacy::new();
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
    /// use rgms_konfig::platine::{SensorMbNe4Legacy};
    ///
    /// let sensor = SensorMbNe4Legacy::new_from_csv();
    /// assert!(sensor.is_ok());
    /// let sensor = sensor.unwrap();
    /// assert_eq!(sensor.rregs.len(), 14);
    /// assert_eq!(sensor.rwregs.len(), 41);
    /// ```
    pub fn new_from_csv() -> Result<Self, Box<dyn std::error::Error>> {
        let file_path = CSV_RREG;
        let rregs: Result<Vec<Rreg>, RegisterError> = vec_from_csv(&file_path);
        let file_path = CSV_RWREG;
        let rwregs: Result<Vec<Rwreg>, RegisterError> = vec_from_csv(&file_path);

        Ok(SensorMbNe4Legacy {
            rregs: rregs?,
            rwregs: rwregs?,
        })
    }
}

impl Platine for SensorMbNe4Legacy {
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
    fn new() {
        let platine = SensorMbNe4Legacy::new();
        assert_eq!(platine.rregs.len(), 0);
        assert_eq!(platine.rwregs.len(), 0);
    }

    #[test]
    fn test_new_from_csv_rregs() {
        let platine = SensorMbNe4Legacy::new_from_csv();
        assert!(platine.is_ok());
        let platine = platine.unwrap();
        assert_eq!(platine.rregs.len(), 14);
    }

    #[test]
    fn test_new_from_csv_rwregs() {
        let platine = SensorMbNe4Legacy::new_from_csv();
        assert!(platine.is_ok());
        let platine = platine.unwrap();
        assert_eq!(platine.rwregs.len(), 41);
    }

    #[test]
    fn reg_protection() {
        let platine = SensorMbNe4Legacy::new();
        assert_eq!(platine.reg_protection(), 49);
    }
}
