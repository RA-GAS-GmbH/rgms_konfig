//! # Platine
//! ## Alle unterstützten Platinen.
//!
//! Folgende Platinen werden im Moment von der Software unterstützt.
//!
//! | Bordbezeichnung         | Beschreibung                                   |unterstützte Software|
//! | ----------------------- | ---------------------------------------------- | :---: |
//! | Sensor-MB-NE4-V1.0      | Erste Sensorplatine für Messzellen vom Typ NE4 | 25050 |
//! | Sensor-MB-NE4_REV1_0    | Platine für NE4 Messzellen                     | 11090 |
//! | Sensor-MB-NAP5xx_REV1_0 | Kombisensor für NAP5xx Messzellen              | 11090 |
//! | Sensor-MB-NAP5X_REV1_0  | Platine für NAP5x Messzellen                   | 11090 |
//! | Sensor-MB-CO2_O2_REV1_0 | Kombisensor Platine für CO2 und O2 Messzellen  | 11090 |
//! | Sensor-MB-SP42A_REV1_0  | Platine für SP42 Messzellen                    | 11090 |
//!
//!
//! # CSV Dateien erzeugen
//! Mit `pandoc` wird das .docx Dokument in eine HTML5 Datei konvertiert.
//!
//! ```bash
//! pandoc -f docx -t html5 "11-09-2020_Beschreibung_RA-GAS Sensor-MB.docx" -o "11-09-2020_Beschreibung_RA-GAS Sensor-MB.html"
//! gio open "11-09-2020_Beschreibung_RA-GAS Sensor-MB.html"
//! ```
//!
//! Nun werden nacheinander die Tabellen mit den `Rreg` und `Rwreg` markiert (siehe Screencast)
//! und in eine Tabellenkalkulationssoftware eingefügt.
//!
//! ```bash
//! gio open Beschreibung-Register.ods
//! ```
use crate::registers::{Rreg, Rwreg};
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

/// Resource Counted Optional Boxed Platine
///
/// ```compile_fail
/// Arc<Mutex<Option<Box<dyn Platine>>>>
/// ```
pub type BoxedPlatine = Arc<Mutex<Option<Box<dyn Platine>>>>;

/// Sensoren vom Typ 'RA-GAS Modbus System'
pub trait Platine {
    /// Liefert ein Slice von Lese Registern
    fn rregs(&self) -> &[Rreg];
    /// Liefert ein Slice von Schreib/ Lese Registern
    fn rwregs(&self) -> &[Rwreg];
    /// Vector of rregs
    fn vec_rregs(&self) -> Vec<Rreg> {
        self.rregs().to_vec()
    }
    /// Vector of rwregs
    fn vec_rwregs(&self) -> Vec<Rwreg> {
        self.rwregs().to_vec()
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

/// Mögliche Arbeitsweisen
///
/// Tupple (id, name) wird in 'src/gui/gtk3/mod.rs' verwendet
pub const WORKING_MODES: &'static [(i32, &'static str)] = &[
    (0, "unkonfiguriert"),
    (10, "CO-Sensor (1000)"),
    (12, "CO-Sensor (300)"),
    (150, "NAP-50"),
    (155, "NAP-55"),
    (166, "NAP-66"),
    (20, "NO-Sensor (250)"),
    (204, "R404a (2000)"),
    (205, "R404a (1000)"),
    (210, "R410a (2000)"),
    (234, "R134a (2000)"),
    (249, "R449a (1000)"),
    (257, "R507 (2000)"),
    (270, "R1234ze (1000)"),
    (280, "R1234yf (1000)"),
    (30, "NO2 (20)"),
    (40, "NH3 (1000)"),
    (42, "NH3 (100)"),
    (430, "NAP505 und NAP550"),
    (50, "CL2 (10)"),
    (510, "nur O2-Sensor"),
    (520, "nur CO2-Sensor"),
    (530, "beide Sensoren (kein Stromausgang)"),
    (60, "H2S (100)"),
];
