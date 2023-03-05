use super::component::{Point3D, Normal};

pub type Positions = Vec<Point3D>;
pub type Normals = Vec<Normal>;

/// Vertices contains a vertex list. A vertex is a position in space with additional optional
/// attributes like normals, color, ... etc.
pub struct Vertices {
    positions: Positions,
    normals: Option<Normals>,
}

impl Vertices {
    /// Returns a new empty set of vertices with only positions attribute.
    pub fn new() -> Self {
        Vertices{
            positions: Vec::new(),
            normals: None,
        }
    }

    /// Creates a new vertices object initialized with positions attribute.
    ///
    /// # Argument
    /// * `positions` - The positions to set
    pub fn from_positions(positions: Vec<Point3D>) -> Self {
        Vertices { positions, normals: None }
    }

    /// Returns the number of vertices.
    pub fn len(&self) -> usize {
        self.positions.len()
    }

    /// Returns a reference onto the positions attribute.
    pub fn get_positions(&self) -> &Positions {
        &self.positions
    }

    /// Returns a reference onto the normals attribute.
    pub fn get_normals(&self) -> Option<&Normals> {
        self.normals.as_ref()
    }
}