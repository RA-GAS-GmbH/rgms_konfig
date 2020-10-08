use std::{cell::RefCell, fmt, future::Future, io::Error, pin::Pin, rc::Rc};
use tokio::runtime::Runtime;
use tokio_modbus::{
    client::{
        util::{reconnect_shared_context, NewContext, SharedContext},
        Context,
    },
    prelude::*,
};
use tokio_serial::{Serial, SerialPortSettings};

#[derive(Debug)]
enum ModbusRtuMasterError {
    ReadRRegs { source: std::io::Error },
    ReadRwRegs { source: std::io::Error },
    InitFailure,
    IoError(Error),
    NoSharedContext,
}

impl fmt::Display for ModbusRtuMasterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            ModbusRtuMasterError::ReadRRegs { ref source } => {
                write!(f, "Could not read Read Register")
            }
            ModbusRtuMasterError::ReadRwRegs { ref source } => {
                write!(f, "Could not read Read/ Write Register")
            }
            ModbusRtuMasterError::InitFailure => write!(f, "ModbusRtuMaster could not initalized"),
            ModbusRtuMasterError::IoError(ref _error) => write!(f, "Io Error"),
            ModbusRtuMasterError::NoSharedContext => write!(f, "Could not create shared context."),
        }
    }
}

impl From<Error> for ModbusRtuMasterError {
    fn from(error: Error) -> Self {
        ModbusRtuMasterError::IoError(error)
    }
}

impl std::error::Error for ModbusRtuMasterError {}

/// Modbus RTU Master
pub struct ModbusRtuMaster {
    shared_context: std::rc::Rc<std::cell::RefCell<tokio_modbus::client::util::SharedContext>>,
}

#[derive(Debug)]
struct SerialConfig {
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

impl ModbusRtuMaster {
    /// Create a new Modbus RTU Master
    pub fn new(path: String) -> Self {
        let serial_config = SerialConfig {
            path,
            settings: SerialPortSettings {
                baud_rate: 9600,
                ..Default::default()
            },
        };

        let shared_context = Rc::new(RefCell::new(SharedContext::new(
            None, // no initial context, i.e. not connected
            Box::new(serial_config),
        )));

        ModbusRtuMaster { shared_context }
    }

    async fn init(&self) {
        &self.reconnect().await;
        assert!(&self.shared_context.borrow().is_connected());
    }

    async fn reconnect(&self) -> Result<(), ModbusRtuMasterError> {
        reconnect_shared_context(&self.shared_context)
            .await
            .map_err(|e| e.into())
    }

    async fn nullpunkt(&self) -> Result<(), ModbusRtuMasterError> {
        Ok(())
    }

    async fn messgas(&self) -> Result<(), ModbusRtuMasterError> {
        Ok(())
    }

    async fn set_slave(&self, id: u8) -> Result<(), ModbusRtuMasterError> {
        let context = &self
            .shared_context
            .borrow()
            .share_context()
            .ok_or(ModbusRtuMasterError::NoSharedContext)?;
        let mut context = context.borrow_mut();
        context.set_slave(id.into());

        Ok(())
    }

    async fn new_working_mode(&self, mode: u16) -> Result<(), ModbusRtuMasterError> {
        // // entsperren
        // &mut self.context.write_single_register(79, 9876).await?;

        // // set new working mode
        // println!("net new working mode to: {}", mode);
        // &mut self.context.write_single_register(99, mode).await?;
        Ok(())
    }

    async fn read_rregs(&self, rregs: &[u16]) -> Result<Vec<u16>, ModbusRtuMasterError> {
        let mut regs = rregs;

        let context = &self
            .shared_context
            .borrow()
            .share_context()
            .ok_or(ModbusRtuMasterError::NoSharedContext)?;
        let mut context = context.borrow_mut();
        context.set_slave(247.into());
        let result = context.read_input_registers(0u16, 10).await?;

        Ok(result)
    }

    async fn read_rwregs(&self, rwregs: &[u16]) -> Result<Vec<u16>, ModbusRtuMasterError> {
        let mut regs = rwregs;

        let context = &self
            .shared_context
            .borrow()
            .share_context()
            .ok_or(ModbusRtuMasterError::NoSharedContext)?;
        let mut context = context.borrow_mut();

        // entsperren
        // context.write_single_register(49, 9876).await?;

        let result = context.read_holding_registers(90u16, 10).await?;
        // for (i, &reg)in regs.iter().enumerate() {
        //     match &mut self.context.read_holding_registers(i as u16, 1).await {
        //         Ok(value) => println!("i {}, reg {}, value {:?}", i, reg, value),
        //         Err(e) => (),
        //     }
        // };
        Ok(result)
    }
}
