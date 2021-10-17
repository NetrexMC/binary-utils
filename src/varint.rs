use crate::Streamable;
use byteorder::{ReadBytesExt, WriteBytesExt};
use std::convert::{From, Into};
use std::io::{self, Cursor};
use std::ops::{Add, BitOr, Div, Mul, Sub};
/// A minecraft specific unsized integer
/// A varint can be one of `32` and `64` bits
#[derive(Clone, Copy, Debug)]
pub struct VarInt<T>(pub T);

pub trait VarIntWriter<T>: io::Write {
    fn write_var_int(&mut self, num: VarInt<T>) -> io::Result<usize>;
}

pub trait VarIntReader<T>: io::Read {
    fn read_var_int(&mut self) -> io::Result<VarInt<T>>;
}

pub const VAR_INT_32_BYTE_MAX: usize = 5;
pub const VAR_INT_64_BYTE_MAX: usize = 10;

macro_rules! varint_impl_generic {
    ($ty:ty) => {
        impl VarInt<$ty> {
            /// Encodes the var_int into Big Endian Bytes
            pub fn to_be_bytes(self) -> Vec<u8> {
                self.to_bytes_be()
            }

            pub fn to_le_bytes(self) -> Vec<u8> {
                let mut v = Vec::<u8>::new();
                let bytes = self.to_bytes_be();
                for x in (0..bytes.len()).rev() {
                    v.push(*bytes.get(x).unwrap());
                }
                v
            }

            pub fn get_byte_length(self) -> u8 {
                self.to_be_bytes().len() as u8
            }

            fn to_bytes_be(self) -> Vec<u8> {
                let mut to_write: $ty = self.0;
                let mut buf: Vec<u8> = Vec::new();

                // while there is more than a single byte to write
                while to_write >= 0x80 {
                    // write at most a byte, to account for overflow
                    buf.write_u8(to_write as u8 | 0x80).unwrap();
                    to_write >>= 7;
                }

                buf.write_u8(to_write as u8).unwrap();
                buf
            }

            pub fn from_be_bytes_cursor(stream: &mut Cursor<Vec<u8>>) -> Self {
                 let mut value: $ty  = 0;

                 for x in (0..35).step_by(7) {
                    let byte = stream.read_u8().unwrap();
                    value |= (byte & 0x7f) as $ty << x;

                    // if the byte is a full length of a byte
                    // we can assume we are done
                    if byte & 0x80 == 0 {
                         break;
                    }
                 }

                 VarInt::<$ty>(value)
            }

            pub fn from_be_bytes(bstream: &[u8]) -> Self {
                let mut stream = Cursor::new(bstream);
                let mut value: $ty  = 0;

                for x in (0..35).step_by(7) {
                   let byte = stream.read_u8().unwrap();
                   value |= (byte & 0x7f) as $ty << x;

                   // if the byte is a full length of a byte
                   // we can assume we are done
                   if byte & 0x80 == 0 {
                        break;
                   }
                }

                VarInt::<$ty>(value)
           }

            //   pub fn from_le_bytes(bytes: &[u8]) -> Self {
            //       <$ty>::from_be_bytes([0, bytes[1], bytes[2], bytes[3]]).into()
            //   }
            pub fn is_var_int(_: $ty) -> bool {
                true
            }
        }

        impl Streamable for VarInt<$ty> {
            /// Writes `self` to the given buffer.
            fn parse(&self) -> Vec<u8> {
                self.to_be_bytes().to_vec().clone()
            }
            /// Reads `self` from the given buffer.
            fn compose(source: &[u8], position: &mut usize) -> Self {
               let v = Self::from_be_bytes(source);
               *position += v.get_byte_length() as usize;
               v
            }
        }

        impl VarIntReader<$ty> for dyn io::Read {
            #[inline]
            fn read_var_int(&mut self) -> io::Result<VarInt<$ty>> {
                let mut value: $ty  = 0;

                for x in (0..35).step_by(7) {
                   let byte = self.read_u8().unwrap();
                   value |= (byte & 0x7f) as $ty << x;

                   // if the byte is a full length of a byte
                   // we can assume we are done
                   if byte & 0x80 == 0 {
                        break;
                   }
                }

                Ok(VarInt::<$ty>(value))
            }
        }

        impl VarIntWriter<$ty> for dyn io::Write {
            #[inline]
            fn write_var_int(&mut self, num: VarInt<$ty>) -> io::Result<usize> {
                self.write_all(&num.to_be_bytes()[..]).unwrap();
                Ok(num.get_byte_length() as usize)
            }
        }
    };
}
macro_rules! varint_impl_generic64 {
    ($ty:ty) => {
        impl VarInt<$ty> {
            /// Encodes the var_int into Big Endian Bytes
            pub fn to_be_bytes(self) -> Vec<u8> {
                self.to_bytes_be()
            }

            pub fn to_le_bytes(self) -> Vec<u8> {
                let mut v = Vec::<u8>::new();
                let bytes = self.to_bytes_be();
                for x in (0..bytes.len()).rev() {
                    v.push(*bytes.get(x).unwrap());
                }
                v
            }

            pub fn get_byte_length(self) -> u8 {
                self.to_be_bytes().len() as u8
            }

            fn to_bytes_be(self) -> Vec<u8> {
                let mut to_write: $ty = self.0;
                let mut buf: Vec<u8> = Vec::new();

                // while there is more than a single byte to write
                while to_write >= 0x80 {
                    // write at most a byte, to account for overflow
                    buf.write_u8(to_write as u8 | 0x80).unwrap();
                    to_write >>= 7;
                }

                buf.write_u8(to_write as u8).unwrap();
                buf
            }

            pub fn from_be_bytes(stream: &mut Cursor<Vec<u8>>) -> Self {
               let mut value: $ty  = 0;

               for x in (0..70).step_by(7) {
                  let byte = stream.read_u8().unwrap();
                  value |= (byte & 0x7f) as $ty << x;

                  // if the byte is a full length of a byte
                  // we can assume we are done
                  if byte & 0x80 == 0 {
                       break;
                  }
               }

               VarInt::<$ty>(value)
          }

            //   pub fn from_be_bytes(bytes: &[u8]) -> Self {
            //       <$ty>::from_be_bytes([bytes[0], bytes[1], bytes[2], 0]).into()
            //   }

            //   pub fn from_le_bytes(bytes: &[u8]) -> Self {
            //       <$ty>::from_be_bytes([0, bytes[1], bytes[2], bytes[3]]).into()
            //   }
            pub fn is_var_int(_: $ty) -> bool {
                true
            }
        }

        impl Streamable for VarInt<$ty> {
            /// Writes `self` to the given buffer.
            fn parse(&self) -> Vec<u8> {
                self.to_be_bytes().to_vec().clone()
            }
            /// Reads `self` from the given buffer.
            fn compose(source: &[u8], position: &mut usize) -> Self {
               let v = Self::from_be_bytes(&mut Cursor::new(source.to_vec()));
               *position += v.get_byte_length() as usize;
               v
            }
        }
    };
}
varint_impl_generic!(u32);
varint_impl_generic!(i32);
varint_impl_generic64!(u64);
varint_impl_generic64!(i64);

macro_rules! impl_primitive_VarInt {
    ($ty:ty, $vk:ty) => {
        impl From<$ty> for VarInt<$vk> {
            fn from(value: $ty) -> Self {
                if !VarInt::<$vk>::is_var_int(value as $vk) {
                    panic!(
                        "Can not convert a number larger than the bounds of a VarInt into a VarInt"
                    )
                } else {
                    VarInt(value as $vk)
                }
            }
        }

        impl BitOr<$ty> for VarInt<$vk> {
            type Output = Self;

            fn bitor(self, rhs: $ty) -> Self::Output {
                VarInt(self.0 | rhs as $vk)
            }
        }

        impl Into<$ty> for VarInt<$vk> {
            fn into(self) -> $ty {
                self.0 as $ty
            }
        }

        impl Add<$ty> for VarInt<$vk> {
            type Output = Self;

            fn add(self, other: $ty) -> Self::Output {
                VarInt(self.0 + other as $vk)
            }
        }

        impl Mul<$ty> for VarInt<$vk> {
            type Output = Self;

            fn mul(self, other: $ty) -> Self::Output {
                VarInt(self.0 * other as $vk)
            }
        }

        impl Sub<$ty> for VarInt<$vk> {
            type Output = Self;

            fn sub(self, other: $ty) -> Self::Output {
                VarInt(self.0 - other as $vk)
            }
        }

        impl Div<$ty> for VarInt<$vk> {
            type Output = Self;

            fn div(self, other: $ty) -> Self::Output {
                VarInt(self.0 / other as $vk)
            }
        }
    };
}
impl_primitive_VarInt!(u8, u32);
impl_primitive_VarInt!(u16, u32);
impl_primitive_VarInt!(u32, u32);
impl_primitive_VarInt!(u64, u32);
impl_primitive_VarInt!(u128, u32);
impl_primitive_VarInt!(i8, u64);
impl_primitive_VarInt!(i16, u64);
impl_primitive_VarInt!(i32, u64);
impl_primitive_VarInt!(i64, u64);
impl_primitive_VarInt!(i128, u64);
