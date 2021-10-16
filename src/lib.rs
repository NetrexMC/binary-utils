#![feature(log_syntax)]

use std::convert::TryInto;
use std::io;

pub use bin_macro::*;

pub mod u24;
pub mod varint;

pub use self::{
    u24::*,
    varint::*
};

pub type Stream = io::Cursor<Vec<u8>>;

pub trait Streamable {
    /// Writes `self` to the given buffer.
    fn write(&self) -> Vec<u8>;
    /// Reads `self` from the given buffer.
    fn read(source: &[u8], position: &mut usize) -> Self
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
            fn write(&self) -> Vec<u8> {
                self.to_be_bytes().to_vec()
            }

            fn read(source: &[u8], position: &mut usize) -> Self {
                // get the size
                let size = ::std::mem::size_of::<$ty>();
                let range = position.clone()..size;
                let data = <$ty>::from_be_bytes(source[range].try_into().unwrap());
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

// implements bools
impl Streamable for bool {
    fn write(&self) -> Vec<u8> {
        vec![if *self { 1 } else { 0 }]
    }

    fn read(source: &[u8], position: &mut usize) -> Self {
        let v = source[*position] == 1;
        *position += 1;
        v
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