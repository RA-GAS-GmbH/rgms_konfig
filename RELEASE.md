# Dokumentation für den Release Prozess.

- [ ] `rustup update` Rust toolchains aktuell?
- [ ] `rustup default stable` Rust 'stable' toolchain ist default?
- [ ] `git checkout development` in den 'development' Branch wechseln
- [ ] evtl. alle lokalen Branches in development Zweig mergen
- [ ] stelle sicher das lokal *alle* Tests fehlerfrei durchlaufen werden
  - [ ] `cargo test` Cargo Tests fehlerfrei?
- [ ] Rust Code nach den Richtlinien des Rust Projekts formatieren
  - [ ] `cargo fmt`
- [ ] Test mit der 'nightly' Rust Version
  - [ ] `cargo +nightly test` Build und Test unter nighly ok?
- [ ] README.md korrekte Version der Sensor Software?
- [ ] Versionsnummer in 'Cargo.toml' erhöht?
- [ ] Changelog aktuell? Wurde die Datei 'CHANGELOG.md' mit allen wichtigen Änderungen am System gefüllt?
  - [ ] Update der nächsten Version Nummer im 'CHANGELOG.md' <https://keepachangelog.com/en/1.0.0>
  - [ ] aktuelles Tagesdatum neben der Version im 'CHANGELOG.md' stehen
- eventuell müssen nun die geänderten Dateien in die Versionskontrolle
  aufgenommen werden `git commit -a -m "Finaler Commit vor Release"`
- [ ] `git checkout release` in den 'release' Branch wechseln
- [ ] `git merge --no-ff development` merge den lokalen 'development' Branch
- [ ] `git tag vN.N.N` Version getagged?
- [ ] `git push --tags` Tags veröffentlicht?
- [ ] optional `git push --tags github` Tags auf Github veröffentlicht?
- [ ] `git push github` Branch ins github backup Repo pushen
- [ ] `git push origin` Branch ins gitlab Repo pushen
- CI überprüft?
  - [ ] <https://gitlab.com/RA-GAS-GmbH/rgms_konfig/pipelines> Ok?
- [ ] `git checkout master` wechsele in den *master* Branch
- [ ] `git merge --no-ff release` merge den lokalen 'release' Branch
- [ ] `git push github` finale Version auf Github veröffentlicht?
- [ ] `git push origin` finale Version auf Gitlab veröffentlicht?

## Release packen

## Windows Binaries (32 und 64Bit gemeinsam)
- [ ] `docker start -ai rgms_konfig-build > build.log 2> build.error.log` Windows Binaries gebilded
- [ ] `less build.log` und `less build.error.log` überprüft? Keine Fehler vorhanden
- Cleanup
  - [ ] `rm rgms_konfig-* -rf`
  - [ ] `rm build.log build.error.log`
  - [ ] `git checkout development && git rebase -i release` Development Branch auf den neusten Stand bringen
