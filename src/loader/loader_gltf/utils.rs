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
