#![feature(log_syntax)]

use std::io;
use byteorder::{ReadBytesExt, WriteBytesExt};
use std::convert::TryInto;

// pub use bin_macro::*;

pub mod u24;

pub type Stream = io::Cursor<Vec<u8>>;

pub trait Streamable {
	/// Writes `self` to the given buffer.
	fn write(&self) -> Vec<u8>;
	/// Reads `self` from the given buffer.
	fn read(source: &[u8], position: &mut usize) -> Self where Self: Sized;
}

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