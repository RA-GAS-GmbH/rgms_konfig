/// Fehler die im Modbus RTU Context auftreten können
pub mod context_error;

/// ModbusMaster Fehler
pub mod error;

/// Modbus RTU Context
pub mod context;

use context::ModbusRtuContext;
pub use error::ModbusMasterError;

use crate::{
    gui::gtk3::GuiMessage,
    registers::{Rreg, Rwreg},
};
use futures::channel::mpsc::Sender;
use std::sync::Arc;
use std::thread;
use tokio::sync::Mutex;
use tokio::{runtime::Runtime, sync::mpsc};
use tokio_modbus::prelude::*;

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
    },
    /// Messgas
    Messgas {
        /// serielle Schnittstelle
        tty_path: String,
        /// Modbus Slave ID
        slave: u8,
        /// Entsperr Register Nummer
        reg_protection: u16,
    },
    /// Setzt die Arbeitsweise
    SetNewWorkingMode(String, u8, u16, u16),
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
        // erzeugt den RTU Context
        let modbus_rtu_context = ModbusRtuContext::new();
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

                            let mut state = is_online.lock().await;
                            *state = true;

                            // Sende Start Commando an Control Loop
                            match control_loop_tx.try_send(MsgControlLoop::Start(
                                is_online.clone(),
                                modbus_rtu_context.clone(),
                                tty_path,
                                slave,
                                rregs,
                                rwregs,
                                reg_protection,
                                gui_tx.clone(),
                            )) {
                                Ok(_empty_tupple) => {
                                    // TODO: disable GUI Elements here?
                                }
                                Err(error) => {
                                    gui_tx
                                        .clone()
                                        .try_send(GuiMessage::ShowWarning(format!(
                                            "Control Loop konnte nicht erreicht werden: {}",
                                            error
                                        )))
                                        .expect(r#"Failed to send Message"#);
                                }
                            }
                        }
                        ModbusMasterMessage::Disconnect => {
                            info!("ModbusMasterMessage::Disconnect");
                            let mut state = is_online.lock().await;
                            *state = false;
                        }
                        ModbusMasterMessage::Nullgas {tty_path, slave, reg_protection} => {
                            match nullgas(modbus_rtu_context.clone(), tty_path, slave, reg_protection).await {
                                Ok(_) => {}
                                Err(_error) => {}
                            }
                        },
                        ModbusMasterMessage::Messgas {tty_path, slave, reg_protection} => {
                            match messgas(modbus_rtu_context.clone(), tty_path, slave, reg_protection).await {
                                Ok(_) => {}
                                Err(_error) => {}
                            }
                        },
                        ModbusMasterMessage::SetNewWorkingMode(
                            tty_path,
                            slave,
                            working_mode,
                            reg_protection,
                        ) => {
                            info!("ModbusMasterMessage::SetNewWorkingMode");
                            // Stop control loop
                            let mut state = is_online.lock().await;
                            *state = false;
                            // Sende register
                            match set_working_mode(
                                modbus_rtu_context.clone(),
                                tty_path,
                                slave,
                                working_mode,
                                reg_protection,
                            )
                            .await
                            {
                                Ok(_) => {}
                                Err(error) => {
                                    gui_tx
                                        .clone()
                                        .try_send(GuiMessage::ShowWarning(format!(
                                            "Konnte Arbeitsweise nicht festlegen:\r\n{}",
                                            error
                                        )))
                                        .expect(r#"Failed to send Message"#);
                                }
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
        ModbusRtuContext,
        String,
        u8,
        Vec<Rreg>,
        Vec<Rwreg>,
        u16, // Protection Register Nummer
        Sender<GuiMessage>,
    ),
}

fn spawn_control_loop() -> mpsc::Sender<MsgControlLoop> {
    let (tx, mut rx) = mpsc::channel(1);

    thread::spawn(move || {
        let mut rt = Runtime::new().expect("Could not create Runtime");

        rt.block_on(async {
            while let Some(command) = rx.recv().await {
                match command {
                    MsgControlLoop::Start(
                        is_online,
                        modbus_rtu_context,
                        tty_path,
                        slave,
                        rregs,
                        rwregs,
                        reg_protection,
                        gui_tx,
                    ) => {
                        println!("Msg::Start");

                        loop {
                            if *is_online.lock().await == false {
                                break;
                            };
                            // Lese-Register auslesen
                            let rregs = read_rregs(
                                modbus_rtu_context.clone(),
                                tty_path.clone(),
                                slave,
                                rregs.clone(),
                            )
                            .await;
                            // Lese-Register an GUI senden
                            match rregs {
                                Ok(results) => {
                                    // Lese-Register an Gui senden
                                    gui_tx
                                        .clone()
                                        .try_send(GuiMessage::UpdateRregs(results.clone()))
                                        .expect(r#"Failed to send Message"#);
                                    gui_tx
                                        .clone()
                                        .try_send(GuiMessage::UpdateSensorValues(results.clone()))
                                        .expect(r#"Failed to send Message"#);
                                }
                                Err(e) => {
                                    gui_tx
                                        .clone()
                                        .try_send(GuiMessage::ShowWarning(format!(
                                            "Konnte Lese-Register nicht lesen:\r\n{}",
                                            e
                                        )))
                                        .expect(r#"Failed to send Message"#);
                                }
                            }

                            // Schreib.-/ Lese-Register auslesen
                            let rwregs = read_rwregs(
                                modbus_rtu_context.clone(),
                                tty_path.clone(),
                                slave,
                                rwregs.clone(),
                                reg_protection,
                            )
                            .await;
                            // Schreib.-/ Lese-Register an Gui senden
                            match rwregs {
                                Ok(results) => {
                                    gui_tx
                                        .clone()
                                        .try_send(GuiMessage::UpdateRwregs(results))
                                        .expect(r#"Failed to send Message"#);
                                }
                                Err(e) => {
                                    gui_tx
                                        .clone()
                                        .try_send(GuiMessage::ShowWarning(format!(
                                            "Konnte Schreib.-/ Lese-Register nicht lesen:\r\n{}",
                                            e
                                        )))
                                        .expect(r#"Failed to send Message"#);
                                }
                            }
                            thread::sleep(std::time::Duration::from_millis(1000));
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
async fn read_rregs(
    modbus_rtu_context: ModbusRtuContext,
    tty_path: String,
    slave: u8,
    regs: Vec<Rreg>,
) -> Result<Vec<(u16, u16)>, ModbusMasterError> {
    let mut result: Vec<(u16, u16)> = vec![];
    for reg in regs {
        match read_input_register(
            modbus_rtu_context.clone(),
            tty_path.clone(),
            slave.clone(),
            reg,
        )
        .await
        {
            Ok(tupple) => result.push(tupple),
            Err(e) => return Err(e),
        }
    }

    Ok(result)
}

/// Diese Funktion iteriert über die Schreib.-/ Lese-Register und liest diese
/// sequenziell (nach einander) aus
async fn read_rwregs(
    modbus_rtu_context: ModbusRtuContext,
    tty_path: String,
    slave: u8,
    regs: Vec<Rwreg>,
    reg_protection: u16,
) -> Result<Vec<(u16, u16)>, ModbusMasterError> {
    let mut result: Vec<(u16, u16)> = vec![];
    for reg in regs {
        match read_holding_register(
            modbus_rtu_context.clone(),
            tty_path.clone(),
            slave.clone(),
            reg,
            reg_protection,
        )
        .await
        {
            Ok(tupple) => result.push(tupple),
            Err(e) => return Err(e),
        }
    }

    Ok(result)
}

// Liest die Input Register (0x04) (Lese-Register)
//
// Diese Funktion ist einfach. Sie liest immer ein Register aus und gibt den
// Wert oder ein Fehler zurück.
async fn read_input_register(
    modbus_rtu_context: ModbusRtuContext,
    tty_path: String,
    slave: u8,
    reg: Rreg,
) -> Result<(u16, u16), ModbusMasterError> {
    let reg_nr = reg.reg_nr() as u16;
    let mut ctx = modbus_rtu_context.context(tty_path, slave).await?;
    let value = match ctx.read_input_registers(reg_nr, 1).await {
        Ok(value) => Ok((reg_nr, value[0])),
        Err(_) => Err(ModbusMasterError::ReadInputRegister),
    };
    debug!("Rreg: (reg_nr, value): {:?}", &value);
    value
}

// Liest die Holding Register (0x03) (Schreib.-/ Lese-Register)
//
// Im Prinzip funktioniert diese Funktion wie `read_input_register` jedoch
// gibt es bei den (RA-GAS Sensoren vom Typ: Sensor-MB-x) so genannte
// "gesperrte" Register. Diese Register sind nur nach "Eingabe" eines Freigabe
// Codes lesbar. Der Code wird in ein Register geschreiben.
// TODO: Mehr Beschreibung der Freigabe Codes
async fn read_holding_register(
    modbus_rtu_context: ModbusRtuContext,
    tty_path: String,
    slave: u8,
    reg: Rwreg,
    reg_protection: u16,
) -> Result<(u16, u16), ModbusMasterError> {
    let reg_nr = reg.reg_nr() as u16;
    let mut ctx = modbus_rtu_context.context(tty_path, slave).await?;

    // TODO: Bessere Fehlermelung
    if reg.is_protected() {
        ctx.write_single_register(reg_protection, 9876).await?;
        // FIXME: Hässlicher Timeout , nötig damit die nächsten Register gelesen werden können
        thread::sleep(std::time::Duration::from_millis(20));
    }

    let value = match ctx.read_holding_registers(reg_nr, 1).await {
        Ok(value) => Ok((reg_nr, value[0])),
        Err(e) => Err(ModbusMasterError::ReadHoldingRegister(reg_nr, e)),
    };

    value
}

// Setzt die Arbeitsweise des Sensors (Rwreg 99)
async fn set_working_mode(
    modbus_rtu_context: ModbusRtuContext,
    tty_path: String,
    slave: u8,
    working_mode: u16,
    reg_protection: u16,
) -> Result<(), ModbusMasterError> {
    let mut ctx = modbus_rtu_context.context(tty_path, slave).await?;
    ctx.write_single_register(reg_protection, 9876).await?;
    // FIXME: Hässlicher Timeout , nötig damit die nächsten Register gelesen werden können
    thread::sleep(std::time::Duration::from_millis(20));

    ctx.write_single_register(99, working_mode)
        .await
        .map_err(|e| e.into())
}

// Nullgas Rwreg 10 - 11111
async fn nullgas(
    modbus_rtu_context: ModbusRtuContext,
    tty_path: String,
    slave: u8,
    reg_protection: u16,
) -> Result<(), ModbusMasterError> {
    let mut ctx = modbus_rtu_context.context(tty_path, slave).await?;
    ctx.write_single_register(reg_protection, 9876).await?;
    // FIXME: Hässlicher Timeout , nötig damit die nächsten Register gelesen werden können
    thread::sleep(std::time::Duration::from_millis(20));

    ctx.write_single_register(10, 11111)
        .await
        .map_err(|e| e.into())
}

// Messgas Rwreg 12 - 11111
async fn messgas(
    modbus_rtu_context: ModbusRtuContext,
    tty_path: String,
    slave: u8,
    reg_protection: u16,
) -> Result<(), ModbusMasterError> {
    let mut ctx = modbus_rtu_context.context(tty_path, slave).await?;
    ctx.write_single_register(reg_protection, 9876).await?;
    // FIXME: Hässlicher Timeout , nötig damit die nächsten Register gelesen werden können
    thread::sleep(std::time::Duration::from_millis(20));

    ctx.write_single_register(12, 11111)
        .await
        .map_err(|e| e.into())
}
