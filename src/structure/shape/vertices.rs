use crate::{basic_types::RGBA, error::Error};

use super::component::{Normal, Point3D};

pub type Positions = Vec<Point3D>;
pub type Normals = Vec<Normal>;
pub type Colors = Vec<RGBA>;

/// Vertices contains a vertex list. A vertex is a position in space with additional optional
/// attributes like normals, color, ... etc.
pub struct Vertices {
    positions: Positions,
    normals: Option<Normals>,
    colors: Option<Colors>,
}

impl Vertices {
    /// Returns a new empty set of vertices with only positions attribute.
    pub fn new() -> Self {
        Vertices {
            positions: Vec::new(),
            normals: None,
            colors: None,
        }
    }

    /// Creates a new vertices object initialized with positions attribute.
    ///
    /// # Argument
    /// * `positions` - The positions to set
    pub fn from_positions(positions: Vec<Point3D>) -> Self {
        Vertices {
            positions,
            normals: None,
            colors: None,
        }
    }

    /// Returns the number of vertices.
    pub fn len(&self) -> usize {
        self.positions.len()
    }

    /// Sets the normal attribute. If the number of normals does not match the number
    /// of vertices, an error is returned.
    ///
    /// # Arguments
    /// * `colors` - The color attribute to set.
    pub fn set_normals(&mut self, normals: Normals) -> Result<(), Error> {
        if self.positions.len() != normals.len() {
            Err(Error::InvalidArgument(format!(
                "Got {} vertices, but normal attribute only has {} entries",
                self.positions.len(),
                normals.len()
            )))
        } else {
            self.normals = Some(normals);
            Ok(())
        }
    }

    /// Sets the color attribute. If the number of colors does not match the number
    /// of vertices, an error is returned.
    ///
    /// # Arguments
    /// * `colors` - The color attribute to set.
    pub fn set_colors(&mut self, colors: Colors) -> Result<(), Error> {
        if self.positions.len() != colors.len() {
            Err(Error::InvalidArgument(format!(
                "Got {} vertices, but color attribute only has {} entries",
                self.positions.len(),
                colors.len()
            )))
        } else {
            self.colors = Some(colors);
            Ok(())
        }
    }

    /// Returns a reference onto the positions attribute.
    pub fn get_positions(&self) -> &Positions {
        &self.positions
    }

    /// Returns a reference onto the normals attribute.
    pub fn get_normals(&self) -> Option<&Normals> {
        self.normals.as_ref()
    }

    /// Returns a reference onto the colors attribute.
    pub fn get_colors(&self) -> Option<&Colors> {
        self.colors.as_ref()
    }
}
