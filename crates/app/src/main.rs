use esp32_contract::{
    DepthReading, DepthSensorConfig, Esp32ContractError, Esp32DepthContract,
};

/// Mock implementation of the ESP32 depth-sensor contract for development/testing.
struct MockEsp32Depth {
    config: Option<DepthSensorConfig>,
    calibrated: bool,
}

impl MockEsp32Depth {
    fn new() -> Self {
        Self {
            config: None,
            calibrated: false,
        }
    }
}

impl Esp32DepthContract for MockEsp32Depth {
    fn init(&mut self, config: &DepthSensorConfig) -> Result<(), Esp32ContractError> {
        self.config = Some(config.clone());
        Ok(())
    }

    fn read_depth(&mut self) -> Result<DepthReading, Esp32ContractError> {
        if self.config.is_none() {
            return Err(Esp32ContractError::I2cError(
                "sensor not initialized".into(),
            ));
        }
        if !self.calibrated {
            return Err(Esp32ContractError::I2cError(
                "sensor not calibrated".into(),
            ));
        }
        Ok(DepthReading {
            distance_cm: 42.5,
            timestamp_ms: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64,
        })
    }

    fn calibrate(&mut self) -> Result<(), Esp32ContractError> {
        self.calibrated = true;
        Ok(())
    }
}

fn main() -> Result<(), Esp32ContractError> {
    let mut sensor = MockEsp32Depth::new();

    let config = DepthSensorConfig::default();
    sensor.init(&config)?;
    sensor.calibrate()?;

    let reading = sensor.read_depth()?;
    println!(
        "DepthCast reading: {:.2} cm at {} ms",
        reading.distance_cm, reading.timestamp_ms
    );

    Ok(())
}
