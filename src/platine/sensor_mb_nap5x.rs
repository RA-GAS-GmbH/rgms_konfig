/// Sensor-MB-NAP5X_REV1_0
///
/// Sensorplatine der Firma 'RA-GAS GmbH Kernen'
use crate::registers::{vec_from_csv, Rreg, Rwreg};

const CSV_RREG: &str = "resources/sensor_mb_nap5x-rregs.csv";
const CSV_RWREG: &str = "resources/sensor_mb_nap5x-rwregs.csv";

/// Sensor-MB-NAP5X_REV1_0
pub struct SensorMbNap5x {
    /// Lese Register
    pub rregs: Vec<Rreg>,
    /// Schreib/ Lese Register
    pub rwregs: Vec<Rwreg>,
}

impl SensorMbNap5x {
    /// Erstellt den Sensor aus den CSV Dateien
    ///
    /// # Examples
    /// ```rust
    /// use rgms_konfig::platine::{SensorMbNap5x};
    ///
    /// let sensor = SensorMbNap5x::new_from_csv();
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

        Ok(SensorMbNap5x {
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
        let sensor = SensorMbNap5x::new_from_csv();
        assert!(sensor.is_ok());
        let sensor = sensor.unwrap();
        assert_eq!(sensor.rregs.len(), 16);
        assert_eq!(sensor.rwregs.len(), 44);
    }
}
