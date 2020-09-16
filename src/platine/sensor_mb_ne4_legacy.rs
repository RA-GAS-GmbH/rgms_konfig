/// Sensor-MB-NE4_REV1_0
///
/// Sensorplatine der Firma 'RA-GAS GmbH Kernen'
use crate::{
    platine::Platine,
    registers::{vec_from_csv, Rreg, Rwreg},
};

const CSV_RREG: &str = "resources/sensor_mb_ne4_legacy-rregs.csv";
const CSV_RWREG: &str = "resources/sensor_mb_ne4_legacy-rwregs.csv";

/// Sensor-MB-NE4_REV1_0
#[derive(Clone, Debug)]
pub struct SensorMbNe4Legacy {
    /// Lese Register
    pub rregs: Vec<Rreg>,
    /// Schreib/ Lese Register
    pub rwregs: Vec<Rwreg>,
}

impl SensorMbNe4Legacy {
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
        let rregs: Result<Vec<Rreg>, Box<dyn std::error::Error>> = vec_from_csv(&file_path);
        let file_path = CSV_RWREG;
        let rwregs: Result<Vec<Rwreg>, Box<dyn std::error::Error>> = vec_from_csv(&file_path);

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
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
