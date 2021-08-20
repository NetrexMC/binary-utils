use std::ops::Deref;
use std::convert::TryInto;
use std::string::FromUtf8Error;

pub trait IBufferRead {
     /// Reads a unsigned byte (0 - 255)
     /// There is no `u` preceeding the function name
     /// because unsigned bytes are default.
     fn read_byte(&mut self) -> u8;
     /// Reads a signed byte (-128 - 128)
     fn read_ibyte(&mut self) -> i8;
     /// Reads a boolean, either `true` or `false`
     /// A boolean is represented as follows:
     /// - **true** -> 0b01
     /// - **false** -> 0b00
     fn read_bool(&mut self) -> bool;
     fn read_short(&mut self) -> i16;
     fn read_ushort(&mut self) -> u16;
     fn read_short_le(&mut self) -> i16;
     fn read_ushort_le(&mut self) -> u16;
     /// Reads a three byte unsigned integer.
     /// A triad is defined as follows:
     /// - **MAX:** 0x00FF_FFFF
     /// - **MIN:** 0x0000_0000
     fn read_triad(&mut self) -> u32;
     /// Same as `read_triad` except reads in a
     /// [Big Endian](https://www.freecodecamp.org/news/what-is-endianness-big-endian-vs-little-endian/) format.
     fn read_triad_be(&mut self) -> u32;
     fn read_int(&mut self) -> i32;
     fn read_int_le(&mut self) -> i32;
     fn read_uint(&mut self) -> u32;
     fn read_uint_le(&mut self) -> u32;
     fn read_float(&mut self) -> f32;
     fn read_float_le(&mut self) -> f32;
     fn read_double(&mut self) -> f64;
     fn read_double_le(&mut self) -> f64;
     fn read_long(&mut self) -> i64;
     fn read_ulong(&mut self) -> u64;
     fn read_long_le(&mut self) -> i64;
     fn read_ulong_le(&mut self) -> u64;
     fn read_var_int(&mut self) -> i32;
     fn read_uvar_int(&mut self) -> u32;
     fn read_var_long(&mut self) -> i64;
     fn read_uvar_long(&mut self) -> u64;
     fn read_string(&mut self) -> Result<String, FromUtf8Error>;
}

pub trait IBufferWrite {
     fn write_byte(&mut self, v: u8);
     fn write_ibyte(&mut self, v: i8);
     fn write_bool(&mut self, v: bool);
     fn write_short(&mut self, v: i16);
     fn write_ushort(&mut self, v: u16);
     fn write_short_le(&mut self, v: i16);
     fn write_ushort_le(&mut self, v: u16);
     // Any bytes exceeding the size of a 3 byte number, are automatically removed.
     fn write_triad(&mut self, v: u32);
     fn write_triad_be(&mut self, v: u32);
     fn write_int(&mut self, v: i32);
     fn write_int_le(&mut self, v: i32);
     fn write_uint(&mut self, v: u32);
     fn write_uint_le(&mut self, v: u32);
     fn write_float(&mut self, v: f32);
     fn write_float_le(&mut self, v: f32);
     fn write_double(&mut self, v: f64);
     fn write_double_le(&mut self, v: f64);
     fn write_long(&mut self, v: i64);
     fn write_ulong(&mut self, v: u64);
     fn write_long_le(&mut self, v: i64);
     fn write_ulong_le(&mut self, v: u64);
     fn write_var_int(&mut self, v: i32);
     fn write_uvar_int(&mut self, v: u32);
     fn write_var_long(&mut self, v: i64);
     fn write_uvar_long(&mut self, v: u64);
     fn write_string(&mut self, v: String);
}

pub fn is_u24(num: u32) -> bool {
     // checks if num is within range
     !(num > 0x00FF_FFFF)
}

// /// Buffer implementation on Array (im lazy someone pls)
// impl<T> IBufferRead for T where T: Deref<Target = [u8]> {
//      fn read_byte(&mut self) -> u16 {
//           0
//      }

//      fn read_signed_byte(&mut self) -> i8 {
//           0
//      }

//      fn read_bool(&mut self) -> bool {
//           false
//      }

//      fn read_short(&mut self) -> u16 {
//           0
//      }

//      fn read_signed_short(&mut self) -> u16 {
//           0
//      }

//      fn read_short_le(&mut self) -> u16 {
//           0
//      }

//      fn read_signed_short_le(&mut self) -> u16 {
//           0
//      }

//      fn read_triad(&mut self) -> usize {
//           0
//      }

//      fn read_triad_le(&mut self) -> usize {
//           0
//      }

//      fn read_int(&mut self) -> usize {
//           0
//      }

//      fn read_int_le(&mut self) -> usize {
//           0
//      }

//      fn read_float(&mut self) -> f32 {
//           0.0
//      }

//      fn read_float_le(&mut self) -> f32 {
//           0.0
//      }

//      fn read_double(&mut self) -> f64 {
//           0.0
//      }

//      fn read_double_le(&mut self) -> f64 {
//           0.0
//      }

//      fn read_long(&mut self) -> usize {
//           0
//      }

//      fn read_long_le(&mut self) -> usize {
//           0
//      }

//      fn read_var_int(&mut self) -> usize {
//           0
//      }

//      fn read_signed_var_int(&mut self) -> usize {
//           0
//      }

//      fn read_var_long(&mut self) -> usize {
//           0
//      }

//      fn read_signed_var_long(&mut self) -> usize {
//           0
//      }
// }