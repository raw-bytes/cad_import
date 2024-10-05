use std::{
    fmt::{Debug, Display},
    ops,
};

/// A unit for length, i.e., a unit could be meter or inch
#[derive(Clone, Copy, PartialEq)]
pub struct Length {
    /// The length unit expressed in meters
    in_meters: f64,
}

impl Length {
    pub const MILLIMETER: Length = Length { in_meters: 1e-3f64 };
    pub const CENTIMETER: Length = Length { in_meters: 1e-2f64 };
    pub const METER: Length = Length { in_meters: 1f64 };
    pub const KILOMETER: Length = Length { in_meters: 1e3f64 };

    pub const INCH: Length = Length {
        in_meters: 0.02539999f64,
    };
    pub const FEET: Length = Length {
        in_meters: 0.3048f64,
    };
    pub const MILE: Length = Length {
        in_meters: 1609.344f64,
    };

    /// Returns a new unit based on the provided length in meters
    ///
    /// # Arguments
    /// * `in_meters` - The new unit defined in meters.
    pub fn new(in_meters: f64) -> Self {
        Self { in_meters }
    }

    /// Returns the unit in meters.
    #[inline]
    pub fn get_unit_in_meters(&self) -> f64 {
        self.in_meters
    }

    /// Returns the unit in millimeters.
    #[inline]
    pub fn get_unit_in_millimeters(&self) -> f64 {
        self.in_meters * 1e3f64
    }
}

impl Debug for Length {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "unit={} meter", self.in_meters)
    }
}

impl Display for Length {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.in_meters)
    }
}

impl ops::Mul<f64> for Length {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            in_meters: self.in_meters * rhs,
        }
    }
}

impl ops::Div<f64> for Length {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self {
            in_meters: self.in_meters / rhs,
        }
    }
}

/// A unit for angle, i.e., the unit could be in radians or degrees
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Angle {
    /// The length unit expressed in meters
    in_radians: f64,
}

impl Angle {
    /// Returns a new unit based on the provided angle in radians.
    ///
    /// # Arguments
    /// * `in_radians` - The new unit defined in radians.
    pub fn new(in_radians: f64) -> Self {
        Self { in_radians }
    }

    /// Returns the unit in radians.
    #[inline]
    pub fn get_unit_in_radians(&self) -> f64 {
        self.in_radians
    }

    /// Returns the unit in degrees.
    #[inline]
    pub fn get_unit_in_degrees(&self) -> f64 {
        self.in_radians.to_degrees()
    }

    /// Returns the unit in gradians.
    pub fn get_unit_in_gradians(&self) -> f64 {
        self.in_radians.to_degrees() * 10f64 / 9f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_provided_units() {
        assert_eq!(Length::MILLIMETER * 1000f64, Length::METER);
        assert_eq!(Length::CENTIMETER * 100f64, Length::METER);
        assert_eq!(Length::METER.get_unit_in_meters(), 1f64);
        assert_eq!(Length::METER * 1000f64, Length::KILOMETER);

        assert!(((Length::INCH * 39.37f64).get_unit_in_meters() - 1f64).abs() <= 1e-4f64);
        assert!(
            ((Length::INCH * 12f64).get_unit_in_meters() - Length::FEET.get_unit_in_meters()).abs()
                <= 1e-4f64
        );
        assert_eq!(Length::FEET * 5280f64, Length::MILE);
    }

    #[test]
    fn test_angle_units() {
        let angle = Angle::new(std::f64::consts::PI);
        assert_eq!(angle.get_unit_in_radians(), std::f64::consts::PI);
        assert_eq!(angle.get_unit_in_degrees(), 180f64);
        assert_eq!(angle.get_unit_in_gradians(), 200f64);
    }
}
