use tokio_modbus::{
    prelude::*,
    client::{
        util::{reconnect_shared_context, NewContext, SharedContext},
        Context,
    },
};
use tokio_serial::{Serial, SerialPortSettings};
use tokio::runtime::Runtime;
use std::{
    cell::RefCell,
    future::Future,
    io::Error,
    pin::Pin,
    rc::Rc,
    fmt,
};

#[derive(Debug)]
enum ClientError {
    ReadRRegs { source: std::io::Error },
    ReadRwRegs { source: std::io::Error },
    IoError(Error),
}

impl fmt::Display for ClientError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            ClientError::ReadRRegs  { ref source }=> write!(f, "Could not read Read Register"),
            ClientError::ReadRwRegs { ref source } => write!(f, "Could not read Read/ Write Register"),
            ClientError::IoError(ref error) => write!(f, "Io Error"),
        }
    }
}

impl From<Error> for ClientError {
    fn from(error: Error) -> Self {
        ClientError::IoError(error)
    }
}

impl std::error::Error for ClientError {}

struct Client {
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

impl Client {
    async fn new(path: String) -> Self {
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

        Client {
            shared_context,
        }
    }

    async fn reconnect(&self) -> Result<(), ClientError> {
        reconnect_shared_context(&self.shared_context).await?;

        Ok(())
    }

    async fn nullpunkt(&self) -> Result<(), ClientError> {
        Ok(())
    }

    async fn messgas(&self) -> Result<(), ClientError> {
        Ok(())
    }

    async fn set_slave(&self, id: u8) {
        // &mut self.context.set_slave(id.into());
    }

    async fn new_working_mode(&self, mode: u16) -> Result<(), ClientError> {
        // // entsperren
        // &mut self.context.write_single_register(79, 9876).await?;

        // // set new working mode
        // println!("net new working mode to: {}", mode);
        // &mut self.context.write_single_register(99, mode).await?;
        Ok(())
    }

    async fn read_rregs(&self, rregs: &[u16]) -> Result<Vec<u16>, ClientError> {
        let mut regs = rregs;

        &self.reconnect().await;
        // &self.connect_slave().await;
        // &self.shared_context.borrow().connect_slave(247.into());
        assert!(&self.shared_context.borrow().is_connected());

        // let context = &self.shared_context.borrow().share_context().unwrap();
        // let mut context = context.borrow_mut();
        let context = &self.shared_context.borrow().share_context().unwrap();
        let mut context = context.borrow_mut();
        context.set_slave(247.into());
        let result = context.read_input_registers(0u16, 10).await?;
        println!("{:#?}", result);

        // // this construct fulfills two jobs
        // // First it converts the `&mut vec` into a `vec`
        // match &mut self.context.read_input_registers(0u16, 1u16).await {
        //     Ok(result) => Ok(result.to_vec()),
        //     Err(e) => Err(ClientError::ReadRRegs { source: e }),
        // }
        Ok(regs.to_vec())
    }

    async fn read_rwregs(&self, rwregs: &[u16]) -> Result<Vec<u16>, ClientError> {
        let mut regs = rwregs;
        // // entsperren
        // &mut self.context.write_single_register(49, 9876).await?;

        // for (i, &reg)in regs.iter().enumerate() {
        //     match &mut self.context.read_holding_registers(i as u16, 1).await {
        //         Ok(value) => println!("i {}, reg {}, value {:?}", i, reg, value),
        //         Err(e) => (),
        //     }
        // };
        Ok(regs.to_vec())
    }
}

fn main() -> Result<(), Error> {
    let mut rt = Runtime::new()?;

    rt.block_on(async {
        let mut client = Client::new("/dev/ttyUSB0".to_string()).await;
        let rregs = vec![0u16; 10];
        let rwregs = vec![0u16; 100];

        // // client.new_working_mode(430).await.map_err(|e| println!("Error: {}", e));

        let res = client.read_rregs(&rregs).await.map_err(|e| println!("Error: {}", e));
        println!("{:#?}", res);

        // let res = client.read_rwregs(&rwregs).await.map_err(|e| println!("Error: {}", e));
    });

    Ok(())
}
