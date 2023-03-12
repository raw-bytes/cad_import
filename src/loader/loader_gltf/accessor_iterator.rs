use std::{
    marker::PhantomData,
    mem::{size_of, MaybeUninit},
};

use gltf::{
    accessor::{DataType, Dimensions},
    buffer::View,
    Accessor,
};

use super::utils::get_size_in_bytes;

pub struct AccessorIterator<'a, Element: Copy> {
    element: PhantomData<Element>,
    buffer: &'a [u8],
    offset: usize,
    stride: usize,
    count: usize,
    index: usize,
}

impl<'a, Element: Copy> AccessorIterator<'a, Element> {
    pub fn new(buffer: &'a [u8], buffer_view: View, accessor: Accessor) -> Self {
        // determine stride
        let stride = match buffer_view.stride() {
            Some(stride) => stride,
            None => 0,
        };

        Self::new_detail(
            buffer,
            accessor.count(),
            stride,
            accessor.data_type(),
            accessor.dimensions(),
            accessor.offset(),
            buffer_view.offset(),
        )
    }

    pub fn new_detail(
        buffer: &'a [u8],
        count: usize,
        buffer_stride: usize,
        data_type: DataType,
        dimension: Dimensions,
        accessor_offset: usize,
        buffer_offset: usize,
    ) -> Self {
        // determine stride
        let stride = if buffer_stride == 0 {
            get_size_in_bytes(data_type) * dimension.multiplicity()
        } else {
            buffer_stride
        };

        assert_eq!(
            std::mem::size_of::<Element>(),
            get_size_in_bytes(data_type) * dimension.multiplicity()
        );

        // determine offset
        let offset = buffer_offset + accessor_offset;

        Self {
            element: PhantomData::<Element> {},
            buffer,
            offset,
            stride,
            count: count,
            index: 0,
        }
    }

    fn current(&mut self) -> Element {
        let pos = self.offset + self.index * self.stride;

        debug_assert!(pos + size_of::<Element>() <= self.buffer.len());

        let ptr = &self.buffer[pos..(pos + size_of::<Element>())];

        let mut result = unsafe { [MaybeUninit::<Element>::uninit().assume_init()] };

        unsafe {
            std::slice::from_raw_parts_mut(result.as_mut_ptr().cast(), size_of::<Element>())
                .clone_from_slice(ptr);
        }

        result[0]
    }
}

impl<'a, Element: Copy> Iterator for AccessorIterator<'a, Element> {
    type Item = Element;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.count {
            return None;
        }

        let result = self.current();

        self.index += 1;

        Some(result)
    }
}

#[test]
fn test_iterator() {
    use byteorder::{NativeEndian, WriteBytesExt};
    use std::io::Cursor;

    let mut data: Vec<u8> = Vec::new();
    let mut c = Cursor::new(&mut data);
    {
        for i in 0..0x10000 {
            let value = i * 133;

            c.write_i8((value % 0x100 - 0x80) as i8).unwrap();
            c.write_u8((value % 0x100) as u8).unwrap();

            c.write_i16::<NativeEndian>((value % 0x10000 - 0x80000) as i16)
                .unwrap();
            c.write_u16::<NativeEndian>((value % 0x10000) as u16)
                .unwrap();

            c.write_u32::<NativeEndian>(value as u32).unwrap();
            c.write_f32::<NativeEndian>(value as f32).unwrap();
        }
    }

    let byte_stride = 2 + 4 + 8;
    assert_eq!(byte_stride * 0x10000, data.len());

    let it = AccessorIterator::<i8>::new_detail(
        &data,
        0x10000,
        byte_stride,
        DataType::I8,
        Dimensions::Scalar,
        0,
        0,
    );

    for (i, e) in it.enumerate() {
        let value = (i * 133) as i32;

        assert_eq!((value % 0x100 - 0x80) as i8, e);
    }

    // // test u8
    let it = AccessorIterator::<u8>::new_detail(
        &data,
        0x10000,
        byte_stride,
        DataType::U8,
        Dimensions::Scalar,
        1,
        0,
    );

    for (i, e) in it.enumerate() {
        let value = (i * 133) as i32;

        assert_eq!((value % 0x100) as u8, e);
    }

    let it = AccessorIterator::<i16>::new_detail(
        &data,
        0x10000,
        byte_stride,
        DataType::I16,
        Dimensions::Scalar,
        2,
        0,
    );

    for (i, e) in it.enumerate() {
        let value = (i * 133) as i32;

        assert_eq!((value % 0x10000 - 0x80000) as i16, e);
    }

    // // test u16
    let it = AccessorIterator::<u16>::new_detail(
        &data,
        0x10000,
        byte_stride,
        DataType::U16,
        Dimensions::Scalar,
        4,
        0,
    );

    for (i, e) in it.enumerate() {
        let value = (i * 133) as i32;

        assert_eq!((value % 0x10000) as u16, e);
    }

    // // test u32
    let it = AccessorIterator::<u32>::new_detail(
        &data,
        0x10000,
        byte_stride,
        DataType::U32,
        Dimensions::Scalar,
        6,
        0,
    );

    for (i, e) in it.enumerate() {
        let value = (i * 133) as i32;

        assert_eq!(value as u32, e);
    }

    // // test f32
    let it = AccessorIterator::<f32>::new_detail(
        &data,
        0x10000,
        byte_stride,
        DataType::F32,
        Dimensions::Scalar,
        10,
        0,
    );

    for (i, e) in it.enumerate() {
        let value = (i * 133) as i32;

        assert_eq!(value as f32, e);
    }
}
