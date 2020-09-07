//!
//! # CSV Dateien erzeugen
//! Mit `pandoc` wird das .docx Dokument in eine HTML5 Datei konvertiert.
//!
//! ```bash
//! pandoc -f docx -t html5 28-08-2020_Beschreibung_RA-GAS\ Sensor-MB.docx -o 28-08-2020_Beschreibung_RA-GAS\ Sensor-MB.html
//! gio open 28-08-2020_Beschreibung_RA-GAS\ Sensor-MB.html
//! ```
//!
//! Nun werden nacheinander die Tabellen mit den `Rreg` und `Rwreg` markiert (siehe Screencast)
//! und in eine Tabellenkalkulationssoftware eingefügt.
//!
//! ```bash
//! 28-08-2020_Beschreibung_RA-GAS\ Sensor-MB.ods
//! ```
//!

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
pub use sensor_mb_ne4_legacy::SensorMbNe4Legacy;
