GUI für die Konfiguration der Sensoren vom Typ 'RA-GAS Modbus System'

[![GitLab CI status](https://gitlab.com/RA-GAS-GmbH/rgms_konfig/badges/master/pipeline.svg)](https://gitlab.com/RA-GAS-GmbH/rgms_konfig/pipelines)
[![Appveyor CI status](https://ci.appveyor.com/api/projects/status/sqhnkrgqba67o4m4/branch/master?svg=true)](https://ci.appveyor.com/project/zzeroo/rgms-konfig/branch/master)

![About](resources/about.png)

# unterstützte Hardware

| Bordbezeichnung         | Beschreibung                                                             |
| ----------------------- | ------------------------------------------------------------------------ |
| Sensor-MB-NE4-V1.0      | Erste Sensorplatine für Messzellen vom Typ NE4, bis Softwarestand: 25050 |
| Sensor-MB-NE4_REV1_0    | Platine für NE4 Messzellen                                               |
| Sensor-MB-NAP5xx_REV1_0 | Kombisensor für NAP5xx Messzellen                                        |
| Sensor-MB-NAP5X_REV1_0  | Platine für NAP5x Messzellen                                             |
| Sensor-MB-CO2_O2_REV1_0 | Kombisensor Platine für CO2 und O2 Messzellen                            |
| Sensor-MB-SP42A_REV1_0  | Platine für SP42 Messzellen                                              |

# Installation

## Installation - Linux

Siehe: [Releases]

## Installation - Windows

Siehe: [Releases]


# Qellcode selber übersetzen

Das Projekt nutzt den `stable` Zweig von Rust.
Die minimal kompatible Rust Version ist 1.43.0, die nightly Version von Rust
wird aber auch von der CI getestet und sollte ebenfalls funktionieren.

Neben Rust müssen auch die Gtk und Udev Entwicklungs Bibliotheken installiert
werden.

## unter Linux

Die Installation von Rust wird hier beschrieben: https://rustup.rs/

```bash
rustup default stable
```

Die Gtk und Udev Bibliotheken können unter anderem so installiert werden:

```bash
# debian/ ubuntu
apt install libudev-dev libgtk-3-dev
```

## unter Windows

Für Windows ist die Installation von Rust hier beschrieben: https://rustup.rs/

Wir verwenden unter Windows das Host Tripple `x86_64-pc-windows-gnu`,
die `stable` Toolchain und das `minimal` Rustup Profil.

```powershell
# Powershell
curl -sSf -o rustup-init.exe https://win.rustup.rs/
rustup-init.exe -y --default-host x86_64-pc-windows-gnu --default-toolchain stable
set PATH=%PATH%;%USERPROFILE%\.cargo\bin
rustc -Vv
cargo -V
```

Die Installation der Gtk Bibliotheken wird hier beschrieben: [Compiling Rust + Windows + GTK step-by-step]

# Entwicklung

Die Entwicklung wird mit dem `stable` Zweig von Rust durchgeführt.

Die Verwendung von `rustfmt` ist zwingend. Für die `cargo fmt` Durchläufe
verwenden wir ebenfalls den `stable` Zweig von Rust.

```bash
rustup component add rustfmt
```

Zudem sollten alle Pull Requests vorab mit `cargo clippy` geprüft werden.

```bash
rustup component add clippy
```


[Gitlab CI]: https://gitlab.com/RA-GAS-GmbH/rgms_konfig/pipelines
[Appveyor CI]: https://ci.appveyor.com/project/zzeroo/rgms-konfig
[Compiling Rust + Windows + GTK step-by-step]: https://www.reddit.com/r/rust/comments/86kmhu/compiling_rust_windows_gtk_stepbystep/
[Releases]: https://gitlab.com/RA-GAS-GmbH/rgms_konfig/-/releases
