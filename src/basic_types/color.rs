use nalgebra_glm::{vec4_to_vec3, Vec3, Vec4};

use crate::structure::Component;

/// Trait for all color types
pub trait Color {
    /// Returns the color black
    fn black() -> Self;
}

/// Basic color type for RGB colors
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct RGB(pub Vec3);

impl RGB {
    /// Returns a new RGB based on the provided red, green and blue values.
    pub fn new(red: f32, green: f32, blue: f32) -> Self {
        Self(Vec3::new(red, green, blue))
    }
}

impl Default for RGB {
    fn default() -> Self {
        Self::black()
    }
}

impl Component for RGB {
    fn interpolate(&self, rhs: &Self, f: f32) -> Self {
        RGB(self.0 * (1f32 - f) + rhs.0 * f)
    }
}

impl Color for RGB {
    fn black() -> Self {
        RGB(Vec3::new(0f32, 0f32, 0f32))
    }
}

impl From<RGBA> for RGB {
    fn from(value: RGBA) -> Self {
        RGB(vec4_to_vec3(&value.0))
    }
}

/// Basic color type for RGBA colors
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct RGBA(pub Vec4);

impl RGBA {
    /// Returns a new RGB based on the provided red, green, blue and alpha values.
    pub fn new(red: f32, green: f32, blue: f32, alpha: f32) -> Self {
        Self(Vec4::new(red, green, blue, alpha))
    }
}

impl Component for RGBA {
    fn interpolate(&self, rhs: &Self, f: f32) -> Self {
        RGBA(self.0 * (1f32 - f) + rhs.0 * f)
    }
}

impl Default for RGBA {
    fn default() -> Self {
        Self::black()
    }
}

impl Color for RGBA {
    fn black() -> Self {
        RGBA(Vec4::new(0f32, 0f32, 0f32, 1f32))
    }
}

impl From<RGB> for RGBA {
    fn from(value: RGB) -> Self {
        RGBA(Vec4::new(value.0[0], value.0[1], value.0[2], 1f32))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rgb() {
        let black = RGB::black();
        assert_eq!(black.0[0], 0f32);
        assert_eq!(black.0[1], 0f32);
        assert_eq!(black.0[2], 0f32);

        let red = RGB::new(1f32, 0f32, 0f32);
        assert_eq!(red.0[0], 1f32);
        assert_eq!(red.0[1], 0f32);
        assert_eq!(red.0[2], 0f32);

        let green = RGB::new(0f32, 1f32, 0f32);
        assert_eq!(green.0[0], 0f32);
        assert_eq!(green.0[1], 1f32);
        assert_eq!(green.0[2], 0f32);

        let blue = RGB::new(0f32, 0f32, 1f32);
        assert_eq!(blue.0[0], 0f32);
        assert_eq!(blue.0[1], 0f32);
        assert_eq!(blue.0[2], 1f32);

        let rgb = RGB::new(0.2f32, 0.8f32, 0.5f32);
        assert_eq!(rgb.0[0], 0.2f32);
        assert_eq!(rgb.0[1], 0.8f32);
        assert_eq!(rgb.0[2], 0.5f32);

        let rgba: RGBA = RGBA::from(rgb);
        assert_eq!(rgba.0[0], 0.2f32);
        assert_eq!(rgba.0[1], 0.8f32);
        assert_eq!(rgba.0[2], 0.5f32);
        assert_eq!(rgba.0[3], 1f32);
    }

    #[test]
    fn test_rgba() {
        let black = RGBA::black();
        assert_eq!(black.0[0], 0f32);
        assert_eq!(black.0[1], 0f32);
        assert_eq!(black.0[2], 0f32);
        assert_eq!(black.0[3], 1f32);

        let red = RGBA::new(1f32, 0f32, 0f32, 1f32);
        assert_eq!(red.0[0], 1f32);
        assert_eq!(red.0[1], 0f32);
        assert_eq!(red.0[2], 0f32);
        assert_eq!(red.0[3], 1f32);

        let green = RGBA::new(0f32, 1f32, 0f32, 1f32);
        assert_eq!(green.0[0], 0f32);
        assert_eq!(green.0[1], 1f32);
        assert_eq!(green.0[2], 0f32);
        assert_eq!(green.0[3], 1f32);

        let blue = RGBA::new(0f32, 0f32, 1f32, 1f32);
        assert_eq!(blue.0[0], 0f32);
        assert_eq!(blue.0[1], 0f32);
        assert_eq!(blue.0[2], 1f32);
        assert_eq!(blue.0[3], 1f32);

        let transparent_color = RGBA::new(0.5f32, 0.2f32, 0.3f32, 0.8f32);
        assert_eq!(transparent_color.0[0], 0.5f32);
        assert_eq!(transparent_color.0[1], 0.2f32);
        assert_eq!(transparent_color.0[2], 0.3f32);
        assert_eq!(transparent_color.0[3], 0.8f32);

        let rgb: RGB = RGB::from(transparent_color);
        assert_eq!(rgb.0[0], 0.5f32);
        assert_eq!(rgb.0[1], 0.2f32);
        assert_eq!(rgb.0[2], 0.3f32);
    }
}
