use rgms_konfig::{
    sensors::SensorMbNe4Legacy,
    modbus_master::ModbusMaster,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // start gui
    // user selects sensor type
    // create sensor
    let _sensor = SensorMbNe4Legacy::new_from_csv();
    // update sensor registers

    let _modbus_master = ModbusMaster::new()?;

    Ok(())
}    
