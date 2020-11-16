<div align="center">
  <h1>rgms_konfig</h1>
</div>

<div align="center">
 <strong>
    'RA-GAS GmbH Modbus System' - Konfigurator
 </strong>
</div>

<br />

<div align="center">
   <!-- GitLab CI status -->
  <a href="https://gitlab.com/RA-GAS-GmbH/rgms_konfig/pipelines">
    <img src="https://gitlab.com/RA-GAS-GmbH/rgms_konfig/badges/master/pipeline.svg"
      alt="GitLab CI status" />
  </a>
  <!-- Appveyor CI status -->
  <a href="https://ci.appveyor.com/project/zzeroo/rgms-konfig-uhy10">
    <img src="https://ci.appveyor.com/api/projects/status/cwu9pnq1ma1rqgo5?svg=true"
    alt="Appveyor CI status" />
  </a>
  <!-- Travis-ci.com CI status -->
  <a href="https://travis-ci.com/RA-GAS-GmbH/rgms_konfig">
    <img src="https://travis-ci.com/RA-GAS-GmbH/rgms_konfig.svg?branch=master"
    alt="Travis-ci.com CI status" />
  </a>
</div>

<div align="center">
  <h3>
    <a href="https://docs.rs/rgms_konfig">
      API Docs
    </a>
    <span> | </span>
    <a href="https://gitlab.com/RA-GAS-GmbH/rgms_konfig/-/releases">
      Releases
    </a>
    <span> | </span>
    <a href="https://gitlab.com/RA-GAS-GmbH/rgms_konfig/-/issues">
      Contributing
    </a>
  </h3>
</div>

<br/>

<div align="center">
  Konfigurator für Sensoren der 'RA-GAS GmbH Modbus System' Serie
</div>

<br/>

<div align="center" >
  <img src="resources/about.png" alt="About" />
</div>

# unterstützte Hardware

| Bordbezeichnung         | Beschreibung                                   |unterstützte Software|
| ----------------------- | ---------------------------------------------- | :---: |
| Sensor-MB-NE4-V1.0      | Erste Sensorplatine für Messzellen vom Typ NE4 | 25050 |
| Sensor-MB-NE4_REV1_0    | Platine für NE4 Messzellen                     | 27100 |
| Sensor-MB-NAP5xx_REV1_0 | Kombisensor für NAP5xx Messzellen              | 27100 |
| Sensor-MB-NAP5X_REV1_0  | Platine für NAP5x Messzellen                   | 27100 |
| Sensor-MB-CO2_O2_REV1_0 | Kombisensor Platine für CO2 und O2 Messzellen  | 27100 |
| Sensor-MB-SP42A_REV1_0  | Platine für SP42 Messzellen                    | 27100 |

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

# Verwendete Software
## Rust

Die minimale Rust Version ist 1.43.

## Gtk3

Es sollte die libgtk3 Version 3.22 verwendet werden.


[Gitlab CI]: https://gitlab.com/RA-GAS-GmbH/rgms_konfig/pipelines
[Appveyor CI]: https://ci.appveyor.com/project/zzeroo/rgms-konfig
[Compiling Rust + Windows + GTK step-by-step]: https://www.reddit.com/r/rust/comments/86kmhu/compiling_rust_windows_gtk_stepbystep/
[Releases]: https://gitlab.com/RA-GAS-GmbH/rgms_konfig/-/releases
