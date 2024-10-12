use std::fmt::Debug;

use nalgebra_glm::Vec3;

/// The trait for components inside attributes.
pub trait Component: Sized + Default + Clone + Copy + PartialEq + Debug {
    /// Linear interpolation operator for the component. Interpolates between self and rhs.
    ///  f == 0 => returns self
    /// ]0,1[ => interpolates between self and rhs
    ///  f == 1 => returns rhs
    ///
    /// # Arguments
    /// * `rhs` - The right-hand-side component of the interpolation.
    /// * `f` - The interpolation factor between 0 and 1.
    fn interpolate(&self, rhs: &Self, f: f32) -> Self;
}

/// A single floating point
#[derive(Clone, Copy, Default, PartialEq, Debug)]
pub struct Float(f32);

impl Component for Float {
    #[inline]
    fn interpolate(&self, rhs: &Self, f: f32) -> Self {
        Float(self.0 * (1f32 - f) + f * rhs.0)
    }
}

/// A single point in 3D.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point3D(pub Vec3);

impl Point3D {
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self(Vec3::new(x, y, z))
    }
}

impl Default for Point3D {
    #[inline]
    fn default() -> Self {
        Self(Vec3::new(0f32, 0f32, 0f32))
    }
}

impl Component for Point3D {
    #[inline]
    fn interpolate(&self, rhs: &Self, f: f32) -> Self {
        Self(self.0 * (1f32 - f) + rhs.0 * f)
    }
}

/// A single normal.
pub type Normal = Point3D;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scalar_interpolate() {
        let a: Float = Float(-10f32);
        let b: Float = Float(10f32);

        assert_eq!(a.interpolate(&b, 0f32), a);
        assert_eq!(a.interpolate(&b, 1f32), b);
        assert_eq!(a.interpolate(&b, 0.5f32), Float(0f32));
    }

    #[test]
    fn test_point_interpolate() {
        let a: Point3D = Point3D::new(-1f32, -4f32, -8f32);
        let b: Point3D = Point3D::new(1f32, 4f32, 8f32);

        assert_eq!(a.interpolate(&b, 0f32), a);
        assert_eq!(a.interpolate(&b, 1f32), b);
        assert_eq!(a.interpolate(&b, 0.5f32), Point3D::new(0f32, 0f32, 0f32));
    }
}
