use bytes::{Buf, BufMut, Bytes, BytesMut};
use std::{
    collections::VecDeque,
    io::{Error, IoSlice},
};

pub const ERR_EOB: &str = "No more bytes left to be read in buffer";
pub const ERR_EOM: &str = "Buffer is full, cannot write more bytes";
pub const ERR_VARINT_TOO_LONG: &str = "Varint is too long to be written to buffer";

macro_rules! can_read {
    ($self: ident, $size: expr) => {
        $self.buf.remaining() >= $size
    };
}

macro_rules! can_write {
    ($self: ident, $size: expr) => {
        $self.buf.remaining_mut() >= $size
    };
}

macro_rules! read_fn {
    ($name: ident, $typ: ident, $fn_name: ident, $byte_size: literal) => {
        #[inline]
        pub fn $name(&mut self) -> Result<$typ, std::io::Error> {
            if can_read!(self, $byte_size) {
                return Ok(self.buf.$fn_name());
            } else {
                return Err(Error::new(std::io::ErrorKind::UnexpectedEof, ERR_EOB));
            }
        }
    };
}

macro_rules! write_fn {
    ($name: ident, $typ: ident, $fn_name: ident, $byte_size: literal) => {
        #[inline]
        pub fn $name(&mut self, num: $typ) -> Result<(), std::io::Error> {
            if can_write!(self, $byte_size) {
                self.buf.$fn_name(num);
                return Ok(());
            } else {
                return Err(Error::new(std::io::ErrorKind::OutOfMemory, ERR_EOM));
            }
        }
    };
}

/// ByteReader is a panic-free way to read bytes from the `byte::Buf` trait.
///
/// ## Example
/// ```rust
/// use binary_utils::io::ByteReader;
///
/// fn main() {
///    let mut buf = ByteReader::from(&[0, 253, 255, 255, 255, 15][..]);
///    assert_eq!(buf.read_u8().unwrap(), 0);
///    assert_eq!(buf.read_var_i32().unwrap(), -2147483647);
/// }
/// ```
///
/// ## Peek Ahead
/// `ByteReader` also provides a utility `peek_ahead` function that allows you to
/// "peek ahead" at the next byte in the stream without advancing the stream.
///
/// Do not confuse this with any sort of "peek" function. This function does not
/// increment the read position of the stream, but rather copies the byte at the
/// specified position.
/// ```rust
/// use binary_utils::io::ByteReader;
///
/// fn main() {
///    let mut buf = ByteReader::from(&[253, 255, 14, 255, 255, 15][..]);
///    if buf.peek_ahead(3).unwrap() != 255 {
///        // buffer is corrupted!
///    } else {
///        // read the varint
///        let num = buf.read_var_i32().unwrap();
///    }
/// }
/// ```
///
/// ## Reading a struct without `Composable`
/// This is useful if you are trying to read a struct or optional type and validate the type before
/// reading the rest of the struct.
/// ```rust
/// use binary_utils::io::ByteReader;
///
/// struct PingPacket {
///    pub id: u8,
///    pub time: u64,
///    pub ack_id: Option<i32>
/// }
///
/// fn main() {
///     let mut buf = ByteReader::from(&[0, 253, 255, 255, 255, 255, 255, 255, 255, 0][..]);
///
///     // Read the id
///     let id = buf.read_u8().unwrap();
///
///     if id == 0 {
///         // Read the time
///        let time = buf.read_u64().unwrap();
///        // read ack
///        if buf.read_bool().unwrap() {
///            let ack_id = buf.read_var_i32().unwrap();
///            let packet = PingPacket { id, time, ack_id: Some(ack_id) };
///        } else {
///            let packet = PingPacket { id, time, ack_id: None };
///        }
///    }
/// }
/// ```
pub struct ByteReader {
    pub(crate) buf: Bytes,
}

impl From<ByteWriter> for ByteReader {
    fn from(writer: ByteWriter) -> Self {
        Self {
            buf: writer.buf.freeze(),
        }
    }
}

impl Into<Bytes> for ByteReader {
    fn into(self) -> Bytes {
        self.buf
    }
}

impl Into<Vec<u8>> for ByteReader {
    fn into(self) -> Vec<u8> {
        self.buf.to_vec()
    }
}

impl Into<VecDeque<u8>> for ByteReader {
    fn into(self) -> VecDeque<u8> {
        self.buf.to_vec().into()
    }
}

impl From<Bytes> for ByteReader {
    fn from(buf: Bytes) -> Self {
        Self { buf }
    }
}

impl From<Vec<u8>> for ByteReader {
    fn from(buf: Vec<u8>) -> Self {
        Self { buf: buf.into() }
    }
}

impl From<&[u8]> for ByteReader {
    fn from(buf: &[u8]) -> Self {
        Self {
            buf: Bytes::from(buf.to_vec()),
        }
    }
}

impl ByteReader {
    /// `ByteReader` also provides a utility `peek_ahead` function that allows you to
    /// "peek ahead" at the next byte in the stream without advancing the stream.
    ///
    /// Do not confuse this with any sort of "peek" function. This function does not
    /// increment the read position of the stream, but rather copies the byte at the
    /// specified position.
    /// ```rust
    /// use binary_utils::io::ByteReader;
    ///
    /// fn main() {
    ///    let mut buf = ByteReader::from(&[253, 255, 14, 255, 255, 15][..]);
    ///    if buf.peek_ahead(3).unwrap() != 255 {
    ///        // buffer is corrupted, varints can never have a leading byte less than 255 if
    ///        // Their are bytes remaining!
    ///    } else {
    ///        // read the varint
    ///        let num = buf.read_var_i32().unwrap();
    ///    }
    /// }
    /// ```
    pub fn peek_ahead(&mut self, pos: usize) -> Result<u8, std::io::Error> {
        if can_read!(self, pos) {
            return Ok(self.buf.chunk()[pos]);
        } else {
            return Err(Error::new(std::io::ErrorKind::UnexpectedEof, ERR_EOB));
        }
    }

    read_fn!(read_u8, u8, get_u8, 1);
    read_fn!(read_u16, u16, get_u16, 2);
    read_fn!(read_u16_le, u16, get_u16_le, 2);
    read_fn!(read_i16, i16, get_i16, 2);
    read_fn!(read_i16_le, i16, get_i16_le, 2);

    /// Reads a 3-byte unsigned integer from the stream.
    pub fn read_u24(&mut self) -> Result<u32, std::io::Error> {
        if can_read!(self, 3) {
            if let Ok(num) = self.read_uint(3) {
                return Ok(num as u32);
            } else {
                return Err(Error::new(std::io::ErrorKind::UnexpectedEof, ERR_EOB));
            }
        } else {
            return Err(Error::new(std::io::ErrorKind::UnexpectedEof, ERR_EOB));
        }
    }

    /// Reads a 3-byte unsigned integer from the stream in little endian.
    /// This is the same as `read_u24` but in little endian.
    pub fn read_u24_le(&mut self) -> Result<u32, std::io::Error> {
        if can_read!(self, 3) {
            if let Ok(num) = self.read_uint_le(3) {
                return Ok(num as u32);
            } else {
                return Err(Error::new(std::io::ErrorKind::UnexpectedEof, ERR_EOB));
            }
        } else {
            return Err(Error::new(std::io::ErrorKind::UnexpectedEof, ERR_EOB));
        }
    }

    pub fn read_i24(&mut self) -> Result<i32, std::io::Error> {
        if can_read!(self, 3) {
            if let Ok(num) = self.read_int(3) {
                return Ok(num as i32);
            } else {
                return Err(Error::new(std::io::ErrorKind::UnexpectedEof, ERR_EOB));
            }
        } else {
            return Err(Error::new(std::io::ErrorKind::UnexpectedEof, ERR_EOB));
        }
    }

    read_fn!(read_u32, u32, get_u32, 4);
    read_fn!(read_u32_le, u32, get_u32_le, 4);
    read_fn!(read_f32, f32, get_f32, 4);
    read_fn!(read_f32_le, f32, get_f32_le, 4);

    /// Reads a var-int 32-bit unsigned integer from the stream.
    /// This is a variable length integer that can be 1, 2, 3, or 4 bytes long.
    ///
    /// This function is recoverable, meaning that if the stream ends before the
    /// var-int is fully read, it will return an error, and will not consume the
    /// bytes that were read.
    ///
    /// #### Decoding VarInt
    /// You will need to:
    ///  1. Read the next byte
    ///  2. Get the 7 significant bits (next byte & 0x7F [segment bit]) (continuation bit is 0x80)
    ///  3. Shift the value to the left by i bits
    ///  4. Validate whether this is the last byte (next byte & 0x80 == 0)
    ///  5. IF the last byte, return the value and
    ///     add the iterative value (current byte) to the current value.
    ///
    /// A visual representation of how this works:
    /// ```txt
    /// ========================================================
    /// Buffer:          0xdd 0xc7 0x01
    ///
    /// Initial values:
    ///  i = 0           Current iteration
    ///  v = 0           Current value
    ///
    /// -------------------------------------------------------
    /// First iteration (i = 0):
    /// -------------------------------------------------------
    /// 0xdd 0xc7 0x01
    /// ^                                - b
    /// b = 0xdd                11011101 - Next byte
    /// b & 0x7F                 1011101 - 7 significant bits
    /// b << 0                   1011101 - Shifted by iteration bits
    /// b & 0x80 == 0              false - There are more bytes
    /// v =| 1011101 (93)        1011101 - Added to the current value
    /// i = i + 1:                     1 - Current iteration
    ///
    /// -------------------------------------------------------
    /// Second iteration (i = 7):
    /// -------------------------------------------------------
    /// 0xdd 0xc7 0x01
    ///      ^                           - b
    /// b = 0xc7                11000111 - Next byte
    /// b & 0x7F                 1000111 - 7 significant bits
    /// b << 7                           - Shifted by iteration bits
    ///    | -> 10001110000000
    /// b & 0x80 == 0              false - There are more bytes
    /// v =| b                           - Added to the current value
    ///    | -> 110001111011101 (25565)
    ///
    /// -------------------------------------------------------
    /// Third iteration (i = 14):
    /// -------------------------------------------------------
    /// 0xdd 0xc7 0x01
    ///           ^                      - b
    /// 0x01                    00000001 - Next byte
    /// b & 0x7F                 0000001 - 7 significant bits
    /// b << 14                          - Shifted by iteration bits
    ///   |-> 000000100000000
    /// b & 0x80 == 0               true - This byte is the last byte
    /// v = 25565                        - Last bit, return the value
    ///     |-> 110001111011101 (25565)
    /// ```
    #[inline]
    pub fn read_var_u32(&mut self) -> Result<u32, std::io::Error> {
        let mut num = 0u32;
        let mut interval = 0_usize;
        for i in (0..35).step_by(7) {
            let byte = self.peek_ahead(interval)?;

            num |= ((byte & 0x7F) as u32) << i;
            interval += 1;

            if byte & 0x80 == 0 {
                self.buf.advance(interval);
                return Ok(num);
            }
        }
        return Err(Error::new(
            std::io::ErrorKind::Other,
            "Varint overflow's 32-bit integer",
        ));
    }

    read_fn!(read_i32, i32, get_i32, 4);
    read_fn!(read_i32_le, i32, get_i32_le, 4);

    /// Reads a var-int 32-bit signed integer from the stream.
    /// This method is the same as `read_var_u32` but it will return a signed integer.
    pub fn read_var_i32(&mut self) -> Result<i32, std::io::Error> {
        // todo: fails on -2147483648, which is the minimum value for i32
        // todo: probably nothing to worry about, but should be fixed
        let num = self.read_var_u32()?;

        // for some reason this does not work on large numbers
        Ok((num >> 1) as i32 ^ -((num & 1) as i32))

        // return Ok(if num & 1 != 0 {
        //     !((num >> 1) as i32)
        // } else {
        //     (num >> 1) as i32
        // });
    }

    read_fn!(read_u64, u64, get_u64, 8);
    read_fn!(read_u64_le, u64, get_u64_le, 8);
    read_fn!(read_i64, i64, get_i64, 8);
    read_fn!(read_i64_le, i64, get_i64_le, 8);
    read_fn!(read_f64, f64, get_f64, 8);
    read_fn!(read_f64_le, f64, get_f64_le, 8);

    /// Reads a var-int 64-bit unsigned integer from the stream.
    /// This is a variable length integer that can be 1, 2, 3, 4, 5, 6, 7, or 8 bytes long.
    #[inline]
    pub fn read_var_u64(&mut self) -> Result<u64, std::io::Error> {
        let mut num = 0u64;
        let mut interval = 0_usize;
        for i in (0..70).step_by(7) {
            let byte = self.peek_ahead(interval)?;

            num |= ((byte & 0x7F) as u64) << i;
            interval += 1;

            if byte & 0x80 == 0 {
                self.buf.advance(interval);
                return Ok(num);
            }
        }
        return Err(Error::new(
            std::io::ErrorKind::Other,
            "Varint overflow's 64-bit integer",
        ));
    }

    /// Reads a var-int 64-bit signed integer from the stream.
    /// This method is the same as `read_var_u64` but it will return a signed integer.
    ///
    /// For more information on how this works, see `read_var_i32`.
    #[inline]
    pub fn read_var_i64(&mut self) -> Result<i64, std::io::Error> {
        let num = self.read_var_u64()?;
        Ok((num >> 1) as i64 ^ -((num & 1) as i64))
    }

    read_fn!(read_u128, u128, get_u128, 16);
    read_fn!(read_u128_le, u128, get_u128_le, 16);
    read_fn!(read_i128, i128, get_i128, 16);
    read_fn!(read_i128_le, i128, get_i128_le, 16);

    /// Reads an unsigned integer from the stream with a varying size
    /// indicated by the `size` parameter.
    pub fn read_uint(&mut self, size: usize) -> Result<u64, std::io::Error> {
        // todo: Check whether we should use `copy_nonoverlapping` or `self.get_uint`
        if can_read!(self, size) {
            let mut num = 0u64;
            let ptr_to = &mut num as *mut u64 as *mut u8;
            // we're not using for loops because they're slower
            unsafe {
                core::ptr::copy_nonoverlapping(self.buf.chunk().as_ptr(), ptr_to, size);
            }
            // increment the position
            self.buf.advance(size);
            return Ok(num.to_be());
        } else {
            return Err(Error::new(std::io::ErrorKind::UnexpectedEof, ERR_EOB));
        }
    }

    /// Reads an unsigned integer from the stream with a varying size in little endian
    /// indicated by the `size` parameter.
    pub fn read_uint_le(&mut self, size: usize) -> Result<u64, std::io::Error> {
        if can_read!(self, size) {
            let mut num = 0u64;
            let ptr_to = &mut num as *mut u64 as *mut u8;
            unsafe {
                core::ptr::copy_nonoverlapping(self.buf.chunk().as_ptr(), ptr_to, size);
            }
            // increment the position
            self.buf.advance(size);
            return Ok(num.to_le());
        } else {
            return Err(Error::new(std::io::ErrorKind::UnexpectedEof, ERR_EOB));
        }
    }

    pub fn read_int(&mut self, size: usize) -> Result<i64, std::io::Error> {
        if can_read!(self, size) {
            return Ok(self.buf.get_int(size));
        } else {
            return Err(Error::new(std::io::ErrorKind::UnexpectedEof, ERR_EOB));
        }
    }

    pub fn read_int_le(&mut self, size: usize) -> Result<i64, std::io::Error> {
        if can_read!(self, size) {
            return Ok(self.buf.get_int_le(size));
        } else {
            return Err(Error::new(std::io::ErrorKind::UnexpectedEof, ERR_EOB));
        }
    }

    pub fn read_bool(&mut self) -> Result<bool, std::io::Error> {
        if can_read!(self, 1) {
            return Ok(self.buf.get_u8() != 0);
        } else {
            return Err(Error::new(std::io::ErrorKind::UnexpectedEof, ERR_EOB));
        }
    }

    /// Reads a string from the stream.
    /// This is a reversable operation, meaning if it fails,
    /// the stream will be in the same state as before.
    pub fn read_string(&mut self) -> Result<String, std::io::Error> {
        // todo: Make this reversable
        let len = self.read_var_u64()?;
        if can_read!(self, len as usize) {
            let mut string = String::with_capacity(len as usize);
            unsafe {
                let v = string.as_mut_vec();
                v.set_len(len as usize);
                self.buf.copy_to_slice(&mut v[..]);
            }
            return Ok(string);
        } else {
            return Err(Error::new(std::io::ErrorKind::UnexpectedEof, ERR_EOB));
        }
    }

    pub fn as_slice(&self) -> &[u8] {
        self.buf.chunk()
    }
}

/// ByteWriter is a panic-free way to write bytes to a `BufMut` trait.
///
/// ## Example
/// A generic example of how to use the `ByteWriter` struct.
/// ```rust
/// use binary_utils::io::ByteWriter;
/// use binary_utils::io::ByteReader;
///
/// fn main() {
///    let mut writer = ByteWriter::new();
///    writer.write_string("Hello World!").unwrap();
///    writer.write_var_u32(65536).unwrap();
///    writer.write_u8(0).unwrap();
///
///    println!("Bytes: {:?}", writer.as_slice());
/// }
/// ```
///
/// `ByteWriter` also implements the `Into` trait to convert the `ByteWriter` into a `BytesMut` or `Bytes` structs.
/// ```rust
/// use binary_utils::io::ByteWriter;
/// use binary_utils::io::ByteReader;
///
/// fn main() {
///     let mut writer = ByteWriter::new();
///     writer.write_u8(1);
///     writer.write_u8(2);
///     writer.write_u8(3);
///
///     let mut reader: ByteReader = writer.into();
///     assert_eq!(reader.read_u8().unwrap(), 1);
///     assert_eq!(reader.read_u8().unwrap(), 2);
///     assert_eq!(reader.read_u8().unwrap(), 3);
/// }
/// ```
///
/// #### ByteWriter Implementation Notice
/// While most of the methods are reversable, some are not.
/// Meaning there is a chance that if you call a method in a edge case, it will corrupt the stream.
///
/// For example, `write_var_u32` is not reversable because we currently do not
/// allocate a buffer to store the bytes before writing them to the buffer.
/// While you should never encounter this issue, it is possible when you run out of memory.
/// This issue is marked as a todo, but is low priority.
pub struct ByteWriter {
    pub(crate) buf: BytesMut,
}

impl Into<BytesMut> for ByteWriter {
    fn into(self) -> BytesMut {
        self.buf
    }
}

impl Into<Bytes> for ByteWriter {
    fn into(self) -> Bytes {
        self.buf.freeze()
    }
}

impl Into<Vec<u8>> for ByteWriter {
    fn into(self) -> Vec<u8> {
        self.buf.to_vec()
    }
}

impl Into<VecDeque<u8>> for ByteWriter {
    fn into(self) -> VecDeque<u8> {
        self.buf.to_vec().into()
    }
}

impl From<IoSlice<'_>> for ByteWriter {
    fn from(slice: IoSlice) -> Self {
        let mut buf = BytesMut::with_capacity(slice.len());
        buf.put_slice(&slice);
        return Self { buf };
    }
}

impl From<&[u8]> for ByteWriter {
    fn from(slice: &[u8]) -> Self {
        let mut buf = BytesMut::with_capacity(slice.len());
        buf.put_slice(slice);
        return Self { buf };
    }
}

impl From<ByteReader> for ByteWriter {
    fn from(reader: ByteReader) -> Self {
        Self {
            buf: reader.buf.chunk().into(),
        }
    }
}

impl ByteWriter {
    pub fn new() -> Self {
        return Self {
            buf: BytesMut::new(),
        };
    }

    write_fn!(write_u8, u8, put_u8, 1);
    write_fn!(write_u16, u16, put_u16, 2);
    write_fn!(write_u16_le, u16, put_u16_le, 2);
    write_fn!(write_i16, i16, put_i16, 2);
    write_fn!(write_i16_le, i16, put_i16_le, 2);

    pub fn write_u24(&mut self, num: u32) -> Result<(), std::io::Error> {
        return self.write_uint(num.into(), 3);
    }

    pub fn write_u24_le(&mut self, num: u32) -> Result<(), std::io::Error> {
        return self.write_uint_le(num.into(), 3);
    }

    pub fn write_i24(&mut self, num: i32) -> Result<(), std::io::Error> {
        return self.write_int(num.into(), 3);
    }

    pub fn write_i24_le(&mut self, num: i32) -> Result<(), std::io::Error> {
        return self.write_int_le(num.into(), 3);
    }

    write_fn!(write_u32, u32, put_u32, 4);
    write_fn!(write_u32_le, u32, put_u32_le, 4);
    write_fn!(write_i32, i32, put_i32, 4);
    write_fn!(write_i32_le, i32, put_i32_le, 4);
    write_fn!(write_f32, f32, put_f32, 4);
    write_fn!(write_f32_le, f32, put_f32_le, 4);

    // todo: write_var_u32, write_var_i32 should be reversable and should not corrupt the stream on failure
    pub fn write_var_u32(&mut self, num: u32) -> Result<(), std::io::Error> {
        let mut x = num;
        while x >= 0x80 {
            self.write_u8((x as u8) | 0x80)?;
            x >>= 7;
        }
        self.write_u8(x as u8)?;
        return Ok(());
    }

    pub fn write_var_i32(&mut self, num: i32) -> Result<(), std::io::Error> {
        return if num < 0 {
            let num = num as u32;
            self.write_var_u32(!(num << 1))
        } else {
            let num = num as u32;
            self.write_var_u32(num << 1)
        };
        // let mut x = (num as u32) & u32::MAX;
        // for _ in (0..35).step_by(7) {
        //     if x >> 7 == 0 {
        //         self.write_u8(x as u8)?;
        //         return Ok(());
        //     } else {
        //         self.write_u8(((x & 0x7F) | 0x80) as u8)?;
        //         x >>= 7;
        //     }
        // }
        // return Err(Error::new(std::io::ErrorKind::InvalidData, ERR_VARINT_TOO_LONG));
    }

    write_fn!(write_u64, u64, put_u64, 8);
    write_fn!(write_u64_le, u64, put_u64_le, 8);
    write_fn!(write_i64, i64, put_i64, 8);
    write_fn!(write_i64_le, i64, put_i64_le, 8);
    write_fn!(write_f64, f64, put_f64, 8);
    write_fn!(write_f64_le, f64, put_f64_le, 8);

    pub fn write_var_u64(&mut self, num: u64) -> Result<(), std::io::Error> {
        let mut x = (num as u64) & u64::MAX;
        for _ in (0..70).step_by(7) {
            if x >> 7 == 0 {
                self.write_u8(x as u8)?;
                return Ok(());
            } else {
                self.write_u8(((x & 0x7F) | 0x80) as u8)?;
                x >>= 7;
            }
        }

        return Err(Error::new(
            std::io::ErrorKind::InvalidData,
            ERR_VARINT_TOO_LONG,
        ));
    }

    pub fn write_var_i64(&mut self, num: i64) -> Result<(), std::io::Error> {
        return if num < 0 {
            let num = num as u64;
            self.write_var_u64(!(num << 1))
        } else {
            let num = num as u64;
            self.write_var_u64(num << 1)
        };
    }

    pub fn write_uint(&mut self, num: u64, size: usize) -> Result<(), std::io::Error> {
        if can_write!(self, size) {
            self.buf.put_uint(num, size);
            return Ok(());
        } else {
            return Err(Error::new(std::io::ErrorKind::OutOfMemory, ERR_EOM));
        }
    }

    pub fn write_uint_le(&mut self, num: u64, size: usize) -> Result<(), std::io::Error> {
        if can_write!(self, size) {
            self.buf.put_uint_le(num, size);
            return Ok(());
        } else {
            return Err(Error::new(std::io::ErrorKind::OutOfMemory, ERR_EOM));
        }
    }

    pub fn write_int(&mut self, num: i64, size: usize) -> Result<(), std::io::Error> {
        if can_write!(self, size) {
            self.buf.put_int(num, size);
            return Ok(());
        } else {
            return Err(Error::new(std::io::ErrorKind::OutOfMemory, ERR_EOM));
        }
    }

    pub fn write_int_le(&mut self, num: i64, size: usize) -> Result<(), std::io::Error> {
        if can_write!(self, size) {
            self.buf.put_int_le(num, size);
            return Ok(());
        } else {
            return Err(Error::new(std::io::ErrorKind::OutOfMemory, ERR_EOM));
        }
    }

    pub fn write_slice(&mut self, slice: &[u8]) -> Result<(), std::io::Error> {
        if can_write!(self, slice.len()) {
            self.buf.put_slice(slice);
            return Ok(());
        } else {
            return Err(Error::new(std::io::ErrorKind::OutOfMemory, ERR_EOM));
        }
    }

    pub fn write_bool(&mut self, b: bool) -> Result<(), std::io::Error> {
        if can_write!(self, 1) {
            self.buf.put_u8(b as u8);
            return Ok(());
        } else {
            return Err(Error::new(std::io::ErrorKind::OutOfMemory, ERR_EOM));
        }
    }

    /// Write a string to the buffer
    /// The string is written as a var_u32 length followed by the bytes of the string.
    /// Uses <https://protobuf.dev/programming-guides/encoding/#length-types> for length encoding
    pub fn write_string(&mut self, string: &str) -> Result<(), std::io::Error> {
        // https://protobuf.dev/programming-guides/encoding/#length-types
        if can_write!(self, string.len()) {
            self.write_var_u32(string.len() as u32)?;
            self.buf.put_slice(string.as_bytes());
            return Ok(());
        } else {
            return Err(Error::new(std::io::ErrorKind::OutOfMemory, ERR_EOM));
        }
    }

    pub fn as_slice(&self) -> &[u8] {
        self.buf.chunk()
    }

    pub fn clear(&mut self) {
        self.buf.clear();
    }
}
