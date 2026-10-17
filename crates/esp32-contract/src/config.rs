use crate::pins::GpioPin;

/// Depth sensor configuration contract for ESP32.
#[derive(Debug, Clone)]
pub struct DepthSensorConfig {
    pub trig_pin: GpioPin,
    pub echo_pin: GpioPin,
    pub max_range_cm: f32,
    pub sample_rate_hz: u32,
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
