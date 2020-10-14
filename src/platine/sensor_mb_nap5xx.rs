/// Sensor-MB-NAP5XX_REV1_0
///
/// Sensorplatine der Firma 'RA-GAS GmbH Kernen'
use crate::{
    platine::Platine,
    registers::{vec_from_csv, RegisterError, Rreg, Rwreg},
};

const CSV_RREG: &str = "resources/sensor_mb_nap5xx-rregs.csv";
const CSV_RWREG: &str = "resources/sensor_mb_nap5xx-rwregs.csv";
const REG_PROTECTION: u16 = 79;

/// Sensor-MB-NAP5XX_REV1_0
#[derive(Clone, Debug, Default)]
pub struct SensorMbNap5xx {
    /// Lese Register
    pub rregs: Vec<Rreg>,
    /// Schreib/ Lese Register
    pub rwregs: Vec<Rwreg>,
}

impl SensorMbNap5xx {
    /// Erstellt ein "leere" Instanz des Sensors
    ///
    /// Diese wird nur in den Tests verwendete.
    ///
    /// # Examples
    /// ```rust
     /// use rgms_konfig::platine::{SensorMbNap5xx};
    ///
   /// let platine = SensorMbNap5xx::new();
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
    /// use rgms_konfig::platine::{SensorMbNap5xx};
    ///
    /// let sensor = SensorMbNap5xx::new_from_csv();
    /// assert!(sensor.is_ok());
    /// let sensor = sensor.unwrap();
    /// assert_eq!(sensor.rregs.len(), 23);
    /// assert_eq!(sensor.rwregs.len(), 49);
    /// ```
    pub fn new_from_csv() -> Result<Self, Box<dyn std::error::Error>> {
        let file_path = CSV_RREG;
        let rregs: Result<Vec<Rreg>, RegisterError> = vec_from_csv(&file_path);
        let file_path = CSV_RWREG;
        let rwregs: Result<Vec<Rwreg>, RegisterError> = vec_from_csv(&file_path);

        Ok(SensorMbNap5xx {
            rregs: rregs?,
            rwregs: rwregs?,
        })
    }
}

impl Platine for SensorMbNap5xx {
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
        let platine = SensorMbNap5xx::new();
        assert_eq!(platine.rregs.len(), 0);
        assert_eq!(platine.rwregs.len(), 0);
    }

    #[test]
    fn test_new_from_csv_rregs() {
        let platine = SensorMbNap5xx::new_from_csv();
        assert!(platine.is_ok());
        let platine = platine.unwrap();
        assert_eq!(platine.rregs.len(), 23);
    }

    #[test]
    fn test_new_from_csv_rwregs() {
        let platine = SensorMbNap5xx::new_from_csv();
        assert!(platine.is_ok());
        let platine = platine.unwrap();
        assert_eq!(platine.rwregs.len(), 49);
    }

    #[test]
    fn reg_protection() {
        let platine = SensorMbNap5xx::new();
        assert_eq!(platine.reg_protection(), 79);
    }
}
