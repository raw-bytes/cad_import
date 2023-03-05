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

/// The primitives defined by its indices.
pub struct Primitives {
    /// The primitive type of the index data
    primitive_type: PrimitiveType,

    /// The raw stored index data
    indices: Vec<u32>,
}

impl Primitives {
    /// Returns a new empty primitive data.
    /// 
    /// # Arguments
    /// * `indices` - The indices to use.
    /// * `primitive_type` - The primitive type.
    pub fn new(indices: Vec<u32>, primitive_type: PrimitiveType) -> Result<Self, Error> {
        // check constraints
        match primitive_type {
            PrimitiveType::Line => {
                if indices.len() % 2 != 0 {
                    return Err(Error::Indices(format!("Lines primitive indices must be a multiple of 2")));
                }
            },
            PrimitiveType::LineStrip => {
                if indices.len() < 2 {
                    return Err(Error::Indices(format!("Line-strips indices must be at least 2")));
                }
            },
            PrimitiveType::LineLoop => {
                if indices.len() < 2 {
                    return Err(Error::Indices(format!("Line-loop indices must be at least 2")));
                }
            },
            PrimitiveType::Triangles => {
                if indices.len() % 3 != 0 {
                    return Err(Error::Indices(format!("Triangle indices must be a multiple of 3")));
                }
            },
            PrimitiveType::TriangleStrip => {
                if indices.len() < 3 {
                    return Err(Error::Indices(format!("Triangle-strip indices must be at least 3")));
                }
            },
            PrimitiveType::TriangleFan => {
                if indices.len() < 3 {
                    return Err(Error::Indices(format!("Triangle-fan indices must be at least 3")));
                }
            },
            _ => {},
        }
        
        Ok(Self {
            primitive_type,
            indices,
        })
    }

    /// Returns the number of primitives.
    pub fn num_primitives(&self) -> usize {
        match self.primitive_type {
            PrimitiveType::Point => self.indices.len(),
            PrimitiveType::Line => self.indices.len() / 2,
            PrimitiveType::LineStrip => self.indices.len() - 1,
            PrimitiveType::LineLoop => self.indices.len(),
            PrimitiveType::Triangles => self.indices.len() / 3,
            PrimitiveType::TriangleStrip => self.indices.len() - 2,
            PrimitiveType::TriangleFan => self.indices.len() - 2,
        }
    }

    /// Returns the primitive type.
    pub fn get_primitive_type(&self) -> PrimitiveType {
        self.primitive_type
    }

    /// Returns a slice on the raw indices.
    pub fn get_raw_index_data(&self) -> &[u32] {
        &self.indices
    }

    /// Returns the maximal referenced vertex.
    pub fn max_index(&self) -> Option<u32> {
        match self.indices.iter().max() {
            Some(m) => Some(*m),
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_primitives() {
        let p = Primitives::new(vec![1,2], PrimitiveType::Point).unwrap();
        assert_eq!(p.num_primitives(), 2);

        let p = Primitives::new(vec![1,2,3,4], PrimitiveType::Line).unwrap();
        assert_eq!(p.num_primitives(), 2);

        let p = Primitives::new(vec![1,2,3,4], PrimitiveType::LineLoop).unwrap();
        assert_eq!(p.num_primitives(), 4);

        let p = Primitives::new(vec![1,2,3,4], PrimitiveType::LineStrip).unwrap();
        assert_eq!(p.num_primitives(), 3);

        let p = Primitives::new(vec![1,2,3,4,5,6], PrimitiveType::Triangles).unwrap();
        assert_eq!(p.num_primitives(), 2);

        let p = Primitives::new(vec![1,2,3,4,5,6], PrimitiveType::TriangleStrip).unwrap();
        assert_eq!(p.num_primitives(), 4);

        let p = Primitives::new(vec![1,2,3,4,5,6], PrimitiveType::TriangleFan).unwrap();
        assert_eq!(p.num_primitives(), 4);
    }
}