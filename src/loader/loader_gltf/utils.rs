use gltf::accessor::DataType;

/// Returns size in bytes for the given data type.
///
/// # Arguments
/// * `data_type` - The data type whose size will be returned.
pub fn get_size_in_bytes(data_type: DataType) -> usize {
    match data_type {
        DataType::I8 => 1,
        DataType::U8 => 1,
        DataType::I16 => 2,
        DataType::U16 => 2,
        DataType::U32 => 4,
        DataType::F32 => 4,
    }
}

/// Transmutes the given vector of type U to vector of type V. However, this should only be done
/// to primitive types U and V. Moreover, U and V must be of same size.
///
/// # Arguments
/// * `vec` - The vector to transmute.
pub fn transmute_vec<U: Sized, V: Sized>(vec: Vec<U>) -> Vec<V> {
    assert_eq!(std::mem::size_of::<U>(), std::mem::size_of::<V>());

    unsafe {
        let mut v_clone = std::mem::ManuallyDrop::new(vec);
        Vec::from_raw_parts(
            v_clone.as_mut_ptr() as *mut V,
            v_clone.len(),
            v_clone.capacity(),
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_transmute() {
        let values = vec![0u32, 13u32, 52u32];
        let values: Vec<i32> = transmute_vec(values);

        assert_eq!(values, [0i32, 13i32, 52i32]);
    }
}
