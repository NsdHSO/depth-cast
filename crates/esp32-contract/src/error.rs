use thiserror::Error;

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
