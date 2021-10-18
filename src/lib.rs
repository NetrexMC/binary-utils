#![feature(log_syntax)]

use std::convert::TryInto;
use std::io;

pub use bin_macro::*;

use std::io::{Cursor, Write};
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

pub mod u24;
pub mod util;
pub mod varint;

pub use self::{u24::*, varint::*};

pub type Stream = io::Cursor<Vec<u8>>;

pub trait Streamable {
    /// Writes `self` to the given buffer.
    fn parse(&self) -> Vec<u8>;
    /// Reads `self` from the given buffer.
    fn compose(source: &[u8], position: &mut usize) -> Self
    where
        Self: Sized;
}

/// Little Endian Encoding
pub struct LE<T>(pub T);

/// Big Endian Encoding
pub struct BE<T>(pub T);

macro_rules! impl_streamable_primitive {
    ($ty: ty) => {
        impl Streamable for $ty {
            fn parse(&self) -> Vec<u8> {
                self.to_be_bytes().to_vec()
            }

            fn compose(source: &[u8], position: &mut usize) -> Self {
                // get the size
                let size = ::std::mem::size_of::<$ty>();
                let range = position.clone()..(size + position.clone());
                let data = <$ty>::from_be_bytes(source.get(range).unwrap().try_into().unwrap());
                *position += size;
                data
            }
        }
    };
}

impl_streamable_primitive!(u8);
impl_streamable_primitive!(u16);
impl_streamable_primitive!(u32);
impl_streamable_primitive!(u64);
impl_streamable_primitive!(u128);
impl_streamable_primitive!(i8);
impl_streamable_primitive!(i16);
impl_streamable_primitive!(i32);
impl_streamable_primitive!(i64);
impl_streamable_primitive!(i128);

macro_rules! impl_streamable_vec_primitive {
    ($ty: ty) => {
        impl Streamable for Vec<$ty> {
            fn parse(&self) -> Vec<u8> {
                use ::std::io::Write;
                // write the length as a varint
                let mut v: Vec<u8> = Vec::new();
                v.write_all(&VarInt(v.len() as u32).to_be_bytes()[..])
                    .unwrap();
                for x in self.iter() {
                    v.extend(x.parse().iter());
                }
                v
            }

            fn compose(source: &[u8], position: &mut usize) -> Self {
                // use ::std::io::Read;
                // read a var_int
                let mut ret: Vec<$ty> = Vec::new();
                let varint = VarInt::<u32>::from_be_bytes(source);
                let length: u32 = varint.into();

                *position += varint.get_byte_length() as usize;

                // read each length
                for _ in 0..length {
                    ret.push(<$ty>::compose(&source, position));
                }
                ret
            }
        }
    };
}

impl_streamable_vec_primitive!(u8);
impl_streamable_vec_primitive!(u16);
impl_streamable_vec_primitive!(u32);
impl_streamable_vec_primitive!(u64);
impl_streamable_vec_primitive!(u128);
impl_streamable_vec_primitive!(i8);
impl_streamable_vec_primitive!(i16);
impl_streamable_vec_primitive!(i32);
impl_streamable_vec_primitive!(i64);
impl_streamable_vec_primitive!(i128);


// implements bools
impl Streamable for bool {
    fn parse(&self) -> Vec<u8> {
        vec![if *self { 1 } else { 0 }]
    }

    fn compose(source: &[u8], position: &mut usize) -> Self {
        let v = source[*position] == 1;
        *position += 1;
        v
    }
}

impl Streamable for String {
    fn parse(&self) -> Vec<u8> {
         let mut buffer = Vec::<u8>::new();
         buffer.write_u16::<BigEndian>(self.len() as u16).unwrap();
         buffer.write_all(self.as_bytes()).unwrap();
         buffer
    }

    fn compose(source: &[u8], position: &mut usize) -> Self {
         let mut stream = Cursor::new(source);
         stream.set_position(position.clone() as u64);
         // Maybe do this in the future?
         let len: usize = stream.read_u16::<BigEndian>().unwrap().into();

         unsafe {
              // todo: Remove this nasty hack.
              // todo: The hack being, remove the 2 from indexing on read_short
              // todo: And utilize stream.
              String::from_utf8_unchecked(stream.get_ref()[2..len].to_vec())
         }
    }
}


// impl<T> Streamable for Vec<T>
// where
//     T: Streamable {
//     fn write(&self) -> Vec<u8> {
//         // write the length as a varint
//         let mut v: Vec<u8> = Vec::new();
//         v.write_all(&VarInt(v.len() as u32).to_be_bytes()[..]).unwrap();
//         for x in self.iter() {
//             v.extend(x.write().iter());
//         }
//         v
//     }

//     fn read(source: &[u8], position: &mut usize) -> Self {
//         // read a var_int
//         let mut ret: Vec<T> = Vec::new();
//         let varint = VarInt::<u32>::from_be_bytes(source);
//         let length: u32 = varint.into();

//         *position += varint.get_byte_length() as usize;

//         // read each length
//         for _ in 0..length {
//             ret.push(T::read(&source, position));
//         }
//         ret
//     }
// }
