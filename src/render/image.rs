use std::{fs::File, io::Write, path::Path};

use crate::render::colour::Colour;

pub struct Image {
    /// The resolution of the image, width x height.
    resolution: (usize, usize),
    /// Colour of each pixel.
    pixels: Vec<Colour>,
}

impl Image {
    pub fn new(resolution: (usize, usize)) -> Self {
        let pixels = Vec::with_capacity(resolution.0 * resolution.1);
        Self { resolution, pixels }
    }

    pub fn set_colour(&mut self, pixel: &(usize, usize), colour: Colour) {
        self.pixels[pixel.0 * self.resolution.0 + pixel.1] = colour;
    }

    pub fn save_to_file(self, path: &Path) -> std::io::Result<()> {
        let mut file = File::create(path)?;
        writeln!(file, "P6")?;
        writeln!(file, "{} {}", self.resolution.0, self.resolution.1)?;
        writeln!(file, "255")?;
        for pixel in self.pixels {
            file.write_all(&pixel.to_bytes())?;
        }
        Ok(())
    }
}
