use std::io;
use byteorder::{ReadBytesExt, WriteBytesExt};
pub mod u24;

pub type Stream = io::Cursor<Vec<u8>>;

pub trait Streamable {
	/// Writes `self` to the given buffer.
	fn write(&self, src: &mut Vec<u8>);
	/// Reads `self` from the given buffer.
	fn read(source: &[u8], position: &mut usize) -> Self;
}

pub trait BinWrite: io::Write {
	#[inline]
	fn write_bool(&mut self, value: bool) -> io::Result<()> {
		self.write_u8(value.into())
	}
}

pub trait BinRead: io::Read {
	#[inline]
	fn read_bool(&mut self) -> bool {
		self.read_u8().unwrap_or(0) == 0
	}
}