<h1 align="center">RGMS</h1>
<div align="center">
 <strong>
   RA-GAS Modbus System
 </strong>
</div>

<br />

[![](https://ci.appveyor.com/api/projects/status/sqhnkrgqba67o4m4/branch/master?svg=true)]()


<div align="center">
   <!-- GitLab CI status -->
  <a href="https://gitlab.com/RA-GAS-GmbH/rgms_konfig/pipelines">
    <img src="https://gitlab.com/RA-GAS-GmbH/rgms_konfig/badges/master/pipeline.svg"
      alt="GitLab CI status" />
  </a>
  <!-- Appveyor CI status -->
  <a href="https://ci.appveyor.com/project/zzeroo/rgms-konfig/branch/master">
    <img src="https://ci.appveyor.com/api/projects/status/sqhnkrgqba67o4m4/branch/master?svg=true"
    alt="Appveyor CI status" />
  </a>
</div>

<div align="center">
  <h3>
    <a href="https://docs.rs/rgms_konfig">
      API Docs
    </a>
    <span> | </span>
    <a href="https://github.com/RA-GAS-GmbH/rgms_konfig/releases">
      Releases
    </a>
    <span> | </span>
    <a href="https://rgms_konfig.zzeroo.com/contribute">
      Contributing
    </a>
  </h3>
</div>

<br/>

GUI für die Konfiguration der Sensoren vom Typ 'RA-GAS Modbus System'

<div align="center">
    <img src="resources/about.png"
      alt="About" />
</div>

# unterstützte Hardware

| Bordbezeichnung         | Beschreibung                                   |unterstützte Software|
| ----------------------- | ---------------------------------------------- | :---: |
| Sensor-MB-NE4-V1.0      | Erste Sensorplatine für Messzellen vom Typ NE4 | 25050 |
| Sensor-MB-NE4_REV1_0    | Platine für NE4 Messzellen                     | 11090 |
| Sensor-MB-NAP5xx_REV1_0 | Kombisensor für NAP5xx Messzellen              | 11090 |
| Sensor-MB-NAP5X_REV1_0  | Platine für NAP5x Messzellen                   | 11090 |
| Sensor-MB-CO2_O2_REV1_0 | Kombisensor Platine für CO2 und O2 Messzellen  | 11090 |
| Sensor-MB-SP42A_REV1_0  | Platine für SP42 Messzellen                    | 11090 |

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
