use std::fmt::Debug;

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
#[derive(Clone, Copy, Default, Debug)]
pub struct Point3D {
    pub v: [f32; 3],
}

impl PartialEq for Point3D {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.v[0] == other.v[0] && self.v[1] == other.v[1] && self.v[2] == other.v[2]
    }
}

impl Component for Point3D {
    #[inline]
    fn interpolate(&self, rhs: &Self, f: f32) -> Self {
        Self {
            v: [
                self.v[0] * (1f32 - f) + f * rhs.v[0],
                self.v[1] * (1f32 - f) + f * rhs.v[1],
                self.v[2] * (1f32 - f) + f * rhs.v[2],
            ],
        }
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
        let a: Point3D = Point3D {
            v: [-1f32, -4f32, -8f32],
        };
        let b: Point3D = Point3D {
            v: [1f32, 4f32, 8f32],
        };

        assert_eq!(a.interpolate(&b, 0f32), a);
        assert_eq!(a.interpolate(&b, 1f32), b);
        assert_eq!(
            a.interpolate(&b, 0.5f32),
            Point3D {
                v: [0f32, 0f32, 0f32]
            }
        );
    }
}
