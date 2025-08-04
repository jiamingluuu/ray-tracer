struct Pixel([f32; 3]);

pub struct Film {
    pub(crate) resolution: (usize, usize),
    pub(crate) filename: String,
    pub(crate) pixels: Vec<Pixel>,
}

impl Film {
    pub fn new(resolution: (usize, usize), filename: String) -> Self {
        let pixels = Vec::with_capacity(resolution.0 * resolution.1);
        Self {
            resolution,
            filename,
            pixels
        }
    }
}