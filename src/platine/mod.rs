//! # Platine
//! ## Alle unterstützten Platinen.
//!
//! Folgende Platinen werden im Moment von der Software unterstützt.
//!
//! | Bordbezeichnung         | Beschreibung                                   |unterstützte Software|
//! | ----------------------- | ---------------------------------------------- | :---: |
//! | Sensor-MB-NE4-V1.0      | Erste Sensorplatine für Messzellen vom Typ NE4 | 25050 |
//! | Sensor-MB-NE4_REV1_0    | Platine für NE4 Messzellen                     | 15100 |
//! | Sensor-MB-NAP5xx_REV1_0 | Kombisensor für NAP5xx Messzellen              | 15100 |
//! | Sensor-MB-NAP5X_REV1_0  | Platine für NAP5x Messzellen                   | 15100 |
//! | Sensor-MB-CO2_O2_REV1_0 | Kombisensor Platine für CO2 und O2 Messzellen  | 15100 |
//! | Sensor-MB-SP42A_REV1_0  | Platine für SP42 Messzellen                    | 15100 |
//!
//!
//! # CSV Dateien erzeugen
//! Die Tabellen mit den `Rreg` und `Rwreg` Tabellen markieren (siehe Screencast)
//! und in eine Tabellenkalkulationssoftware eingefügt. Anschließend werden die Tabellen als CSV
//! Dateien unter `ressources` gespeichert. **Original Dateinamen müssen erhalten bleiben!**
//! ```
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
pub const HW_VERSIONS: &'static [(i32, &'static str, &'static str)] = &[
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

// TODO: Finde eine bessere Darstellung
// TODO: Arbeitsweisen pro Platinen-Typ möglich?
/// Mögliche Arbeitsweisen (Softwarestand: 15100)
///
/// Tupple (id, name) wird in 'src/gui/gtk3/mod.rs' verwendet
pub const WORKING_MODES: &'static [(i32, &'static str)] = &[
    (0, "unkonfiguriert"),
    (10, "CO-Sensor (1000)"),
    (12, "CO-Sensor (300)"),
    (20, "NO-Sensor (250)"),
    (30, "NO2 (20)"),
    (40, "NH3 (1000)"),
    (42, "NH3 (100)"),
    (50, "CL2 (10)"),
    (60, "H2S (100)"),
    (150, "NAP-50"),
    (155, "NAP-55"),
    (166, "NAP-66"),
    (204, "für GAS R404a [2000]"),
    (204, "für GAS R404a [1000]"),
    (210, "für GAS R410a [2000]"),
    (234, "für GAS R134a [2000]"),
    (249, "für GAS R449a [1000]"),
    (257, "für GAS R507 [2000]"),
    (270, "für GAS R1234ze [1000]"),
    (280, "für GAS R1234yt [1000]"),
    (430, "NAP505 und NAP550"),
    (510, "nur O2-Sensor"),
    (520, "nur CO2-Sensor"),
    (530, "beide Sensoren (kein Stromausgang)"),
];
