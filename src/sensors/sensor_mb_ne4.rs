/// Sensor-MB-NE4_V1_0
///
/// Sensorplatine der Firma 'RA-GAS GmbH Kernen'
use crate::registers::{vec_from_csv, Rreg, Rwreg};

const CSV_RREG: &str = "resources/sensor_mb_ne4-rregs.csv";
const CSV_RWREG: &str = "resources/sensor_mb_ne4-rwregs.csv";

/// Sensor-MB-NE4_V1_0
pub struct SensorMbNe4 {
    /// Lese Register
    pub rregs: Vec<Rreg>,
    /// Schreib/ Lese Register
    pub rwregs: Vec<Rwreg>,
}

impl SensorMbNe4 {
    /// Erstellt den Sensor aus den CSV Dateien
    ///
    /// # Examples
    /// ```rust
    /// use rgms_konfig::sensors::{SensorMbNe4};
    ///
    /// let sensor = SensorMbNe4::new_from_csv();
    /// assert!(sensor.is_ok());
    /// let sensor = sensor.unwrap();
    /// assert_eq!(sensor.rregs.len(), 16);
    /// assert_eq!(sensor.rwregs.len(), 44);
    /// ```
    pub fn new_from_csv() -> Result<Self, Box<dyn std::error::Error>> {
        let file_path = CSV_RREG;
        let rregs: Result<Vec<Rreg>, Box<dyn std::error::Error>> = vec_from_csv(&file_path);
        let file_path = CSV_RWREG;
        let rwregs: Result<Vec<Rwreg>, Box<dyn std::error::Error>> = vec_from_csv(&file_path);

        Ok(SensorMbNe4 {
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
        let sensor = SensorMbNe4::new_from_csv();
        assert!(sensor.is_ok());
        let sensor = sensor.unwrap();
        assert_eq!(sensor.rregs.len(), 16);
        assert_eq!(sensor.rwregs.len(), 44);
    }
}
