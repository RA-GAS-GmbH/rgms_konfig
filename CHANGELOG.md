# Changelog

Alle erwähnenswert Änderungen am Projekt werden in dieser Datei dokumentiert.

Das Format der Datei basiert auf [Führe ein CHANGELOG](https://keepachangelog.com/de/1.0.0/),
außerdem befolgt dieses Projekt die [Semantische Versionierung](https://semver.org/lang/de/spec/v2.0.0.html)

## [Unveröffentlicht]

### Geändert

- Fehler behoben
  - Sensor-MB-NAP5x_REV1_0 konnte nicht ausgewählt werden

## [v1.0.1] - 2020-11-30

### Geändert

- Fehler beim speichern der MCS Konfiguration behoben
  - betraf Sensoren vom Typ 'Sensor-MB-NE4-V1.0'
- Update Lizenz in Hilfe.pdf und Hilfe-ra-gas.pdf


## [v1.0.0] - 2020-11-19

### Neu hinzugefügt

- Arbeitsweise wird nun in der Liveansicht aktualisiert
- Logik verbessert, so dass Fehlbedienungen verringert werden
  - ist zum Beispiel keine Platine gewählt können keine Aktionen gestartet werden
  - wenn keine Schnittstelle gewählt wurde können keine Aktionen gestartete werden
- Button 'Live Ansicht' farblich hervorgehoben
- Button 'Live Ansicht' Logik verbessert
  - bei nicht erfolgreichen Verbindungen wird der Button wieder deaktiviert
- Modbus Adresse wird beim Auslesen der Schreib.-/ Lese-Register aktualisiert
- Haken MCS Konfig wird automatisch gesetzt wenn die Platine entsprechend konfiguriert wurde
- PDF Dateien mit der Softwarebeschreibung (erreichbar über das Menü)
- Hilfe ist aus dem Menu aufrufbar

### Geändert

- Verknüpfungen die vom Installer unter Windows erstellt werden waren teilweise nicht gut benannt
- Arbeitsweisen waren nicht mit Software 27100 kompatibel
- Arbeitsweise nur in der RA-GAS internen Version änderbar
- Sensor 'Sensor-MB-CO2_O2_REV1_0'
  - ppm Werte CO2 werden nun *10 dargestellt. Siehe "27-10-2020_Beschreibung_RA-GAS Sensor-MB.pdf" Seite 14

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
