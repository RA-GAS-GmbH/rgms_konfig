/// ModbusMaster Fehler
pub mod error;

pub use error::ModbusMasterError;

use crate::{
    gui::gtk3::{GuiMessage, *},
    registers::{Rreg, Rwreg},
};
use futures::channel::mpsc::Sender;
use libmodbus::{Modbus, ModbusClient, ModbusRTU};
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use tokio::{runtime::Runtime, sync::mpsc};

const LOCK_TIMEOUT: u64 = 20;

/// Possible ModbusMaster commands
/// TODO: Nutze Struct Enum Types Connect { tty: String, rregs: Vec<Rreg>, rwregs: Vec<Rwregs>, ...}
#[derive(Debug)]
pub enum ModbusMasterMessage {
    /// Starte Control Loop
    Connect(String, u8, Vec<Rreg>, Vec<Rwreg>, u16),
    /// Stoppe Control Loop
    Disconnect,
    /// Nullgas
    Nullgas {
        /// serielle Schnittstelle
        tty_path: String,
        /// Modbus Slave ID
        slave: u8,
        /// Entsperr Register Nummer
        reg_protection: u16,
        /// Messzellen Nummer
        sensor_num: u16,
    },
    /// Messgas
    Messgas {
        /// serielle Schnittstelle
        tty_path: String,
        /// Modbus Slave ID
        slave: u8,
        /// Entsperr Register Nummer
        reg_protection: u16,
        /// Messzellen Nummer
        sensor_num: u16,
    },
    /// Speichert die MCS Bus Konfiguration
    SetNewMcsBusId {
        /// serielle Schnittstelle
        tty_path: String,
        /// Modbus Slave ID
        slave: u8,
        /// Neue Modbus Adresse
        new_slave_id: u16,
        /// Entsperr Register Nummer
        reg_protection: u16,
    },
    /// Speichert die Modbus Konfiguration
    SetNewModbusId {
        /// serielle Schnittstelle
        tty_path: String,
        /// Modbus Slave ID
        slave: u8,
        /// Neue Modbus Adresse
        new_slave_id: u16,
        /// Entsperr Register Nummer
        reg_protection: u16,
    },
    /// Setzt die Arbeitsweise
    // (String, u8, u16, u16),
    SetNewWorkingMode {
        /// serielle Schnittstelle
        tty_path: String,
        /// Modbus Slave ID
        slave: u8,
        /// Neue Modbus Adresse
        working_mode: u16,
        /// Entsperr Register Nummer
        reg_protection: u16,
    },
    /// Update one register
    UpdateRegister {
        /// serielle Schnittstelle
        tty_path: String,
        /// Modbus Slave ID
        slave: u8,
        /// Neue Modbus Adresse
        reg_nr: u16,
        /// Entsperr Register Nummer
        reg_protection: u16,
        /// neuer wert
        new_value: u16,
    },
}

/// Modbus Master
#[derive(Debug, Clone)]
pub struct ModbusMaster {
    /// Sender über den mit dem ModbusMaster kommuniziert werden kann
    pub tx: mpsc::Sender<ModbusMasterMessage>,
}

impl ModbusMaster {
    /// Erzeugt einen neuen Modbus Master
    pub fn new(gui_tx: Sender<GuiMessage>) -> ModbusMaster {
        // Komunikationskanäle
        let (tx, mut rx) = mpsc::channel(1);

        // Control Loop erzeugen
        // Diese Funktion liefert den Empfänger-Teil eines Channels zurück. Über
        // diesen kann mit dem Control Loop kommuniziert werden.
        let mut control_loop_tx = spawn_control_loop();

        std::thread::spawn(move || {
            let mut rt = Runtime::new().expect("Could not create Runtime");

            rt.block_on(async {
                // Control variable die den Control Loop steuert
                let is_online = Arc::new(Mutex::new(false));

                while let Some(command) = rx.recv().await {
                    match command {
                        // Startet dem Control Loop
                        ModbusMasterMessage::Connect(
                            tty_path,
                            slave,
                            rregs,
                            rwregs,
                            reg_protection,
                        ) => {
                            info!("ModbusMasterMessage::Connect");
                            // debug!("tty_path: {}, slave: {}, rregs: {:?}, rwregs: {:?}", tty_path, slave, rregs, rwregs);

                            let mut state = is_online.lock().unwrap();
                            *state = true;

                            // Sende Start Commando an Control Loop
                            match control_loop_tx.try_send(MsgControlLoop::Start(
                                is_online.clone(),
                                tty_path,
                                slave,
                                rregs,
                                rwregs,
                                reg_protection,
                                gui_tx.clone(),
                            )) {
                                Ok(_) => {
                                    // show_info(&gui_tx, "Live Ansicht gestartet");
                                }
                                Err(error) => show_warning(
                                    &gui_tx,
                                    &format!(
                                        "Control Loop konnte nicht erreicht werden: {}",
                                        error
                                    ),
                                ),
                            }
                        }
                        ModbusMasterMessage::Disconnect => {
                            info!("ModbusMasterMessage::Disconnect");
                            let mut state = is_online.lock().unwrap();
                            *state = false;
                        }
                        // Nullgas setzen
                        ModbusMasterMessage::Nullgas {
                            tty_path,
                            slave,
                            reg_protection,
                            sensor_num,
                        } => match set_nullgas(tty_path, slave, reg_protection, sensor_num) {
                            Ok(_) => {
                                show_info(&gui_tx, "Nullpunkt erfolgreich gesetzt");
                            }
                            Err(error) => show_error(
                                &gui_tx,
                                &format!("Nullgas konnte nicht gesetzt werden: {}", error),
                            ),
                        },
                        // Messgas setzen
                        ModbusMasterMessage::Messgas {
                            tty_path,
                            slave,
                            reg_protection,
                            sensor_num,
                        } => match set_messgas(tty_path, slave, reg_protection, sensor_num) {
                            Ok(_) => {
                                show_info(&gui_tx, "Endwert Messgas erfolgreich gesetzt");
                            }
                            Err(error) => show_error(
                                &gui_tx,
                                &format!("Messgas konnte nicht gesetzt werden: {}", error),
                            ),
                        },
                        // Neue MCS Bus ID setzen
                        ModbusMasterMessage::SetNewMcsBusId {
                            tty_path,
                            slave,
                            new_slave_id,
                            reg_protection,
                        } => {
                            match set_new_mcs_bus_id(tty_path, slave, new_slave_id, reg_protection)
                            {
                                Ok(_) => {
                                    show_info(&gui_tx, "MCS Adresse erfolgreich gesetzt");
                                }
                                Err(error) => show_warning(
                                    &gui_tx,
                                    &format!(
                                        "Konnte MCS Adresse '{}' nicht speichern:\r\n{}",
                                        &new_slave_id, error
                                    ),
                                ),
                            }
                        }
                        // Neue Modbus Slave ID setzen
                        ModbusMasterMessage::SetNewModbusId {
                            tty_path,
                            slave,
                            new_slave_id,
                            reg_protection,
                        } => {
                            match set_new_modbus_id(tty_path, slave, new_slave_id, reg_protection) {
                                Ok(_) => {
                                    show_info(&gui_tx, "Modbus Adresse erfolgreich gesetzt");
                                }
                                Err(error) => show_warning(
                                    &gui_tx,
                                    &format!(
                                        "Konnte Modbus Adresse '{}' nicht speichern:\r\n{}",
                                        &new_slave_id, error
                                    ),
                                ),
                            }
                        }
                        // Neue Arbeitsweise auf Platine speichern
                        ModbusMasterMessage::SetNewWorkingMode {
                            tty_path,
                            slave,
                            working_mode,
                            reg_protection,
                        } => {
                            info!("ModbusMasterMessage::SetNewWorkingMode");
                            // Stop control loop
                            let mut state = is_online.lock().unwrap();
                            *state = false;
                            // Sende register
                            match set_working_mode(tty_path, slave, working_mode, reg_protection) {
                                Ok(_) => {
                                    show_info(&gui_tx, "Arbeitsweise erfolgreich gesetzt");
                                }
                                Err(error) => show_warning(
                                    &gui_tx,
                                    &format!("Konnte Arbeitsweise nicht festlegen:\r\n{}", error),
                                ),
                            }
                        }
                        // Update ein einzelnes Register
                        ModbusMasterMessage::UpdateRegister {
                            tty_path,
                            slave,
                            reg_nr,
                            reg_protection,
                            new_value,
                        } => {
                            match update_register(
                                tty_path,
                                slave,
                                reg_nr,
                                reg_protection,
                                new_value,
                            ) {
                                Ok(_) => {
                                    show_info(&gui_tx, "Register erfolgreich aktualisiert");
                                }
                                Err(error) => show_warning(
                                    &gui_tx,
                                    &format!("Konnte Arbeitsweise nicht festlegen:\r\n{}", error),
                                ),
                            }
                        }
                    }
                }
            });
        });

        ModbusMaster { tx }
    }
}

// Nachrichten die den Control Loop streuern
#[derive(Debug)]
enum MsgControlLoop {
    Start(
        Arc<Mutex<bool>>,
        String,
        u8,
        Vec<Rreg>,
        Vec<Rwreg>,
        u16, // Protection Register Nummer
        Sender<GuiMessage>,
    ),
}

#[allow(unused_variables)]
// Starte Control Loop
fn spawn_control_loop() -> mpsc::Sender<MsgControlLoop> {
    let (tx, mut rx) = mpsc::channel(1);

    thread::spawn(move || {
        let mut rt = Runtime::new().expect("Could not create Runtime");

        rt.block_on(async {
            while let Some(command) = rx.recv().await {
                match command {
                    MsgControlLoop::Start(
                        is_online,
                        tty_path,
                        slave,
                        rregs,
                        rwregs,
                        reg_protection,
                        gui_tx,
                    ) => {
                        debug!("MsgControlLoop::Start verarbeiten");

                        loop {
                            if !(*is_online.lock().unwrap()) {
                                break;
                            };
                            // Lese-Register auslesen
                            let rregs = read_rregs(tty_path.clone(), slave, rregs.clone());
                            // Lese-Register
                            match rregs {
                                Ok(results) => {
                                    // Lese-Register an Gui senden
                                    gui_tx
                                        .clone()
                                        .try_send(GuiMessage::UpdateRregs(results.clone()))
                                        .expect(r#"Failed to send Message"#);
                                    // // Sensor Werte an GUI Elemente senden
                                    // gui_tx
                                    //     .clone()
                                    //     .try_send(GuiMessage::UpdateSensorValues(results.clone()))
                                    //     .expect(r#"Failed to send Message"#);
                                }
                                Err(error) => {
                                    // Fehler an GUI Sensen
                                    show_warning(
                                        &gui_tx,
                                        &format!("Konnte Lese-Register nicht lesen:\r\n{}", error),
                                    )
                                }
                            }

                            #[cfg(feature = "ra-gas")]
                            // Schreib.-/ Lese-Register auslesen
                            let rwregs = read_rwregs(
                                tty_path.clone(),
                                slave,
                                rwregs.clone(),
                                reg_protection,
                            );
                            #[cfg(feature = "ra-gas")]
                            // Schreib.-/ Lese-Register
                            match rwregs {
                                // Schreib.-/ Lese-Register an Gui senden
                                Ok(results) => {
                                    gui_tx
                                        .clone()
                                        .try_send(GuiMessage::UpdateRwregs(results))
                                        .expect(r#"Failed to send Message"#);
                                }
                                // Schreib.-/ Lese-Register in GUI aktualisieren
                                Err(error) => show_warning(
                                    &gui_tx,
                                    &format!(
                                        "Konnte Schreib.-/ Lese-Register nicht lesen:\r\n{}",
                                        error
                                    ),
                                ),
                            }
                        }
                    }
                }
            }
        });
    });

    tx
}

/// Diese Funktion iteriert über die Lese-Register und liest diese
/// sequenziell (nach einander) aus
fn read_rregs(
    tty_path: String,
    slave: u8,
    regs: Vec<Rreg>,
) -> Result<Vec<(u16, u16)>, ModbusMasterError> {
    debug!("read_rregs");

    let mut result: Vec<(u16, u16)> = vec![];
    for reg in regs {
        match read_input_register(&tty_path, slave, reg) {
            Ok(tupple) => result.push(tupple),
            Err(error) => return Err(error),
        }
    }
    // thread::sleep(std::time::Duration::from_millis(LOCK_TIMEOUT));
    Ok(result)
}

#[cfg(feature = "ra-gas")]
/// Diese Funktion iteriert über die Schreib.-/ Lese-Register und liest diese
/// sequenziell (nach einander) aus
fn read_rwregs(
    tty_path: String,
    slave: u8,
    regs: Vec<Rwreg>,
    reg_protection: u16,
) -> Result<Vec<(u16, u16)>, ModbusMasterError> {
    debug!("read_rwregs");

    let mut result: Vec<(u16, u16)> = vec![];
    for reg in regs {
        match read_holding_register(&tty_path, slave, reg, reg_protection) {
            Ok(tupple) => result.push(tupple),
            Err(error) => return Err(error),
        }
    }

    // thread::sleep(std::time::Duration::from_millis(LOCK_TIMEOUT));
    Ok(result)
}

// Liest die Input Register (0x04) (Lese-Register)
//
// Diese Funktion ist einfach. Sie liest immer ein Register aus und gibt den
// Wert oder ein Fehler zurück.
fn read_input_register(
    tty_path: &str,
    slave: u8,
    reg: Rreg,
) -> Result<(u16, u16), ModbusMasterError> {
    debug!("read_input_register");

    let reg_nr = reg.reg_nr() as u16;
    let mut value = vec![0u16; 1];

    let mut modbus = Modbus::new_rtu(&tty_path, 9600, 'N', 8, 1)?;
    modbus.set_slave(slave)?;
    // modbus.set_debug(true)?;

    match modbus.connect() {
        Ok(_) => {
            modbus.read_input_registers(reg_nr, 1, &mut value)?;
        }
        Err(e) => return Err(ModbusMasterError::ReadInputRegister { reg_nr, source: e }),
    }

    let value = (reg_nr, value[0]);

    debug!("Rreg: (reg_nr, value): {:?}", &value);
    Ok(value)
}

#[cfg(feature = "ra-gas")]
// Liest die Holding Register (0x03) (Schreib.-/ Lese-Register)
//
// Im Prinzip funktioniert diese Funktion wie `read_input_register` jedoch
// gibt es bei den (RA-GAS Sensoren vom Typ: Sensor-MB-x) so genannte
// "gesperrte" Register. Diese Register sind nur nach "Eingabe" eines Freigabe
// Codes lesbar. Der Code wird in ein Register geschreiben.
fn read_holding_register(
    tty_path: &str,
    slave: u8,
    reg: Rwreg,
    reg_protection: u16,
) -> Result<(u16, u16), ModbusMasterError> {
    debug!("read_holding_register");

    let mut modbus = Modbus::new_rtu(&tty_path, 9600, 'N', 8, 1)?;
    modbus.set_slave(slave)?;
    // modbus.set_debug(true)?;
    let reg_nr = reg.reg_nr() as u16;
    let mut value = vec![0u16; 1];

    match modbus.connect() {
        Ok(_) => {
            if reg.is_protected() {
                modbus.write_register(reg_protection, 9876)?;
                thread::sleep(std::time::Duration::from_millis(LOCK_TIMEOUT));
            }
            modbus.read_registers(reg_nr, 1, &mut value)?;
        }
        Err(e) => return Err(ModbusMasterError::ReadHoldingRegister { reg_nr, source: e }),
    }
    let value = (reg_nr, value[0]);

    debug!("Rreg: (reg_nr, value): {:?}", &value);
    // thread::sleep(std::time::Duration::from_millis(LOCK_TIMEOUT));
    Ok(value)
}

// Setzt die Arbeitsweise des Sensors (Rwreg 99)
fn set_working_mode(
    tty_path: String,
    slave: u8,
    working_mode: u16,
    reg_protection: u16,
) -> Result<(), ModbusMasterError> {
    debug!("set_working_mode: {:?}", working_mode);

    let mut modbus = Modbus::new_rtu(&tty_path, 9600, 'N', 8, 1)?;
    modbus.set_slave(slave)?;
    // modbus.set_debug(true)?;

    match modbus.connect() {
        Ok(_) => {
            // Entsperren
            modbus.write_register(reg_protection, 9876)?;
            thread::sleep(std::time::Duration::from_millis(LOCK_TIMEOUT));
            // Arbeitsweise setzen
            modbus.write_register(99, working_mode)?;
        }
        Err(e) => return Err(e.into()),
    }

    Ok(())
}

// Nullgas Rwreg 10 - 11111
fn set_nullgas(
    tty_path: String,
    slave: u8,
    reg_protection: u16,
    sensor_num: u16,
) -> Result<(), ModbusMasterError> {
    debug!("set_nullgas");

    // Register Nummer Nullgas
    let nullgas_reg_nr = if sensor_num == 1 { 10 } else { 20 };

    let mut modbus = Modbus::new_rtu(&tty_path, 9600, 'N', 8, 1)?;
    modbus.set_slave(slave)?;
    // modbus.set_debug(true)?;

    match modbus.connect() {
        Ok(_) => {
            // Entsperren
            modbus.write_register(reg_protection, 9876)?;
            thread::sleep(std::time::Duration::from_millis(LOCK_TIMEOUT));
            // Nullpunkt festlegen
            modbus.write_register(nullgas_reg_nr, 11111)?;
        }
        Err(e) => return Err(e.into()),
    }

    Ok(())
}

// Messgas Rwreg 12 - 11111
fn set_messgas(
    tty_path: String,
    slave: u8,
    reg_protection: u16,
    sensor_num: u16,
) -> Result<(), ModbusMasterError> {
    debug!("set_messgas");

    // Register Nummer Messgas
    let messgas_reg_nr = if sensor_num == 1 { 12 } else { 22 };

    let mut modbus = Modbus::new_rtu(&tty_path, 9600, 'N', 8, 1)?;
    modbus.set_slave(slave)?;
    // modbus.set_debug(true)?;

    match modbus.connect() {
        Ok(_) => {
            // Entsperren
            modbus.write_register(reg_protection, 9876)?;
            thread::sleep(std::time::Duration::from_millis(LOCK_TIMEOUT));
            // Messgas festlegen
            modbus.write_register(messgas_reg_nr, 11111)?;
        }
        Err(e) => return Err(e.into()),
    }

    Ok(())
}

// Speichert die neue Modbus Adresse (Rwreg 80)
fn set_new_modbus_id(
    tty_path: String,
    slave: u8,
    new_slave_id: u16,
    reg_protection: u16,
) -> Result<(), ModbusMasterError> {
    debug!(
        "set_new_modbus_id: tty_path: {}, slave: {}, new_slave_id: {}",
        tty_path, slave, new_slave_id
    );

    let mut modbus = Modbus::new_rtu(&tty_path, 9600, 'N', 8, 1)?;
    modbus.set_slave(slave)?;
    // modbus.set_debug(true)?;

    match modbus.connect() {
        Ok(_) => {
            // Entsperren
            modbus.write_register(reg_protection, 9876)?;
            thread::sleep(std::time::Duration::from_millis(LOCK_TIMEOUT));
            // Modbus Slave ID festlegen
            modbus.write_register(80, new_slave_id)?;
        }
        Err(e) => return Err(e.into()),
    }

    Ok(())
}

// Speichert die neue MCS Bus Adresse (Rwreg 95)
fn set_new_mcs_bus_id(
    tty_path: String,
    slave: u8,
    new_slave_id: u16,
    reg_protection: u16,
) -> Result<(), ModbusMasterError> {
    debug!(
        "new_mcs_slave_id: tty_path: {}, slave: {}, new_slave_id: {}",
        tty_path, slave, new_slave_id
    );

    let mut modbus = Modbus::new_rtu(&tty_path, 9600, 'N', 8, 1)?;
    modbus.set_slave(slave)?;
    // modbus.set_debug(true)?;

    match modbus.connect() {
        Ok(_) => {
            // Entsperren
            modbus.write_register(reg_protection, 9876)?;
            thread::sleep(std::time::Duration::from_millis(LOCK_TIMEOUT));
            // MCS ID festlegen
            modbus.write_register(95, new_slave_id)?;
        }
        Err(e) => return Err(e.into()),
    }

    Ok(())
}

/// Update ein Register
///
fn update_register(
    tty_path: String,
    slave: u8,
    reg_nr: u16,
    reg_protection: u16,
    new_value: u16,
) -> Result<(), ModbusMasterError> {
    debug!(
        "update_register: tty_path: {}, slave: {}, reg_nr: {}",
        tty_path, slave, reg_nr
    );

    let mut modbus = Modbus::new_rtu(&tty_path, 9600, 'N', 8, 1)?;
    modbus.set_slave(slave)?;
    // modbus.set_debug(true)?;

    match modbus.connect() {
        Ok(_) => {
            // Entsperren
            modbus.write_register(reg_protection, 9876)?;
            thread::sleep(std::time::Duration::from_millis(LOCK_TIMEOUT));
            // Wert schreiben
            modbus.write_register(reg_nr, new_value)?;
        }
        Err(e) => return Err(e.into()),
    }

    Ok(())
}
