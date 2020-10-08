pub(crate) mod context_error;
pub(crate) mod error;

pub(crate) mod context {

    use tokio_modbus::{client::Context, prelude::*};
    use tokio_serial::{Serial, SerialPortSettings};

    // use std::sync::{Arc, Mutex};

    #[derive(Debug)]
    /// SerialConfig
    pub struct SerialConfig {
        path: String,
        settings: SerialPortSettings,
    }

    /// Modbus RTU Master
    #[derive(Clone)]
    pub struct ModbusRtuContext {}

    impl ModbusRtuContext {
        /// Create a new Modbus RTU Context
        pub fn new() -> Self {
            ModbusRtuContext {}
        }

        /// Get context
        pub async fn context(&self, tty_path: String, slave: u8) -> Context {
            let mut settings = SerialPortSettings::default();
            settings.baud_rate = 9600;
            debug!("tty_path: {}, settings: {:?}", &tty_path, &settings);
            let port = Serial::from_path(tty_path, &settings).unwrap();

            let ctx = rtu::connect_slave(port, slave.into()).await.unwrap();
            ctx
        }
    }
}
use context::ModbusRtuContext;
use error::ModbusMasterError;

use crate::{
    gui::gtk3::GuiMessage,
    registers::{Register, Rreg, Rwreg},
};
use futures::channel::mpsc::{channel, Sender};
use futures::{Future, Sink, Stream};
use std::sync::Arc;
use std::thread;
use tokio::sync::Mutex;
use tokio::{runtime::Runtime, sync::mpsc};
use tokio_modbus::prelude::*;

/// Possible ModbusMaster commands
pub enum ModbusMasterMessage {
    /// Starte Control Loop
    ///
    /// # Parameters
    ///     * 'tty_path'
    ///     * 'slave'
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
    /// Creates a new Modbus Master
    pub fn new(gui_tx: Sender<GuiMessage>) -> ModbusMaster {
        let (tx, mut rx) = mpsc::channel(1);
        let modbus_rtu_context = ModbusRtuContext::new();
        // Control Loop Empfänger
        let mut control_loop_tx = spawn_control_loop();

        std::thread::spawn(move || {
            let mut rt = Runtime::new().expect("Could not create Runtime");

            rt.block_on(async {
                // Control variable die den Control Loop steuert
                let is_online = Arc::new(Mutex::new(false));

                while let Some(command) = rx.recv().await {
                    match command {
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
                                Ok(v) => {}
                                Err(e) => {
                                    gui_tx
                                        .clone()
                                        .try_send(GuiMessage::ShowWarning("Control Loop konnte nicht erreicht werden".to_string()))
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

                            let regs = read_rregs(
                                modbus_rtu_context.clone(),
                                tty_path.clone(),
                                slave,
                                rregs.clone(),
                                gui_tx.clone(),
                            )
                            .await;
                            gui_tx
                                    .clone()
                                    .try_send(GuiMessage::ShowQuestion(format!("{:#?}", regs)))
                                    .expect(r#"Failed to send Message"#);

                            read_rwregs(
                                modbus_rtu_context.clone(),
                                tty_path.clone(),
                                slave,
                                rwregs.clone(),
                                gui_tx.clone(),
                            )
                            .await;

                            thread::sleep(std::time::Duration::from_millis(1000));
                        }
                    }
                }
            }
        });
    });

    tx
}
use futures::stream::{self, StreamExt};
async fn read_rregs(
    modbus_rtu_context: ModbusRtuContext,
    tty_path: String,
    slave: u8,
    rregs: Vec<Rreg>,
    gui_tx: Sender<GuiMessage>,
) -> Vec<(u16, u16)> {
    // let mut ctx = modbus_rtu_context.context(tty_path, slave).await;
    // println!("{:#?}", rregs);

    // Ist Ok geht aber alles parallel
    rregs.iter()
        .map(|reg|{read_input_register(modbus_rtu_context.clone(), tty_path.clone(), slave.clone(), reg)})
        // .collect::<futures::stream::futures_unordered::FuturesUnordered<_>>()
        .collect::<futures::stream::FuturesOrdered<_>>()
        .collect::<Vec<_>>()
        .await;

    // rregs.iter()
    //     .map(|reg| async {
    //         read_input_register(modbus_rtu_context.clone(), tty_path.clone(), slave.clone(), reg)
    //     }).collect::<Vec<_>>();


    vec![(0u16, 0u16)]
}

async fn read_input_register (
    modbus_rtu_context: ModbusRtuContext,
    tty_path: String,
    slave: u8,
    reg: &Rreg,
) -> Result<(u16, u16), ModbusMasterError> {
    let reg_nr = reg.reg_nr() as u16;
    let mut ctx = modbus_rtu_context.context(tty_path, slave).await;
    let value = match ctx.read_holding_registers(0u16, 10).await {
        Ok(value) => Ok((reg_nr, value[0])),
        Err(_) => Err(ModbusMasterError::ReadRreg),
    };
    value
}

async fn read_rwregs(
    modbus_rtu_context: ModbusRtuContext,
    tty_path: String,
    slave: u8,
    rwregs: Vec<Rwreg>,
    gui_tx: Sender<GuiMessage>,
) {
    // let mut ctx = modbus_rtu_context.context(tty_path, slave).await;
    // let res = ctx.read_holding_registers(0u16, 10).await;
    // gui_tx
    //     .clone()
    //     .try_send(GuiMessage::ShowQuestion(format!("{:?}", res)))
    //     .expect(r#"Failed to send Message"#);
}
