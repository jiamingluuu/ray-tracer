use std::path::Path;

use super::colour::Colour;

pub struct Image {
    dimension: (usize, usize),
    pixels: Vec<Colour>
}

impl Image {
    pub fn new(sx: usize, sy: usize) -> Self {
        Self {
            dimension: (sx, sy),
            pixels: Vec::new()
        }
    }

    pub fn save_to_file(self, path: &Path) {
        todo!()
    }
}
