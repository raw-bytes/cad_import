use crate::{structure::Mesh, Error};

pub trait Tessellate {
    fn tessellate() -> Result<Mesh, Error>;
}
