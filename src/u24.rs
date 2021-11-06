#![allow(non_camel_case_types)]

use byteorder::ReadBytesExt;
use std::cmp::{Ordering, PartialEq, PartialOrd};
use std::convert::{From, Into};
use std::io;
use std::ops::{Add, BitOr, Div, Mul, Sub};

use crate::error::BinaryError;
use crate::Streamable;
/// Base Implementation for a u24
/// A u24 is 3 bytes (24 bits) wide number.
#[derive(Clone, Copy, Debug)]
pub struct u24(u32); // inner is validated

impl u24 {
    pub fn is_u24(num: usize) -> bool {
        num < 0x00FF_FFFF
    }

    pub fn from_be_bytes(bytes: &[u8]) -> Self {
        u32::from_be_bytes([bytes[0], bytes[1], bytes[2], 0]).into()
    }

    pub fn from_le_bytes(bytes: &[u8]) -> Self {
        u32::from_be_bytes([0, bytes[1], bytes[2], bytes[3]]).into()
    }

    pub fn to_le_bytes(self) -> [u8; 3] {
        let bytes = self.0.to_le_bytes();
        [bytes[0], bytes[1], bytes[2]]
    }

    pub fn to_be_bytes(self) -> [u8; 3] {
        let bytes = self.0.to_be_bytes();
        [bytes[0], bytes[1], bytes[2]]
    }
}

impl Streamable for u24 {
    /// Writes `self` to the given buffer.
    fn parse(&self) -> Result<Vec<u8>, BinaryError> {
        Ok(self.to_be_bytes().to_vec().clone())
    }
    /// Reads `self` from the given buffer.
    fn compose(source: &[u8], position: &mut usize) -> Result<Self, BinaryError> {
        *position += 2;
        Ok(Self::from_be_bytes(source))
    }
}

pub trait u24Writer: io::Write {
    #[inline]
    fn write_u24(&mut self, num: u24) -> io::Result<usize> {
        self.write(&num.to_be_bytes())
    }
}

pub trait u24Reader: io::Read {
    #[inline]
    fn read_u24(&mut self) -> io::Result<u24> {
        let initial = [self.read_u8()?, self.read_u8()?, self.read_u8()?];
        Ok(u24::from_be_bytes(&initial))
    }
}

impl Add<u24> for u24 {
    type Output = Self;

    fn add(self, other: u24) -> Self::Output {
        u24(self.0 + other.0)
    }
}

impl Mul<u24> for u24 {
    type Output = Self;

    fn mul(self, other: u24) -> Self::Output {
        u24(self.0 * other.0)
    }
}

impl Sub<u24> for u24 {
    type Output = Self;

    fn sub(self, other: u24) -> Self::Output {
        u24(self.0 - other.0)
    }
}

impl Div<u24> for u24 {
    type Output = Self;

    fn div(self, other: u24) -> Self::Output {
        u24(self.0 / other.0)
    }
}

impl PartialEq for u24 {
    fn eq(&self, other: &u24) -> bool {
        self.0 == other.0
    }
}

impl PartialOrd for u24 {
    fn partial_cmp(&self, other: &u24) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

macro_rules! impl_primitive_u24 {
    ($ty:ty) => {
        impl From<$ty> for u24 {
            fn from(value: $ty) -> Self {
                if !u24::is_u24(value as usize) {
                    panic!("Can not convert a number larger than the bounds of a u24 into a u24")
                } else {
                    u24(value as u32)
                }
            }
        }

        impl BitOr<$ty> for u24 {
            type Output = Self;

            fn bitor(self, rhs: $ty) -> Self::Output {
                u24(self.0 | rhs as u32)
            }
        }

        impl Into<$ty> for u24 {
            fn into(self) -> $ty {
                self.0 as $ty
            }
        }

        impl Add<$ty> for u24 {
            type Output = Self;

            fn add(self, other: $ty) -> Self::Output {
                u24(self.0 + other as u32)
            }
        }

        impl Mul<$ty> for u24 {
            type Output = Self;

            fn mul(self, other: $ty) -> Self::Output {
                u24(self.0 * other as u32)
            }
        }

        impl Sub<$ty> for u24 {
            type Output = Self;

            fn sub(self, other: $ty) -> Self::Output {
                u24(self.0 - other as u32)
            }
        }

        impl Div<$ty> for u24 {
            type Output = Self;

            fn div(self, other: $ty) -> Self::Output {
                u24(self.0 / other as u32)
            }
        }
    };
}

impl_primitive_u24!(u8);
impl_primitive_u24!(u16);
impl_primitive_u24!(u32);
impl_primitive_u24!(u64);
impl_primitive_u24!(f32);
impl_primitive_u24!(f64);
impl_primitive_u24!(u128);
impl_primitive_u24!(i8);
impl_primitive_u24!(i16);
impl_primitive_u24!(i32);
impl_primitive_u24!(i64);
impl_primitive_u24!(i128);
