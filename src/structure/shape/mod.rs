mod component;
mod mesh;
mod primitives;
mod vertices;

mod material;
mod shape;

pub use component::{Component, Float, Normal, Point3D};
pub use material::{Material, PhongMaterialData};
pub use mesh::Mesh;
pub use primitives::{PrimitiveType, Primitives, IndexData};
pub use shape::{Shape, ShapePart};
pub use vertices::{Colors, Normals, Positions, Vertices};
