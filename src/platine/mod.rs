//! # Platine
//! ## Alle unterstützten Platinen.
//!
//! Folgende Platinen werden im Moment von der Software unterstützt.
//!
//! | Bordbezeichnung         | Beschreibung                                   |unterstützte Software|
//! | ----------------------- | ---------------------------------------------- | :---: |
//! | Sensor-MB-NE4-V1.0      | Erste Sensorplatine für Messzellen vom Typ NE4 | 25050 |
//! | Sensor-MB-NE4_REV1_0    | Platine für NE4 Messzellen                     | 02120 |
//! | Sensor-MB-NAP5xx_REV1_0 | Kombisensor für NAP5xx Messzellen              | 02120 |
//! | Sensor-MB-NAP5X_REV1_0  | Platine für NAP5x Messzellen                   | 02120 |
//! | Sensor-MB-CO2_O2_REV1_0 | Kombisensor Platine für CO2 und O2 Messzellen  | 02120 |
//! | Sensor-MB-SP42A_REV1_0  | Platine für SP42 Messzellen                    | 02120 |

use crate::registers::{Rreg, Rwreg};
use core::fmt::Debug;
use std::sync::{Arc, Mutex};

/// Sensor-MB-CO2-O2_REV1_0
pub mod sensor_mb_co2_o2;

/// Sensor-MB-NAP5xx_REV1_0
pub mod sensor_mb_nap5xx;

/// Sensor-MB-NAP5xx_REV1_0
pub mod sensor_mb_nap5x;

/// Sensor-MB-NE4_V1_0
pub mod sensor_mb_ne4;

/// Sensor-MB-NE4_REV1_0
pub mod sensor_mb_ne4_legacy;

/// Sensor-MB-SP42a_REV1_0
pub mod sensor_mb_sp42a;

// Reexports
pub use sensor_mb_co2_o2::SensorMbCo2O2;
pub use sensor_mb_nap5x::SensorMbNap5x;
pub use sensor_mb_nap5xx::SensorMbNap5xx;
pub use sensor_mb_ne4::SensorMbNe4;
pub use sensor_mb_ne4_legacy::SensorMbNe4Legacy;
pub use sensor_mb_sp42a::SensorMbSp42a;

/// Resource counted, clonbare, optionale Platine
///
/// In diesem Typen wird die verwendete Hardware-Version gespeichert.
pub type BoxedPlatine = Arc<Mutex<Option<Box<dyn Platine>>>>;

/// Sensoren vom Typ 'RA-GAS Modbus System'
pub trait Platine {
    /// Platinebezeichnung
    fn name(&self) -> &str;

    /// Beschreibung der Platine
    fn description(&self) -> &str;

    /// Liefert ein Slice von Lese-Registern
    fn rregs(&self) -> &[Rreg];

    /// Liefert ein Slice von Schreib.-/ Lese-Registern
    fn rwregs(&self) -> &[Rwreg];

    /// Vector der Lese-Register
    fn vec_rregs(&self) -> Vec<Rreg> {
        self.rregs().to_vec()
    }

    /// Vector der Schreib.-/ Lese-Register
    fn vec_rwregs(&self) -> Vec<Rwreg> {
        self.rwregs().to_vec()
    }

    /// Schreibschutz Registernummer
    ///
    /// Liefert die Register Nummer mit dem die Platine entsperrt werden kann
    fn reg_protection(&self) -> u16;
}

impl Debug for dyn Platine {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.name())
    }
}
/// Unterstützte Platinen
///
/// Tupple (id, name, desc) wird in 'src/gui/gtk3/mod.rs' verwendet
pub const HW_VERSIONS: &[(i32, &str, &str)] = &[
    (
        0,
        "Sensor-MB-NE4-V1.0",
        "Erste Sensorplatine für Messzellen vom Typ NE4, bis Softwarestand: 25050",
    ),
    (1, "Sensor-MB-NE4_REV1_0", "Platine für NE4 Messzellen"),
    (
        2,
        "Sensor-MB-NAP5xx_REV1_0",
        "Kombisensor für NAP5xx Messzellen",
    ),
    (3, "Sensor-MB-NAP5x_REV1_0", "Platine für NAP5x Messzellen"),
    (
        4,
        "Sensor-MB-CO2_O2_REV1_0",
        "Kombisensor Platine für CO2 und O2 Messzellen",
    ),
    (5, "Sensor-MB-SP42A_REV1_0", "Platine für SP42 Messzellen"),
];

/// Standard Schreibschutz Register
pub const DEFAULT_REG_PROTECTION: u16 = 79;
