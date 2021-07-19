use std::convert::TryInto;
use std::string::FromUtf8Error;
use std::ops::{ Range, Index, IndexMut };
use std::error::Error;
use std::fmt::{ Display, Formatter, Result as FResult };
use std::mem;

use super::buffer;

// Errors for binarystream
pub struct AllocationError;

#[derive(Debug)]
pub struct IllegalOffsetError {
     legal: usize,
     actual: usize,
     cause_bounds: bool
}

#[derive(Debug)]
pub enum ClampErrorCause {
     AboveBounds,
     BelowBounds,
     InvalidBounds
}

#[derive(Debug)]
pub struct ClampError {
     cause: ClampErrorCause
}

impl ClampError {
     fn new(cause: ClampErrorCause) -> Self {
          Self {
               cause
          }
     }
}

impl Display for AllocationError {
     fn fmt(&self, f: &mut Formatter) -> FResult {
          write!(f, "Attempted to allocate negative bytes.")
     }
}

impl Display for IllegalOffsetError {
     fn fmt(&self, f: &mut Formatter) -> FResult {
          if self.cause_bounds {
               write!(f, "Offset: {} is outside of the BinaryStream's bounds", self.actual)
          } else {
               write!(f, "Offset: {} is outside of possible offset of: {}", self.actual, self.cause_bounds)
          }
     }
}

impl Display for ClampError {
     fn fmt(&self, f: &mut Formatter) -> FResult {
          match self.cause {
               ClampErrorCause::AboveBounds => write!(f, "Clamp start can not be bigger than the buffer's length."),
               ClampErrorCause::BelowBounds => write!(f, "Clamp end is less than the start length."),
               ClampErrorCause::InvalidBounds => write!(f, "Clamp start or end is mismatched.")
          }
     }
}

pub trait IBinaryStream {
     /// Increases the offset. If `None` is given in `amount`, 1 will be used.
     fn increase_offset(&mut self, amount: Option<usize>) -> usize;

     /// Changes the offset of the stream to the new given offset.
     /// returns `true` if the offset is in bounds and `false` if the offset is out of bounds.
     fn set_offset(&mut self, offset: usize) -> bool;

     /// Returns the current offset at the given time when called.
     fn get_offset(&mut self) -> usize;

     /// Returns the length of the current buffer.
     fn get_length(&self) -> usize;

     /// Returns the current buffer as a clone of the original.
     fn get_buffer(&self) -> Vec<u8>;

     /// Allocates more bytes to the binary stream.
     /// Allocations can occur as many times as desired, however a negative allocation will cause
     /// the stream to "drop" or "delete" bytes from the buffer. Discarded bytes are not recoverable.
     ///
     /// Useful when writing to a stream, allows for allocating for chunks, etc.
     ///
     /// **Example:**
     ///
     ///     stream.allocate(1024);
     ///     stream.write_string(String::from("a random string, that can only be a max of 1024 bytes."));
     fn allocate(&mut self, bytes: usize);

     /// Allocates more bytes to the binary stream only **if** the given bytelength will exceed
     /// the current binarystream's bounds.
     fn allocate_if(&mut self, bytes: usize);

     /// Create a new Binary Stream from nothing.
     fn new() -> Self;

     /// Create a new Binary Stream from a vector of bytes.
     fn init(buf: &Vec<u8>) -> Self;

     /// Similar to slice, clamp, "grips" the buffer from a given offset, and changes the initial bounds.
     /// Meaning that any previous bytes before the given bounds are no longer writable.
     ///
     /// Useful for cloning "part" of a stream, and only allowing certain "bytes" to be read.
     /// Clamps can not be undone.
     ///
     /// **Example:**
     ///
     ///     let stream = BinaryStream::new(vec!(([98,105,110,97,114,121,32,117,116,105,108,115]));
     ///     let shareable_stream = stream.clamp(7, None); // 32,117,116,105,108,115 are now the only bytes readable externally
     fn clamp(&mut self, start: usize, end: Option<usize>) -> Self;

     /// Checks whether or not the given offset is in between the streams bounds and if the offset is valid.
     ///
     /// **Example:**
     ///
     ///     if stream.is_within_bounds(100) {
     ///       println!("Can write to offset: 100");
     ///     } else {
     ///       println!("100 is out of bounds.");
     ///     }
     fn is_within_bounds(&self, offset: usize) -> bool;

     /// Reads a byte, updates the offset, clamps to last offset.
     ///
     /// **Example:**
     ///
     ///      let mut fbytes = Vec::new();
     ///      loop {
     ///         if fbytes.len() < 4 {
     ///           fbytes.push(stream.read());
     ///         }
     ///         break;
     ///      }
     fn read(&mut self) -> u8;

     /// Writes a byte ands returns it.
     fn write_usize(&mut self, v: usize) -> usize;

     /// Reads a slice from the Binary Stream and automatically updates
     /// the offset for the given slice's length.
     ///
     /// **Example:**
     ///     stream.read_slice();
     fn read_slice(&mut self, length: Option<usize>) -> Vec<u8>;

     /// Reads a slice from the Binary Stream and automatically updates
     /// the offset for the given slice's length.
     ///
     /// ! This function indexes from 0 always!
     ///
     /// **Example:**
     ///     stream.read_slice();
     fn read_slice_exact(&mut self, length: Option<usize>) -> Vec<u8>;

     /// Writes a slice onto the Binary Stream and automatically allocates
     /// memory for the slice.
     ///
     /// **Example:**
     ///     stream.write_slice(&[0, 38, 92, 10]);
     fn write_slice(&mut self, v: &[u8]);
}

#[derive(Debug, Clone)]
pub struct BinaryStream {
     buffer: Vec<u8>,
     offset: usize,
     bounds: (usize, usize)
}

impl IBinaryStream for BinaryStream {
     /// Increases the offset. If `None` is given in `amount`, 1 will be used.
     fn increase_offset(&mut self, amount: Option<usize>) -> usize {
          let amnt = match amount {
               None => 1 as usize,
               Some(n) => n
          };

          if (self.offset + amnt) > self.bounds.1 {
               panic!(IllegalOffsetError {
                    actual: amnt,
                    legal: self.bounds.1,
                    cause_bounds: true
               })
          }

          self.offset = self.offset + amnt;
          self.offset
     }

     /// Changes the offset of the stream to the new given offset.
     /// returns `true` if the offset is in bounds and `false` if the offset is out of bounds.
     fn set_offset(&mut self, offset: usize) -> bool {
          if offset > self.bounds.1 {
               false
          } else {
               self.offset = offset;
               true
          }
     }

     /// Returns the current offset at the given time when called.
     fn get_offset(&mut self) -> usize {
          self.offset
     }

     /// Returns the length of the current buffer.
     fn get_length(&self) -> usize {
          self.buffer.len() as usize
     }

     /// Returns the current buffer as a clone of the original.
     fn get_buffer(&self) -> Vec<u8> {
          self.buffer.clone()
     }

     /// Allocates more bytes to the binary stream.
     /// Allocations can occur as many times as desired, however a negative allocation will cause
     /// the stream to "drop" or "delete" bytes from the buffer. Discarded bytes are not recoverable.
     ///
     /// Useful when writing to a stream, allows for allocating for chunks, etc.
     ///
     /// **Example:**
     ///
     ///     stream.allocate(1024);
     ///     stream.write_string(String::from("a random string, that can only be a max of 1024 bytes."));
     fn allocate(&mut self, bytes: usize) {
          self.bounds.1 = self.buffer.len() + bytes;
          // self.buffer.resize(self.bounds.1, 0)
     }

     /// Allocates more bytes to the binary stream only **if** the given bytelength will exceed
     /// the current binarystream's bounds.
     fn allocate_if(&mut self, bytes: usize) {
          if (self.buffer.len() + bytes > self.bounds.1) && (self.offset + bytes) >= self.bounds.1 {
               self.allocate(bytes)
          }
     }

     /// Create a new Binary Stream from nothing.
     fn new() -> Self {
          Self {
               buffer: Vec::new(),
               bounds: (0, 0),
               offset: 0
          }
     }

     /// Create a new Binary Stream from a vector of bytes.
     fn init(buf: &Vec<u8>) -> Self {
          Self {
               buffer: buf.clone(),
               bounds: (0, buf.len()),
               offset: 0
          }
     }

     /// Similar to slice, clamp, "grips" the buffer from a given offset, and changes the initial bounds.
     /// Meaning that any previous bytes before the given bounds are no longer writable.
     ///
     /// Useful for cloning "part" of a stream, and only allowing certain "bytes" to be read.
     /// Clamps can not be undone.
     ///
     /// **Example:**
     ///
     ///     let stream = BinaryStream::new(vec!(([98,105,110,97,114,121,32,117,116,105,108,115])));
     ///     let shareable_stream = stream.clamp(7, None); // 32,117,116,105,108,115 are now the only bytes readable externally
     fn clamp(&mut self, start: usize, end: Option<usize>) -> Self {
          if start > self.buffer.len() {
               panic!(ClampError::new(ClampErrorCause::AboveBounds));
          } else if start < self.bounds.0 {
               panic!(ClampError::new(ClampErrorCause::BelowBounds));
          }

          self.bounds.0 = start;

          if match end { None => false, _ => true} {
               if end.unwrap() < self.bounds.0 {
                    panic!(ClampError::new(ClampErrorCause::InvalidBounds));
               }
               self.bounds.1 = end.unwrap();
          }

          BinaryStream::init(&mut self.buffer.clone()) // Dereferrenced for use by consumer.
     }

     /// Checks whether or not the given offset is in between the streams bounds and if the offset is valid.
     ///
     /// **Example:**
     ///
     ///     if stream.is_within_bounds(100) {
     ///       println!("Can write to offset: 100");
     ///     } else {
     ///       println!("100 is out of bounds.");
     ///     }
     fn is_within_bounds(&self, offset: usize) -> bool {
          !(offset > self.bounds.1 || offset < self.bounds.0 || offset > self.buffer.len())
     }

     /// Reads a byte, updates the offset, clamps to last offset.
     ///
     /// **Example:**
     ///
     ///      let mut fbytes = Vec::new();
     ///      loop {
     ///         if fbytes.len() < 4 {
     ///           fbytes.push(stream.read());
     ///         }
     ///         break;
     ///      }
     fn read(&mut self) -> u8 {
          let byte = self[self.offset];
          self.clamp(self.offset, None);
          self.increase_offset(None);
          byte
     }

     /// Writes a byte ands returns it.
     fn write_usize(&mut self, v: usize) -> usize {
          self.allocate_if(1);
          self.buffer.push(v as u8);
          v
     }

     /// Writes a slice onto the Binary Stream and automatically allocates
     /// memory for the slice.
     ///
     /// **Example:**
     ///     stream.write_slice(&[0, 38, 92, 10]);
     fn write_slice(&mut self, v: &[u8]) {
          self.allocate_if(v.len());
          self.buffer.extend_from_slice(v);
     }

     /// Reads a slice from the Binary Stream and automatically updates
     /// the offset for the given slice's length.
     ///
     /// **Example:**
     ///     stream.read_slice();
     fn read_slice_exact(&mut self, length: Option<usize>) -> Vec<u8> {
          let len = match length {
               Some(v) => v,
               None => 1
          };
          let vec = self[self.offset..len].to_vec();
          self.increase_offset(Some(len));
          vec
     }

     /// Reads a slice from the Binary Stream and automatically updates
     /// the offset for the given slice's length.
     ///
     /// **Example:**
     ///     stream.read_slice();
     fn read_slice(&mut self, length: Option<usize>) -> Vec<u8> {
          let len = match length {
               Some(v) => v,
               None => 1
          };
          let vec = self[self.offset..len + self.offset].to_vec();
          self.increase_offset(Some(len));
          vec
     }
}

/// Implements indexing on BinaryStream.
/// When indexing you can access the bytes only readable by the streams bounds.
/// If the offset you're trying to index is "outside" of the "bounds" of the stream this will panic.
///
/// **Example:**
///
///     let first_byte = stream[0];
impl std::ops::Index<usize> for BinaryStream {
     type Output = u8;
     fn index(&self, idx: usize) -> &u8 {
          if !self.is_within_bounds(idx) {
               if self.bounds.0 == 0 && self.bounds.1 == self.buffer.len() {
                    panic!("Index is out of bounds due to clamp.");
               } else {
                    panic!("Index is out of bounds.");
               }
          }

          self.buffer.get(idx).unwrap()
     }
}

/// Implements indexing with slices on BinaryStream.
/// Operates exactly like indexing, except with slices.
///
/// **Example:**
///
///     let first_bytes = stream[0..3];
impl Index<Range<usize>> for BinaryStream {
     type Output = [u8];
     fn index(&self, idx: Range<usize>) -> &[u8] {
          if !self.is_within_bounds(idx.end) || !self.is_within_bounds(idx.start) {
               if self.bounds.0 == 0 && self.bounds.1 == self.buffer.len() {
                    panic!("Index is out of bounds due to clamp.");
               } else {
                    panic!("Index is out of bounds.");
               }
          }

          self.buffer.get(idx).unwrap()
     }
}

impl std::ops::IndexMut<usize> for BinaryStream {
     fn index_mut(&mut self, offset: usize) -> &mut u8 {
          if !self.is_within_bounds(offset) {
               self.buffer.get_mut(offset).unwrap()
          } else {
               panic!("Offset: {} is out of bounds.", offset);
          }
     }
}

impl buffer::IBufferRead for BinaryStream {
     /// Literally, reads a byte
     fn read_ibyte(&mut self) -> i8 {
          // an i8 is only 1 byte
          let b = i8::from_be_bytes(self.buffer[self.offset..self.offset + 1].try_into().unwrap());
          self.increase_offset(Some(1));
          b
     }

     fn read_byte(&mut self) -> u8 {
          let byte = self.buffer[self.offset];
          self.increase_offset(None);
          byte
     }

     fn read_bool(&mut self) -> bool {
          self.read_byte() != 0
     }

     fn read_string(&mut self) -> Result<String, FromUtf8Error> {
          let length = self.read_short();
          let string = String::from_utf8(self[self.offset..self.offset + length as usize].to_vec());
          self.increase_offset(Some(self.offset + length as usize));
          string
     }

     fn read_short(&mut self) -> i16 {
          let b = i16::from_be_bytes(self.buffer[self.offset..self.offset + 2].try_into().unwrap());
          self.increase_offset(Some(2));
          b
     }

     fn read_ushort(&mut self) -> u16 {
          // a short is 2 bytes and is a u16,
          let b = u16::from_be_bytes(self.buffer[self.offset..self.offset + 2].try_into().unwrap());
          self.increase_offset(Some(2));
          b
     }

     fn read_short_le(&mut self) -> i16 {
          let b = i16::from_le_bytes(self.buffer[self.offset..self.offset + 2].try_into().unwrap());
          self.increase_offset(Some(2));
          b
     }

     fn read_ushort_le(&mut self) -> u16 {
          let b = u16::from_le_bytes(self.buffer[self.offset..self.offset + 2].try_into().unwrap());
          self.increase_offset(Some(2));
          b
     }

     fn read_triad(&mut self) -> usize {
          // a triad is 3 bytes
          let b = u32::from_be_bytes(self[self.offset..self.offset + 4].try_into().unwrap());
          self.increase_offset(Some(3));
          b as usize
     }

     fn read_triad_le(&mut self) -> usize {
          let b = u32::from_le_bytes(self[self.offset..self.offset + 4].try_into().unwrap());
          self.increase_offset(Some(3));
          b as usize
     }

     fn read_int(&mut self) -> i16 {
          self.read_short()
     }


     fn read_int_le(&mut self) -> i16 {
          self.read_short_le()
     }

     fn read_float(&mut self) -> f32 {
          let b = f32::from_be_bytes(self.buffer[self.offset..self.offset + 4].try_into().unwrap());
          self.increase_offset(Some(4));
          b
     }

     fn read_float_le(&mut self) -> f32 {
          let b = f32::from_le_bytes(self.buffer[self.offset..self.offset + 4].try_into().unwrap());
          self.increase_offset(Some(4));
          b
     }

     fn read_double(&mut self) -> f64 {
          let b = f64::from_be_bytes(self.buffer[self.offset..self.offset + 8].try_into().unwrap());
          self.increase_offset(Some(8));
          b
     }

     fn read_double_le(&mut self) -> f64 {
          let b = f64::from_le_bytes(self.buffer[self.offset..self.offset + 8].try_into().unwrap());
          self.increase_offset(Some(8));
          b
     }

     fn read_long(&mut self) -> i64 {
          let b = i64::from_be_bytes(self.buffer[self.offset..self.offset + 8].try_into().unwrap());
          self.increase_offset(Some(8));
          b
     }

     fn read_ulong(&mut self) -> u64 {
          let b = u64::from_be_bytes(self.buffer[self.offset..self.offset + 8].try_into().unwrap());
          self.increase_offset(Some(8));
          b
     }

     fn read_long_le(&mut self) -> i64 {
          let b = i64::from_le_bytes(self.buffer[self.offset..self.offset + 8].try_into().unwrap());
          self.increase_offset(Some(8));
          b
     }

     fn read_ulong_le(&mut self) -> u64 {
          let b = u64::from_le_bytes(self.buffer[self.offset..self.offset + 8].try_into().unwrap());
          self.increase_offset(Some(8));
          b
     }

     fn read_var_int(&mut self) -> isize {
          // taken from pmmp, this might be messed up
          let mut b: u16 = 0;
          let mut i = 0;
          while i <= 28 {
               let byte: u16 = self.read_byte().try_into().unwrap();
               b |= (byte & 0x7f) << i;
               if (byte & 0x80) == 0 {
                    return b as isize
               }
               i += 7;
          }
          return b as isize;
     }

     fn read_uvar_int(&mut self) -> isize {
          0
     }

     fn read_var_long(&mut self) -> isize {
          0
     }

     fn read_uvar_long(&mut self) -> isize {
          0
     }
}

impl buffer::IBufferWrite for BinaryStream {
     fn write_ibyte(&mut self, v: i8) {
          self.write_slice(&v.to_be_bytes())
     }

     fn write_byte(&mut self, v: u8) {
          self.write_slice(&v.to_be_bytes())
     }

     fn write_bool(&mut self, v: bool) {
          let byte = match v {
               true => 1,
               false => 0
          };
          self.write_byte(byte);
     }

     fn write_short(&mut self, v: i16) {
          self.write_slice(&v.to_be_bytes());
     }

     fn write_ushort(&mut self, v: u16) {
          self.write_slice(&v.to_be_bytes());
     }

     fn write_short_le(&mut self, v: i16) {
          self.write_slice(&v.to_le_bytes());
     }

     fn write_ushort_le(&mut self, v: u16) {
          self.write_slice(&v.to_le_bytes());
     }

     fn write_triad(&mut self, v: usize) {
          let bytes = &v.to_be_bytes()[1..4];
          self.write_slice(bytes);
     }

     fn write_triad_le(&mut self, v: usize) {
          let bytes = &v.to_le_bytes()[1..4];
          self.write_slice(bytes);
     }

     fn write_int(&mut self, v: i16) {
          self.write_slice(&v.to_be_bytes());
     }

     fn write_int_le(&mut self, v: i16) {
          self.write_slice(&v.to_be_bytes());
     }

     fn write_float(&mut self, v: f32) {
          self.write_slice(&v.to_be_bytes());
     }

     fn write_float_le(&mut self, v: f32) {
          self.write_slice(&v.to_le_bytes());
     }

     fn write_double(&mut self, v: f64) {
          self.write_slice(&v.to_be_bytes());
     }

     fn write_double_le(&mut self, v: f64) {
          self.write_slice(&v.to_le_bytes());
     }

     fn write_long(&mut self, v: i64) {
          self.write_slice(&v.to_be_bytes());
     }

     fn write_ulong(&mut self, v: u64) {
          self.write_slice(&v.to_be_bytes());
     }

     fn write_long_le(&mut self, v: i64) {
          self.write_slice(&v.to_be_bytes());
     }

     fn write_ulong_le(&mut self, v: u64) {
          self.write_slice(&v.to_le_bytes());
     }

     fn write_var_int(&mut self, v: isize) {
          self.write_slice(&v.to_be_bytes());
     }

     fn write_signed_var_int(&mut self, v: isize) {
          self.write_slice(&v.to_be_bytes());
     }

     fn write_var_long(&mut self, v: isize) {
          self.write_slice(&v.to_be_bytes());
     }

     fn write_signed_var_long(&mut self, _v: isize) {

     }

     fn write_string(&mut self, v: String) {
          self.write_ushort(v.len() as u16);
          self.write_slice(v.as_bytes());
     }
}