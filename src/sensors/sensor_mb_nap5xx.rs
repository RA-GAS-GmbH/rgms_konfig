/// Sensor-MB-NAP5XX_REV1_0
///
/// Sensorplatine der Firma 'RA-GAS GmbH Kernen'
use crate::registers::{vec_from_csv, Rreg, Rwreg};

const CSV_RREG: &str = "resources/sensor_mb_nap5xx-rregs.csv";
const CSV_RWREG: &str = "resources/sensor_mb_nap5xx-rwregs.csv";

/// Sensor-MB-NAP5XX_REV1_0
pub struct SensorMbNap5xx {
    /// Lese Register
    pub rregs: Vec<Rreg>,
    /// Schreib/ Lese Register
    pub rwregs: Vec<Rwreg>,
}

impl SensorMbNap5xx {
    /// Erstellt den Sensor aus den CSV Dateien
    ///
    /// # Examples
    /// ```rust
    /// use rgms_konfig::sensors::{SensorMbNap5xx};
    ///
    /// let sensor = SensorMbNap5xx::new_from_csv();
    /// assert!(sensor.is_ok());
    /// let sensor = sensor.unwrap();
    /// assert_eq!(sensor.rregs.len(), 26);
    /// assert_eq!(sensor.rwregs.len(), 62);
    /// ```
    pub fn new_from_csv() -> Result<Self, Box<dyn std::error::Error>> {
        let file_path = CSV_RREG;
        let rregs: Result<Vec<Rreg>, Box<dyn std::error::Error>> = vec_from_csv(&file_path);
        let file_path = CSV_RWREG;
        let rwregs: Result<Vec<Rwreg>, Box<dyn std::error::Error>> = vec_from_csv(&file_path);

        Ok(SensorMbNap5xx {
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
        let sensor = SensorMbNap5xx::new_from_csv();
        assert!(sensor.is_ok());
        let sensor = sensor.unwrap();
        assert_eq!(sensor.rregs.len(), 26);
        assert_eq!(sensor.rwregs.len(), 62);
    }
}
