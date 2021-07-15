use std::ops::Deref;
use std::convert::TryInto;
use std::string::FromUtf8Error;

pub trait IBufferRead {
     fn read_byte(&mut self) -> u8;
     fn read_signed_byte(&mut self) -> i8;
     fn read_bool(&mut self) -> bool;
     fn read_short(&mut self) -> u16;
     fn read_signed_short(&mut self) -> i16;
     fn read_short_le(&mut self) -> u16;
     fn read_signed_short_le(&mut self) -> i16;
     /// ! This method contains undefined behavior
     fn read_triad(&mut self) -> usize;
     /// ! This method contains undefined behavior
     fn read_triad_le(&mut self) -> usize;
     fn read_int(&mut self) -> i16;
     fn read_int_le(&mut self) -> i16;
     fn read_float(&mut self) -> f32;
     fn read_float_le(&mut self) -> f32;
     fn read_double(&mut self) -> f64;
     fn read_double_le(&mut self) -> f64;
     fn read_long(&mut self) -> i64;
     fn read_long_le(&mut self) -> i64;
     fn read_var_int(&mut self) -> isize;
     fn read_signed_var_int(&mut self) -> isize;
     fn read_var_long(&mut self) -> isize;
     fn read_signed_var_long(&mut self) -> isize;
     fn read_string(&mut self) -> Result<String, FromUtf8Error>;
}

pub trait IBufferWrite {
     fn write_byte(&mut self, v: u8);
     fn write_signed_byte(&mut self, v: i8);
     fn write_bool(&mut self, v: bool);
     fn write_short(&mut self, v: u16);
     fn write_signed_short(&mut self, v: i16);
     fn write_short_le(&mut self, v: u16);
     fn write_signed_short_le(&mut self, v: i16);
     fn write_triad(&mut self, v: usize);
     fn write_triad_le(&mut self, v: usize);
     fn write_int(&mut self, v: i16);
     fn write_int_le(&mut self, v: i16);
     fn write_float(&mut self, v: f32);
     fn write_float_le(&mut self, v: f32);
     fn write_double(&mut self, v: f64);
     fn write_double_le(&mut self, v: f64);
     fn write_long(&mut self, v: i64);
     fn write_long_le(&mut self, v: i64);
     fn write_var_int(&mut self, v: isize);
     fn write_signed_var_int(&mut self, v: isize);
     fn write_var_long(&mut self, v: isize);
     fn write_signed_var_long(&mut self, v: isize);
     fn write_string(&mut self, v: String);
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