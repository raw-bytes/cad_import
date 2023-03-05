#[derive(Clone, Copy, Default, PartialEq, Debug)]
pub struct RGB {
    values: [f32; 3],
}

impl RGB {
    /// Returns a new RGB object with the given red, green and blue value.
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Self {
            values: [r,g,b]
        }
    }

    /// Returns the color black
    pub fn black() -> Self {
        Self::new(0f32, 0f32, 0f32)
    }
}