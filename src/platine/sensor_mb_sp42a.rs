/// Sensor-MB-SP42A_REV1_0
///
/// Sensorplatine der Firma 'RA-GAS GmbH Kernen'
use crate::{
    platine::Platine,
    registers::{vec_from_csv, Rreg, Rwreg},
};

const CSV_RREG: &str = "resources/sensor_mb_sp42a-rregs.csv";
const CSV_RWREG: &str = "resources/sensor_mb_sp42a-rwregs.csv";

/// Sensor-MB-SP42A_REV1_0
#[derive(Clone, Debug)]
pub struct SensorMbSp42a {
    /// Lese Register
    pub rregs: Vec<Rreg>,
    /// Schreib/ Lese Register
    pub rwregs: Vec<Rwreg>,
}

impl SensorMbSp42a {
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
    /// assert_eq!(sensor.rwregs.len(), 34);
    /// ```
    pub fn new_from_csv() -> Result<Self, Box<dyn std::error::Error>> {
        let file_path = CSV_RREG;
        let rregs: Result<Vec<Rreg>, Box<dyn std::error::Error>> = vec_from_csv(&file_path);
        let file_path = CSV_RWREG;
        let rwregs: Result<Vec<Rwreg>, Box<dyn std::error::Error>> = vec_from_csv(&file_path);

        Ok(SensorMbSp42a {
            rregs: rregs?,
            rwregs: rwregs?,
        })
    }
}

impl Platine for SensorMbSp42a {
    fn rregs(&self) -> &[Rreg] {
        &self.rregs
    }

    fn rwregs(&self) -> &[Rwreg] {
        &self.rwregs
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_from_csv() {
        let platine = SensorMbSp42a::new_from_csv();
        assert!(platine.is_ok());
        let platine = platine.unwrap();
        assert_eq!(platine.rregs.len(), 14);
        assert_eq!(platine.rwregs.len(), 34);
    }
}
