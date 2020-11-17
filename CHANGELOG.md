# Changelog

Alle erwähnenswert Änderungen am Projekt werden in dieser Datei dokumentiert.

Das Format der Datei basiert auf [Führe ein CHANGELOG](https://keepachangelog.com/de/1.0.0/),
außerdem befolgt dieses Projekt die [Semantische Versionierung](https://semver.org/lang/de/spec/v2.0.0.html)

## [Unveröffentlicht]

### Neu hinzugefügt

- Arbeitsweise wird nun in der Liveansicht aktualisiert
- Logik verbessert, so dass Fehlbedienungen verringert werden
  - ist zum Beispiel keine Platine gewählt können keine Aktionen gestartet werden
- Button 'Live Ansicht' farblich hervorgehoben
- Button 'Live Ansicht' Logik verbessert
  - bei nicht erfolgreichen Verbindungen wird der Button wieder deaktiviert
- Modbus Adresse wird beim Auslesen der Schreib.-/ Lese-Register aktualisiert

### Geändert

- Arbeitsweisen waren nicht mit Software 27100 kompatibel
- Arbeitsweise nur in der RA-GAS internen Version änderbar

## [v0.9.7] - 2020-11-11

### Neu hinzugefügt

- Einzelne Register Schreib.-/ Lese-Register können geschrieben werden

## [v0.9.0] - 2020-11-05

### Neu hinzugefügt

- Die Hardwarebeschreibung der Platinen wird aus CSV Dateien generiert
- dynamische Anzeige die auf die ausgewählte Platine reagiert
- verschiedene Logik in die GUI Elemente implementiert
  - so kann wenn die MCS Konfiguration gewählt ist, keine Modbus Adresse <129 gesetzt werden

### Geändert

[v0.1.0]: https://gitlab.com/RA-GAS-GmbH/rgms_konfig/-/tags/v0.1.0
