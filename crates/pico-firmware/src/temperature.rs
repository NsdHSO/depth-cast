#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SensorError {
    OutOfRange(f32),
}

#[inline]
#[must_use]
pub const fn raw_to_celsius(raw: i16) -> Result<f32, SensorError> {
    let c = raw as f32 * 0.0625;
    if c < -55.0 || c > 125.0 {
        Err(SensorError::OutOfRange(c))
    } else {
        Ok(c)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn datasheet_table1() {
        let raw_words: [(u16, f32); 8] = [
            (0x0191, 25.0625),
            (0x00A2, 10.125),
            (0x0008, 0.5),
            (0x0000, 0.0),
            (0xFFF8, -0.5),
            (0xFF5E, -10.125),
            (0xFE6F, -25.0625),
            (0xFC90, -55.0),
        ];
        for (raw, expected) in raw_words {
            assert_eq!(raw_to_celsius(raw as i16), Ok(expected));
        }
    }

    #[test]
    fn rejectes_corrupt_readings() {
        assert_eq!(
            raw_to_celsius(0x7FFF),
            Err(SensorError::OutOfRange(2047.9375))
        )
    }
}
