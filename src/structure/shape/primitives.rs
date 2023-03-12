use crate::error::Error;

/// The underlying basic primitive type.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum PrimitiveType {
    Point = 0,
    Line = 1,
    LineStrip = 2,
    LineLoop = 3,
    Triangles = 4,
    TriangleStrip = 5,
    TriangleFan = 6,
}

/// The index data defines how the primitives are indexed.
#[derive(Clone)]
pub enum IndexData {
    /// Non-indexed primitives are defined by the natural order of their vertices.
    /// This is for example the usual pattern for point data.
    NonIndexed(usize),

    /// The raw 32-bit indices.
    Indices(Vec<u32>),
}

impl IndexData {
    /// Returns the number of indices.
    pub fn num_indices(&self) -> usize {
        match self {
            IndexData::NonIndexed(n) => *n,
            IndexData::Indices(indices) => indices.len(),
        }
    }

    /// Returns a reference onto the indices.
    pub fn get_indices_ref(&self) -> Option<&[u32]> {
        match self {
            IndexData::Indices(indices) => Some(indices),
            _ => None,
        }
    }
}

/// The primitives defined by its indices.
pub struct Primitives {
    /// The primitive type of the index data
    primitive_type: PrimitiveType,

    /// The raw stored index data
    index_data: IndexData,
}

impl Primitives {
    /// Returns a new empty primitive data.
    /// 
    /// # Arguments
    /// * `index_data` - The index data that define how vertices are combined to primitives.
    /// * `primitive_type` - The primitive type.
    pub fn new(index_data: IndexData, primitive_type: PrimitiveType) -> Result<Self, Error> {
        let num_indices = index_data.num_indices();

        // check constraints
        match primitive_type {
            PrimitiveType::Line => {
                if num_indices % 2 != 0 {
                    return Err(Error::Indices(format!("Lines primitive indices must be a multiple of 2")));
                }
            },
            PrimitiveType::LineStrip => {
                if num_indices < 2 {
                    return Err(Error::Indices(format!("Line-strips indices must be at least 2")));
                }
            },
            PrimitiveType::LineLoop => {
                if num_indices < 2 {
                    return Err(Error::Indices(format!("Line-loop indices must be at least 2")));
                }
            },
            PrimitiveType::Triangles => {
                if num_indices % 3 != 0 {
                    return Err(Error::Indices(format!("Triangle indices must be a multiple of 3")));
                }
            },
            PrimitiveType::TriangleStrip => {
                if num_indices < 3 {
                    return Err(Error::Indices(format!("Triangle-strip indices must be at least 3")));
                }
            },
            PrimitiveType::TriangleFan => {
                if num_indices < 3 {
                    return Err(Error::Indices(format!("Triangle-fan indices must be at least 3")));
                }
            },
            _ => {},
        }
        
        Ok(Self {
            primitive_type,
            index_data,
        })
    }

    /// Returns the number of primitives.
    pub fn num_primitives(&self) -> usize {
        let num_indices = self.index_data.num_indices();
        match self.primitive_type {
            PrimitiveType::Point =>num_indices,
            PrimitiveType::Line =>num_indices / 2,
            PrimitiveType::LineStrip => if num_indices == 0 { 0 } else { num_indices - 1 },
            PrimitiveType::LineLoop =>num_indices,
            PrimitiveType::Triangles =>num_indices / 3,
            PrimitiveType::TriangleStrip => if num_indices >= 2 { num_indices - 2 } else { 0 },
            PrimitiveType::TriangleFan => if num_indices >= 2 { num_indices - 2 } else { 0 },
        }
    }

    /// Returns the primitive type.
    pub fn get_primitive_type(&self) -> PrimitiveType {
        self.primitive_type
    }

    /// Returns a slice on the raw indices.
    pub fn get_raw_index_data(&self) -> &IndexData {
        &self.index_data
    }

    /// Returns the maximal referenced vertex.
    pub fn max_index(&self) -> Option<u32> {
        match &self.index_data {
            IndexData::Indices(indices) => {
                match indices.iter().max() {
                    Some(m) => Some(*m),
                    None => None,
                }
            }
            IndexData::NonIndexed(n) => if *n == 0 { None } else { Some((*n  - 1) as u32) }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_primitives() {
        let p = Primitives::new(IndexData::Indices(vec![1,2]), PrimitiveType::Point).unwrap();
        assert_eq!(p.num_primitives(), 2);

        let p = Primitives::new(IndexData::Indices(vec![1,2,3,4]), PrimitiveType::Line).unwrap();
        assert_eq!(p.num_primitives(), 2);

        let p = Primitives::new(IndexData::Indices(vec![1,2,3,4]), PrimitiveType::LineLoop).unwrap();
        assert_eq!(p.num_primitives(), 4);

        let p = Primitives::new(IndexData::Indices(vec![1,2,3,4]), PrimitiveType::LineStrip).unwrap();
        assert_eq!(p.num_primitives(), 3);

        let p = Primitives::new(IndexData::Indices(vec![1,2,3,4,5,6]), PrimitiveType::Triangles).unwrap();
        assert_eq!(p.num_primitives(), 2);

        let p = Primitives::new(IndexData::Indices(vec![1,2,3,4,5,6]), PrimitiveType::TriangleStrip).unwrap();
        assert_eq!(p.num_primitives(), 4);

        let p = Primitives::new(IndexData::Indices(vec![1,2,3,4,5,6]), PrimitiveType::TriangleFan).unwrap();
        assert_eq!(p.num_primitives(), 4);
    }
}