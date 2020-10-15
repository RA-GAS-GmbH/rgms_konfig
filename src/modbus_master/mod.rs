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
#[derive(Debug)]
pub enum ModbusMasterMessage {
    /// Starte Control Loop
    Connect(String, u8, Vec<Rreg>, Vec<Rwreg>),
    /// Stoppe Control Loop
    Disconnect,
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
                        ModbusMasterMessage::Connect(tty_path, slave, rregs, rwregs) => {
                            info!("ModbusMasterMessage::Connect");
                            // debug!("tty_path: {}, slave: {}, rregs: {:?}, rwregs: {:?}", tty_path, slave, rregs, rwregs);

                            let mut state = is_online.lock().await;
                            *state = true;

                            match control_loop_tx.try_send(Msg::ReadRegister(
                                is_online.clone(),
                                modbus_rtu_context.clone(),
                                tty_path,
                                slave,
                                rregs,
                                rwregs,
                                gui_tx.clone(),
                            )) {
                                Ok(_empty_tupple) => {
                                    // TODO: disable GUI Elements here?
                                }
                                Err(e) => {
                                    gui_tx
                                        .clone()
                                        .try_send(GuiMessage::ShowWarning(format!(
                                            "Control Loop konnte nicht erreicht werden: {}",
                                            e
                                        )))
                                        .expect(r#"Failed to send Message"#);
                                }
                            }
                        }

                        ModbusMasterMessage::Disconnect => {
                            println!("ModbusMasterMessage::Disconnect");
                            let mut state = is_online.lock().await;
                            *state = false;
                        }
                    }
                }
            });
        });

        ModbusMaster { tx }
    }
}

// FIXME: Besserer Name?
#[derive(Debug)]
enum Msg {
    ReadRegister(
        Arc<Mutex<bool>>,
        ModbusRtuContext,
        String,
        u8,
        Vec<Rreg>,
        Vec<Rwreg>,
        Sender<GuiMessage>,
    ),
    // Stop(Arc<Mutex<bool>>),
}

fn spawn_control_loop() -> mpsc::Sender<Msg> {
    let (tx, mut rx) = mpsc::channel(1);

    thread::spawn(move || {
        let mut rt = Runtime::new().expect("Could not create Runtime");

        rt.block_on(async {
            while let Some(command) = rx.recv().await {
                match command {
                    Msg::ReadRegister(
                        is_online,
                        modbus_rtu_context,
                        tty_path,
                        slave,
                        rregs,
                        rwregs,
                        gui_tx,
                    ) => {
                        println!("Msg::ReadRegister");

                        loop {
                            if *is_online.lock().await == false {
                                break;
                            };
                            // Lese Register auslesen
                            let rregs = read_rregs(
                                modbus_rtu_context.clone(),
                                tty_path.clone(),
                                slave,
                                rregs.clone(),
                            )
                            .await;
                            // Lese Register an Gui senden
                            gui_tx
                                .clone()
                                .try_send(GuiMessage::UpdateRregs(rregs))
                                .expect(r#"Failed to send Message"#);

                            // Schreib/ Lese Register auslesen
                            let rwregs = read_rwregs(
                                modbus_rtu_context.clone(),
                                tty_path.clone(),
                                slave,
                                rwregs.clone(),
                            )
                            .await;
                            // Schreib/ Lese Register an Gui senden
                            gui_tx
                                .clone()
                                .try_send(GuiMessage::UpdateRwregs(rwregs))
                                .expect(r#"Failed to send Message"#);

                            thread::sleep(std::time::Duration::from_millis(1000));
                        }
                    }
                }
            }
        });
    });

    tx
}

/// Diese Funktion iteriert über die Lese Register und liest diese
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

/// Diese Funktion iteriert über die Schreib/ Lese Register und liest diese
/// sequenziell (nach einander) aus
async fn read_rwregs(
    modbus_rtu_context: ModbusRtuContext,
    tty_path: String,
    slave: u8,
    regs: Vec<Rwreg>,
) -> Result<Vec<(u16, u16)>, ModbusMasterError> {
    let mut result: Vec<(u16, u16)> = vec![];
    for reg in regs {
        match read_holding_register(
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
// FIXME: Holding Register gehen garnicht
async fn read_holding_register(
    modbus_rtu_context: ModbusRtuContext,
    tty_path: String,
    slave: u8,
    reg: Rwreg,
) -> Result<(u16, u16), ModbusMasterError> {
    let reg_nr = reg.reg_nr() as u16;
    let mut ctx = modbus_rtu_context.context(tty_path, slave).await?;
    // let reg_protection = platine.reg_protection();
    let reg_protection = 49u16;

    // FIXME: Bessere Fehlermelung
    // FIXME: Fehler beim Lesen, warscheinlich nicht dokumentiertes gesperrtes Register
    if reg.is_protected() {
        // FIXME: Urgend! Hard coded control_register problem!
        ctx.write_single_register(reg_protection, 9876).await?;
        // FIXME: Hässlicher Timeout , nötig damit die nächsten Register gelesen werden können
        thread::sleep(std::time::Duration::from_millis(20));
    }

    let value = match ctx.read_holding_registers(reg_nr, 1).await {
        Ok(value) => Ok((reg_nr, value[0])),
        Err(e) => Err(ModbusMasterError::ReadHoldingRegister(reg_nr, e)),
    };

    // // debug
    // println!("reg: {:?} value: {:?}", &reg, &value);

    value
}
