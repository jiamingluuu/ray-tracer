#[derive(Clone, Debug, Default)]
pub struct Colour {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Colour {
    pub fn new() -> Self {
        Self {
            r: 0.0,
            g: 0.0,
            b: 0.0,
        }
    }

    pub fn validate_colour(&self) -> bool {
        0.0 <= self.r && self.r < 1.0
          && 0.0 <= self.g && self.g < 1.0
          && 0.0 <= self.b && self.b < 1.0
    }

    pub fn to_bytes(&self) -> [u8; 3] {
        debug_assert!(self.validate_colour());
        [
            (self.r * 256.0) as u8,
            (self.g * 256.0) as u8,
            (self.b * 256.0) as u8,
        ]
    }
}
