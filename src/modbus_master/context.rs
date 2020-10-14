use tokio_modbus::{client::Context, prelude::*};
use tokio_serial::{Serial, SerialPortSettings};
use crate::modbus_master::context_error::ContextError as Error;


#[derive(Debug)]
/// SerialConfig
pub struct SerialConfig {
    path: String,
    settings: SerialPortSettings,
}

/// Modbus RTU Context
#[derive(Clone, Debug)]
pub struct ModbusRtuContext {}

impl ModbusRtuContext {
    /// Erstellt einen neuen Modbus RTU Context
    pub fn new() -> Self {
        ModbusRtuContext {}
    }

    /// Liefert den Mobus RTU Context zurück
    /// FIXME: entferne die Unwraps, implementiere ein Result und das Error Handling
    pub async fn context(&self, tty_path: String, slave: u8) -> Context {
        info!("ModbusRtuContext::context");
        let mut settings = SerialPortSettings::default();
        settings.baud_rate = 9600;
        let port = Serial::from_path(tty_path, &settings).unwrap();

        let ctx = rtu::connect_slave(port, slave.into()).await.unwrap();

        ctx
    }

    /// Liefert den Mobus RTU Context zurück
    /// FIXME: entferne die Unwraps, implementiere ein Result und das Error Handling
    pub async fn old_context(&self, tty_path: String, slave: u8) -> Result<Context, Error> {
        info!("ModbusRtuContext::context");
        let mut settings = SerialPortSettings::default();
        settings.baud_rate = 9600;
        let port = Serial::from_path(tty_path, &settings).unwrap();

        let ctx = rtu::connect_slave(port, slave.into()).await.unwrap();

        Ok(ctx)
    }
}
