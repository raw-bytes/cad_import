use std::rc::Rc;

use crate::basic_types::{IDCounter, ID};

use super::{material::Material, mesh::Mesh};

static ID_COUNTER: IDCounter = IDCounter::new();

/// A part of shape, i.e., a mesh and a material reference
pub struct ShapePart {
    mesh: Rc<Mesh>,
    material: Rc<Material>,
}

impl ShapePart {
    pub fn new(mesh: Rc<Mesh>, material: Rc<Material>) -> Self {
        Self { mesh, material }
    }

    /// Returns the internal mesh reference.
    pub fn get_mesh(&self) -> Rc<Mesh> {
        self.mesh.clone()
    }

    /// Returns the internal material reference.
    pub fn get_material(&self) -> Rc<Material> {
        self.material.clone()
    }
}

/// A shape is the geometric and visual description of an object. A object is the instantiation
/// of a shape.
pub struct Shape {
    /// The unique id of the shape
    id: ID,

    /// The parts of the shape. Each part is a mesh reference with a material assigned.
    parts: Vec<ShapePart>,
}

impl Shape {
    /// Returns a new empty shape
    pub fn new() -> Self {
        let id = ID_COUNTER.gen();

        Self {
            id,
            parts: Vec::new(),
        }
    }

    /// Returns the id of the shape
    #[inline]
    pub fn get_id(&self) -> ID {
        self.id
    }

    /// Adds a part to the shape.
    ///
    /// # Arguments
    /// * `part` - The part to add.
    pub fn add_part(&mut self, part: ShapePart) {
        self.parts.push(part);
    }

    /// Returns a reference onto the parts of the shapes.
    pub fn get_parts(&self) -> &[ShapePart] {
        &self.parts
    }
}

impl PartialEq for Shape {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Shape {}
