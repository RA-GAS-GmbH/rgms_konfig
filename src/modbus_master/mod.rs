use std::{cell::RefCell, io::Error, rc::Rc};
use std::{future::Future, pin::Pin};
use tokio::runtime::Runtime;
use tokio::sync::mpsc;
use tokio_modbus::client::{
    rtu,
    util::{NewContext, SharedContext},
    Context,
};
use tokio_modbus::prelude::*;
use tokio_serial::{Serial, SerialPortSettings};

/// Possible ModbusMaster commands
#[derive(Debug)]
pub enum ModbusMasterMessage {
    /// Set Modbus Slave Adresse
    SetSlave(u8),
    /// Nullpunktabgleich
    Nullpunkt(u16),
}

#[derive(Debug)]
/// SerialConfig
pub struct SerialConfig {
    path: String,
    settings: SerialPortSettings,
}

impl NewContext for SerialConfig {
    fn new_context(&self) -> Pin<Box<dyn Future<Output = Result<Context, Error>>>> {
        let serial = Serial::from_path(&self.path, &self.settings);
        Box::pin(async {
            let port = serial?;
            rtu::connect(port).await
        })
    }
}

/// Modbus Master
#[derive(Debug)]
pub struct ModbusMaster {
    /// Sender über den mit dem ModbusMaster kommuniziert werden kann
    pub tx: mpsc::Sender<ModbusMasterMessage>,
}

impl ModbusMaster {
    /// Creates a new Modbus Master
    pub fn new() -> ModbusMaster {
        let (tx, mut rx) = mpsc::channel(1);

        std::thread::spawn(move || {
            // let _path = "/dev/ttyUSB0".to_string();
            // let _slave = Slave(247);
            // let mut settings = SerialPortSettings::default();
            // settings.baud_rate = 9600;

            let serial_config = SerialConfig {
                path: "/dev/ttyUSB0".into(),
                settings: SerialPortSettings {
                    baud_rate: 9600,
                    ..Default::default()
                },
            };

            let shared_context = Rc::new(RefCell::new(SharedContext::new(
                None, // no initial context, i.e. not connected
                Box::new(serial_config),
            )));

            let mut rt = Runtime::new().expect("Could not create Runtime");

            rt.block_on(async {
                while let Some(command) = rx.recv().await {
                    match command {
                        ModbusMasterMessage::SetSlave(_slave_id) => {
                            let context = shared_context.borrow().share_context().unwrap();
                            let mut ctx = context.borrow_mut();
                            // FIXME: Remove unwrap()
                            ctx.write_single_register(10u16, 11111u16).await.unwrap();
                        }
                        ModbusMasterMessage::Nullpunkt(reg) => {
                            let context = shared_context.borrow().share_context().unwrap();
                            let mut ctx = context.borrow_mut();
                            // FIXME: Remove unwrap()
                            ctx.write_single_register(reg, 11111u16).await.unwrap();
                        }
                    }
                }
            });
        });

        ModbusMaster { tx }
    }
}