use std::ops::Deref;

pub trait IBufferRead {
     fn read_byte() -> u8;
     fn read_signed_byte() -> u8;
     fn read_bool() -> bool;
     fn read_short() -> u8;
     fn read_signed_short() -> u8;
     fn read_short_le() -> u8;
     fn read_signed_short_le() -> u8;
     fn read_triad() -> usize;
     fn read_triad_le() -> usize;
     fn read_int() -> usize;
     fn read_int_le() -> usize;
     fn read_float() -> f32;
     fn read_float_le() -> f32;
     fn read_double() -> f64;
     fn read_double_le() -> f64;
     fn read_long() -> usize;
     fn read_long_le() -> usize;
     fn read_var_int() -> usize;
     fn read_signed_var_int() -> usize;
     fn read_var_long() -> usize;
     fn read_signed_var_long() -> usize;
}

pub trait IBufferWrite {
     fn write_byte(&self, v: u8);
     fn write_signed_byte(&self, v: u8);
     fn write_bool(&self, v: bool);
     fn write_short(&self, v: u8);
     fn write_signed_short(&self, v: u8);
     fn write_short_le(&self, v: u8);
     fn write_signed_short_le(&self, v: u8);
     fn write_triad(&self, v: usize);
     fn write_triad_le(&self, v: usize);
     fn write_int(&self, v: usize);
     fn write_int_le(&self, v: usize);
     fn write_float(&self, v: f32);
     fn write_float_le(&self, v: f32);
     fn write_double(&self, v: f64);
     fn write_double_le(&self, v: f64);
     fn write_long(&self, v: usize);
     fn write_long_le(&self, v: usize);
     fn write_var_int(&self, v: usize);
     fn write_signed_var_int(&self, v: usize);
     fn write_var_long(&self, v: usize);
     fn write_signed_var_long(&self, v: usize);
}


/// Buffer implementation on Array
impl<Arr> IBufferRead for Arr where Arr: Deref<Target = [u8]> {
     fn read_byte() -> u8 {
          0
     }

     fn read_signed_byte() -> u8 {
          0
     }

     fn read_bool() -> bool {
          false
     }

     fn read_short() -> u8 {
          0
     }

     fn read_signed_short() -> u8 {
          0
     }

     fn read_short_le() -> u8 {
          0
     }

     fn read_signed_short_le() -> u8 {
          0
     }

     fn read_triad() -> usize {
          0
     }

     fn read_triad_le() -> usize {
          0
     }

     fn read_int() -> usize {
          0
     }

     fn read_int_le() -> usize {
          0
     }

     fn read_float() -> f32 {
          0.0
     }

     fn read_float_le() -> f32 {
          0.0
     }

     fn read_double() -> f64 {
          0.
     }

     fn read_double_le() -> f64 {
          0.0
     }

     fn read_long() -> usize {
          0
     }

     fn read_long_le() -> usize {
          0
     }

     fn read_var_int() -> usize {
          0
     }

     fn read_signed_var_int() -> usize {
          0
     }

     fn read_var_long() -> usize {
          0
     }

     fn read_signed_var_long() -> usize {
          0
     }
}