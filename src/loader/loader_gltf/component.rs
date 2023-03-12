pub trait ComponentTrait: Copy + Sized {
    fn to_f32(&self, normalize: bool) -> f32;
}

impl ComponentTrait for i8 {
    fn to_f32(&self, normalize: bool) -> f32 {
        if normalize {
            ((*self as f32) + 0.5f32) / 127.5f32
        } else {
            *self as f32
        }
    }
}

impl ComponentTrait for u8 {
    fn to_f32(&self, normalize: bool) -> f32 {
        if normalize {
            (*self as f32) / 255f32
        } else {
            *self as f32
        }
    }
}

impl ComponentTrait for i16 {
    fn to_f32(&self, normalize: bool) -> f32 {
        if normalize {
            ((*self as f32) + 0.5f32) / 32767.5f32
        } else {
            *self as f32
        }
    }
}

impl ComponentTrait for u16 {
    fn to_f32(&self, normalize: bool) -> f32 {
        if normalize {
            (*self as f32) / 65535f32
        } else {
            *self as f32
        }
    }
}

impl ComponentTrait for u32 {
    fn to_f32(&self, normalize: bool) -> f32 {
        if normalize {
            (*self as f32) / 4294967295f32
        } else {
            *self as f32
        }
    }
}

impl ComponentTrait for f32 {
    fn to_f32(&self, _normalize: bool) -> f32 {
        *self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalization() {
        assert_eq!(i8::MIN.to_f32(true), -1f32);
        assert_eq!(i8::MAX.to_f32(true), 1f32);

        assert_eq!(u8::MIN.to_f32(true), 0f32);
        assert_eq!(u8::MAX.to_f32(true), 1f32);

        assert_eq!(i16::MIN.to_f32(true), -1f32);
        assert_eq!(i16::MAX.to_f32(true), 1f32);

        assert_eq!(u16::MIN.to_f32(true), 0f32);
        assert_eq!(u16::MAX.to_f32(true), 1f32);

        assert_eq!(u32::MIN.to_f32(true), 0f32);
        assert_eq!(u32::MAX.to_f32(true), 1f32);

        assert_eq!(0f32.to_f32(true), 0f32);
        assert_eq!(1f32.to_f32(true), 1f32);
    }
}
