use thiserror::Error;

/// Hardware pin assignments for the ESP32.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GpioPin {
    pub number: u8,
}

/// Depth sensor configuration contract for ESP32.
#[derive(Debug, Clone)]
pub struct DepthSensorConfig {
    pub trig_pin: GpioPin,
    pub echo_pin: GpioPin,
    pub max_range_cm: f32,
    pub sample_rate_hz: u32,
}

/// Result of a depth measurement.
#[derive(Debug, Clone, Copy)]
pub struct DepthReading {
    pub distance_cm: f32,
    pub timestamp_ms: u64,
}

/// Errors returned by the ESP32 depth-sensor contract.
#[derive(Debug, Error)]
pub enum Esp32ContractError {
    #[error("invalid GPIO pin {0}")]
    InvalidPin(u8),
    #[error("sensor timeout after {0} ms")]
    Timeout(u64),
    #[error("i2c bus error: {0}")]
    I2cError(String),
    #[error("measurement out of range: {0} cm")]
    OutOfRange(f32),
}

/// Contract trait that ESP32 hardware must implement for depth casting.
pub trait Esp32DepthContract {
    fn init(&mut self, config: &DepthSensorConfig) -> Result<(), Esp32ContractError>;
    fn read_depth(&mut self) -> Result<DepthReading, Esp32ContractError>;
    fn calibrate(&mut self) -> Result<(), Esp32ContractError>;
}

impl Default for DepthSensorConfig {
    fn default() -> Self {
        Self {
            trig_pin: GpioPin { number: 5 },
            echo_pin: GpioPin { number: 18 },
            max_range_cm: 400.0,
            sample_rate_hz: 10,
        }
    }
}
