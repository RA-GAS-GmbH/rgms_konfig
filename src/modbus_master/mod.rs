use crate::registers::Rreg;
use std::{future::Future, io::Error, pin::Pin};
use tokio::{runtime::Runtime, sync::mpsc};
use tokio_modbus::client::{
    rtu,
    util::NewContext,
    Context,
};

use tokio_serial::{Serial, SerialPortSettings};

/// Possible ModbusMaster commands
pub enum ModbusMasterMessage {
    /// Read Rregs and Rwregs
    Update(Vec<Rreg>, Option<String>, u8),
    /// Read Rregs
    ReadRregs(Option<String>, u8),
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

            let _serial_config = SerialConfig {
                path: "/dev/ttyUSB0".into(),
                settings: SerialPortSettings {
                    baud_rate: 9600,
                    ..Default::default()
                },
            };

            let mut rt = Runtime::new().expect("Could not create Runtime");

            rt.block_on(async {
                while let Some(command) = rx.recv().await {
                    match command {
                        ModbusMasterMessage::Update(_, _, _) => {},
                        ModbusMasterMessage::ReadRregs(_port, _modbus_address) => {
                        }
                        ModbusMasterMessage::SetSlave(_slave_id) => {
                            // let context = shared_context.borrow().share_context().unwrap();
                            // let mut context = context.borrow_mut();
                            // let mut ctx = context.borrow_mut();
                            // context.set_slave(247.into());
                            // // FIXME: Remove unwrap()
                            // ctx.write_single_register(10u16, 11111u16).await.unwrap();
                        }
                        ModbusMasterMessage::Nullpunkt(_reg) => {
                            // let context = shared_context.borrow().share_context().unwrap();
                            // let mut context = context.borrow_mut();
                            // let mut ctx = context.borrow_mut();
                            // context.set_slave(247.into());
                            // // FIXME: Remove unwrap()
                            // ctx.write_single_register(reg, 11111u16).await.unwrap();
                        }
                    }
                }
            });
        });

        ModbusMaster { tx }
    }
}
