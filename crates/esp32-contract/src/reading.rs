/// Result of a depth measurement.
#[derive(Debug, Clone, Copy)]
pub struct DepthReading {
    pub distance_cm: f32,
    pub timestamp_ms: u64,
}
