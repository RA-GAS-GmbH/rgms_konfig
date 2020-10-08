pub(crate) mod error;


pub(crate) mod context {
    
    
    use tokio_modbus::{
        client::{
            Context,
        },
        prelude::*,
    };
    use tokio_serial::{Serial, SerialPortSettings};

    // use std::sync::{Arc, Mutex};
    
    

    #[derive(Debug)]
    /// SerialConfig
    pub struct SerialConfig {
        path: String,
        settings: SerialPortSettings,
    }

    /// Modbus RTU Master
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
            let port = Serial::from_path(tty_path, &settings).unwrap();

            let ctx = rtu::connect_slave(port, slave.into()).await.unwrap();
            ctx
        }
    }
}
use context::ModbusRtuContext;

use crate::{
    gui::gtk3::GuiMessage,
    registers::{Rreg, Rwreg},
};
use futures::channel::mpsc::Sender;

use tokio::{runtime::Runtime, sync::mpsc};

use tokio_modbus::prelude::*;


/// Possible ModbusMaster commands
pub enum ModbusMasterMessage {
    /// Connect  (tty_path, slave)
    Connect(String, u8, Vec<Rreg>, Vec<Rwreg>),
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

        std::thread::spawn(move || {
            let mut rt = Runtime::new().expect("Could not create Runtime");

            rt.block_on(async {
                while let Some(command) = rx.recv().await {
                    match command {
                        ModbusMasterMessage::Connect(tty_path, slave, _rregs, _rwregs) => {
                            // println!("{:#?}", rregs);

                            let mut ctx = modbus_rtu_context.context(tty_path, slave).await;

                            let res = ctx.read_input_registers(0u16, 10).await;
                            gui_tx
                                .clone()
                                .try_send(GuiMessage::ShowInfo(format!("{:?}", res)))
                                .expect(r#"Failed to send Message"#);
                        }
                    }
                }
            });
        });

        ModbusMaster { tx }
    }
}
