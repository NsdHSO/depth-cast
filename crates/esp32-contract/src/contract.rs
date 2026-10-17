use crate::{config::DepthSensorConfig, error::Esp32ContractError, reading::DepthReading};

/// Contract trait that ESP32 hardware must implement for depth casting.
pub trait Esp32DepthContract {
    fn init(&mut self, config: &DepthSensorConfig) -> Result<(), Esp32ContractError>;
    fn read_depth(&mut self) -> Result<DepthReading, Esp32ContractError>;
    fn calibrate(&mut self) -> Result<(), Esp32ContractError>;
}
