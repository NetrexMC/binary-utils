use core::fmt;

macro_rules! impl_type {
    (
        $ty: ty, $for_ty: ty
    ) => {
        impl From<$ty> for $for_ty {
            fn from(val: $ty) -> Self {
                val.0
            }
        }

        impl From<$for_ty> for $ty {
            fn from(val: $for_ty) -> Self {
                Self::new(val)
            }
        }

        impl std::ops::Deref for $ty {
            type Target = $for_ty;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl std::ops::DerefMut for $ty {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
    };
}

/// Little Endian (LE) wrapper type
/// This type is used to indicate that the value is in little endian format
/// It's primary use is in deriving from `BinaryIo` trait
///
/// # Example
/// ```rust ignore
/// use binary_util::types::LE;
/// use binary_util::BinaryIo;
///
/// #[derive(BinaryIo)]
/// struct MyStruct {
///    test: LE<u32>,
/// }
/// ```
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LE<T>(pub T);

impl<T> LE<T> {
    pub fn new(val: T) -> Self {
        Self(val)
    }
}
impl_type!(LE<u16>, u16);
impl_type!(LE<u24>, u24);
impl_type!(LE<u32>, u32);
impl_type!(LE<u64>, u64);
impl_type!(LE<u128>, u128);
impl_type!(LE<i16>, i16);
impl_type!(LE<i24>, i24);
impl_type!(LE<i32>, i32);
impl_type!(LE<i64>, i64);
impl_type!(LE<i128>, i128);
impl_type!(LE<f32>, f32);
impl_type!(LE<f64>, f64);

/// Big Endian (BE) wrapper type
/// This type is used to indicate that the value is in big endian format
/// It's primary use is in deriving from `BinaryIo` trait
///
/// # Example
/// ```rust ignore
/// use binary_util::types::BE;
/// use binary_util::BinaryIo;
///
/// #[derive(BinaryIo)]
/// struct MyStruct {
///   test: BE<u32>,
/// }
/// ```
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BE<T>(pub T);

impl<T> BE<T> {
    pub fn new(val: T) -> Self {
        Self(val)
    }
}

impl_type!(BE<u16>, u16);
impl_type!(BE<u24>, u24);
impl_type!(BE<u32>, u32);
impl_type!(BE<u64>, u64);
impl_type!(BE<u128>, u128);
impl_type!(BE<i16>, i16);
impl_type!(BE<i24>, i24);
impl_type!(BE<i32>, i32);
impl_type!(BE<i64>, i64);
impl_type!(BE<i128>, i128);
impl_type!(BE<f32>, f32);
impl_type!(BE<f64>, f64);

/// Unsigned 24 bit integer explicit type.
/// You should really only use this when you need to derive the `BinaryIo` trait
/// as it is a helper type.
///
/// # Example
/// ```rust ignore
/// use binary_util::types::u24;
/// use binary_util::BinaryIo;
///
/// #[derive(BinaryIo)]
/// struct MyStruct {
///    test: u24,
/// }
/// ```
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct u24(pub u32);

impl u24 {
    pub fn new(val: u32) -> Self {
        if val <= 0xFFFFFF {
            Self(val)
        } else {
            panic!("u24: value out of range")
        }
    }
}

impl fmt::Display for u24 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl_type!(u24, u32);

/// Signed 24 bit integer explicit type.
/// You should really only use this when you need to derive the `BinaryIo` trait
/// as it is a helper type.
///
/// # Example
/// ```rust ignore
/// use binary_util::types::i24;
/// use binary_util::BinaryIo;
///
/// #[derive(BinaryIo)]
/// struct MyStruct {
///   test: i24,
/// }
/// ```
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct i24(pub i32);

impl i24 {
    pub fn new(val: i32) -> Self {
        if val >= -0x800000 && val <= 0x7FFFFF {
            Self(val)
        } else {
            panic!("i24: value out of range")
        }
    }
}

impl fmt::Display for i24 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl_type!(i24, i32);

/// A variable length integer type that can be up to 32 bits.
/// This is a helper type for deriving the `BinaryIo` trait.
///
/// You should not use this type directly, if you are reading or writing
/// a variable length integer, use the `ByteWriter` or `ByteReader` and use
/// the corresponding `read_var_u32` or `write_var_u32` methods.
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct varu32(pub u32);

impl varu32 {
    pub fn new(val: u32) -> Self {
        Self(val)
    }
}
impl_type!(varu32, u32);

/// A variable length integer type that can be up to 32 bits.
/// This is a helper type for deriving the `BinaryIo` trait.
///
/// You should not use this type directly, if you are reading or writing
/// a variable length integer, use the `ByteWriter` or `ByteReader` and use
/// the corresponding `read_var_i32` or `write_var_i32` methods.
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct vari32(pub i32);

impl vari32 {
    pub fn new(val: i32) -> Self {
        Self(val)
    }
}
impl_type!(vari32, i32);

/// A variable length integer type that can be up to 64 bits.
/// This is a helper type for deriving the `BinaryIo` trait.
///
/// > You should not use this type directly, if you are reading or writing
/// > a variable length integer, use the `ByteWriter` or `ByteReader` and use
/// > the corresponding `read_var_u64` or `write_var_u64` methods.
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct varu64(pub u64);

impl varu64 {
    pub fn new(val: u64) -> Self {
        Self(val)
    }
}

impl_type!(varu64, u64);

/// A variable length integer type that can be up to 64 bits.
/// This is a helper type for deriving the `BinaryIo` trait.
///
/// > You should not use this type directly, if you are reading or writing
/// > a variable length integer, use the `ByteWriter` or `ByteReader` and use
/// > the corresponding `read_var_i64` or `write_var_i64` methods.
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct vari64(pub i64);

impl vari64 {
    pub fn new(val: i64) -> Self {
        Self(val)
    }
}

impl_type!(vari64, i64);
