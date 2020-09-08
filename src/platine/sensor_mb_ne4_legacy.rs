/// Sensor-MB-NE4_REV1_0
///
/// Sensorplatine der Firma 'RA-GAS GmbH Kernen'
use crate::registers::{vec_from_csv, Rreg, Rwreg};

const CSV_RREG: &str = "resources/sensor_mb_ne4_legacy-rregs.csv";
const CSV_RWREG: &str = "resources/sensor_mb_ne4_legacy-rwregs.csv";

/// Sensor-MB-NE4_REV1_0
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
    /// assert_eq!(sensor.rregs.len(), 16);
    /// assert_eq!(sensor.rwregs.len(), 49);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_from_csv() {
        let sensor = SensorMbNe4Legacy::new_from_csv();
        assert!(sensor.is_ok());
        let sensor = sensor.unwrap();
        assert_eq!(sensor.rregs.len(), 16);
        assert_eq!(sensor.rwregs.len(), 49);
    }
}
