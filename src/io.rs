use std::io::{self, Result};

use byteorder::ByteOrder;
use byteorder::ReadBytesExt;
use byteorder::WriteBytesExt;

use crate::*;
pub trait BinaryReader: ReadBytesExt + Clone {
    /// Reads a `u32` variable length integer from the stream.
    #[inline]
    fn read_u32_varint(&mut self) -> Result<VarInt<u32>> {
        // this is EXTREMELY hacky!
        let mut ref_to = self.clone();
        let most = &mut [0; 5];
        let four = &mut [0; 4];
        let three = &mut [0; 3];
        let two = &mut [0; 2];

        if let Err(_) = ref_to.read_exact(&mut most[..]) {
            // there was an error with the buffer size.
            // we're going to incrementally, decrease the required size until 0
            if let Ok(_) = ref_to.read_exact(&mut four[..]) {
                if let Ok(var) = VarInt::<u32>::compose(&four[..], &mut 0) {
                    return Ok(var);
                } else {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "Could not read varint",
                    ));
                }
            } else if let Ok(_) = ref_to.read_exact(&mut three[..]) {
                if let Ok(var) = VarInt::<u32>::compose(&three[..], &mut 0) {
                    return Ok(var);
                } else {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "Could not read varint",
                    ));
                }
            } else if let Ok(_) = ref_to.read_exact(&mut two[..]) {
                if let Ok(var) = VarInt::<u32>::compose(&two[..], &mut 0) {
                    return Ok(var);
                } else {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "Could not read varint",
                    ));
                }
            } else {
                return Err(io::Error::new(
                    io::ErrorKind::UnexpectedEof,
                    "unable to read varint",
                ));
            }
        } else {
            if let Ok(var) = VarInt::<u32>::compose(&most[..], &mut 0) {
                return Ok(var);
            } else {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Could not read varint",
                ));
            }
        }
    }

    /// Reads a `u64` variable length integer from the stream.
    #[inline]
    fn read_u64_varint(&mut self) -> Result<VarInt<u64>> {
        // this is EXTREMELY hacky!
        let mut current_buffer: Vec<u8> = Vec::new();

        while current_buffer.len() <= 4 {
            // read a byte!
            let byte = self.read_u8()?;
            current_buffer.push(byte);

            // try making a var_int from the current buffer
            if let Ok(i) = VarInt::<u64>::compose(&current_buffer[..], &mut 0) {
                return Ok(i);
            }
        }

        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Could not read varint",
        ));
    }

    /// Reads a string sized by a `u16`.
    #[inline]
    fn read_string<Endianess>(&mut self) -> Result<String>
    where
        Endianess: ByteOrder,
    {
        let t = self.clone();
        let length = self.read_u16::<Endianess>()?;
        let mut string_data = Vec::new();
        t.take(length as u64).read_to_end(&mut string_data)?;
        self.read(&mut string_data[..])?;
        Ok(unsafe { String::from_utf8_unchecked(string_data.to_vec()) })
    }

    /// Reads a string sized by a `u32`.
    #[inline]
    fn read_string_u32<Endianess>(&mut self) -> Result<String>
    where
        Endianess: ByteOrder,
    {
        let t = self.clone();
        let length = self.read_u32::<Endianess>()?;
        let mut string_data = Vec::new();
        t.take(length as u64).read_to_end(&mut string_data)?;
        self.read(&mut string_data[..])?;
        Ok(unsafe { String::from_utf8_unchecked(string_data.to_vec()) })
    }

    /// Reads a string that will be sized by a u64.
    /// ```rust ignore
    /// use binary_utils::{VarInt, io::BinaryWriter};
    ///
    /// let mut stream = Vec::new();
    /// stream.write_string_u64::<LittleEndian>("Hello World").unwrap();
    /// ```
    #[inline]
    fn read_string_u64<Endianess>(&mut self) -> Result<String>
    where
        Endianess: ByteOrder,
    {
        let t = self.clone();
        let length = self.read_u64::<Endianess>()?;
        let mut string_data = Vec::new();
        t.take(length as u64).read_to_end(&mut string_data)?;
        self.read(&mut string_data[..])?;
        Ok(unsafe { String::from_utf8_unchecked(string_data.to_vec()) })
    }

    // /// Reads an array to the stream. This array will
    // /// be sized by a short (u16) with the contents being
    // /// a vector of `Streamable` types.
    // /// ```rust ignore
    // /// use binary_utils::{Streamable, io::BinaryWriter};
    // /// let my_vec: Vec<String> = vec!["Hello", "World"];
    // /// let mut stream = Vec::new();
    // /// stream.write_array(my_vec).unwrap();
    // /// ```
    // fn write_array<Endianess, T>(&mut self, value: Vec<T>) -> Result<()>
    // where
    //     T: Sized + Streamable,
    //     Endianess: ByteOrder,
    // {
    //     self.write_u16::<Endianess>(value.len() as u16)?;
    //     for x in value {
    //         let res = x.parse();
    //         if let Ok(v) = res {
    //             self.write_all(&v[..])?;
    //         } else if let Err(e) = res {
    //             return Err(io::Error::new(
    //                 io::ErrorKind::InvalidData,
    //                 format!("Array Item could not be parsed due to a Binary Error: {}", e),
    //             ));
    //         } else {
    //             return Err(io::Error::new(
    //                 io::ErrorKind::InvalidData,
    //                 "Array Item could not be parsed due to an unknown error while parsing.",
    //             ));
    //         }
    //     }
    //     Ok(())
    // }

    // /// Reads an array to the stream. This array will
    // /// be sized by a `u32` with the contents being
    // /// a vector of `Streamable` types.
    // fn write_array_u32<Endianess, T>(&mut self, value: Vec<T>) -> Result<()>
    // where
    //     T: Streamable,
    //     Endianess: ByteOrder,
    // {
    //     self.write_u32::<Endianess>(value.len() as u32)?;
    //     for x in value {
    //         let res = x.parse();
    //         if let Ok(v) = res {
    //             self.write_all(&v[..])?;
    //         } else if let Err(e) = res {
    //             return Err(io::Error::new(
    //                 io::ErrorKind::InvalidData,
    //                 format!("Array Item could not be parsed due to a Binary Error: {}", e),
    //             ));
    //         } else {
    //             return Err(io::Error::new(
    //                 io::ErrorKind::InvalidData,
    //                 "Array Item could not be parsed due to an unknown error while parsing.",
    //             ));
    //         }
    //     }
    //     Ok(())
    // }

    // /// Reads an array to the stream. This array will
    // /// be sized by a `u64` with the contents being
    // /// a vector of `Streamable` types.
    // fn write_array_u64<Endianess, T>(&mut self, value: Vec<T>) -> Result<()>
    // where
    //     T: Streamable,
    //     Endianess: ByteOrder,
    // {
    //     self.write_u64::<Endianess>(value.len() as u64)?;
    //     for x in value {
    //         let res = x.parse();
    //         if let Ok(v) = res {
    //             self.write_all(&v[..])?;
    //         } else if let Err(e) = res {
    //             return Err(io::Error::new(
    //                 io::ErrorKind::InvalidData,
    //                 format!("Array Item could not be parsed due to a Binary Error: {}", e),
    //             ));
    //         } else {
    //             return Err(io::Error::new(
    //                 io::ErrorKind::InvalidData,
    //                 "Array Item could not be parsed due to an unknown error while parsing.",
    //             ));
    //         }
    //     }
    //     Ok(())
    // }

    // /// Reads a socket address fron the stream.
    // #[inline]
    // fn write_socket_addr(&mut self, address: SocketAddr) -> Result<()> {
    //     if let Ok(v) = address.parse() {
    //         self.write_all(&v[..])?;
    //         return Ok(());
    //     } else {
    //         return Err(io::Error::new(
    //             io::ErrorKind::InvalidData,
    //             "Invalid Socket Address.",
    //         ));
    //     }
    // }

    // /// Writes a bool to the stream.
    // /// ```rust ignore
    // /// use binary_utils::io::BinaryWriter;
    // ///
    // /// let mut stream = Vec::new();
    // /// stream.write_bool(true)
    // /// ```
    // #[inline]
    // fn write_bool(&mut self, value: bool) -> Result<()> {
    //     self.write_u8(if value { 1 } else { 0 })?;
    //     Ok(())
    // }
}

/// All types that implement `Write` get methods defined in `BinaryWriter`
/// for free.
impl<R: io::Read + Clone + ?Sized> BinaryReader for R {}

pub trait BinaryWriter: WriteBytesExt {
    /// Writes a `u32` variable length integer to the stream.
    /// ```rust ignore
    /// use binary_utils::{VarInt, io::BinaryWriter};
    ///
    /// let mut stream = Vec::new();
    /// stream.write_u32_varint(VarInt::<u32>(0x12345678)).unwrap();
    /// ```
    #[inline]
    fn write_u32_varint(&mut self, value: VarInt<u32>) -> Result<()> {
        if let Ok(v) = value.parse() {
            self.write_all(&v[..])?;
        } else {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "VarInt is too big to be written",
            ));
        }

        Ok(())
    }

    /// Writes a `u64` variable length integer to the stream.
    #[inline]
    fn write_u64_varint(&mut self, value: VarInt<u64>) -> Result<()> {
        if let Ok(v) = value.parse() {
            self.write_all(&v[..])?;
        } else {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "VarInt is too big to be written",
            ));
        }

        Ok(())
    }

    /// Writes a string sized by a `u16`.
    #[inline]
    fn write_string<Str, Endianess>(&mut self, value: Str) -> Result<()>
    where
        Str: Into<String>,
        Endianess: ByteOrder,
    {
        let value: String = value.into();
        self.write_u16::<Endianess>(value.len() as u16)?;
        self.write_all(value.as_bytes())?;
        Ok(())
    }

    /// Writes a string sized by a `u32`.
    #[inline]
    fn write_string_u32<Str, Endianess>(&mut self, value: Str) -> Result<()>
    where
        Str: Into<String>,
        Endianess: ByteOrder,
    {
        let value: String = value.into();
        self.write_u32::<Endianess>(value.len() as u32)?;
        self.write_all(value.as_bytes())?;
        Ok(())
    }

    /// Writes a string that will be sized by a u64.
    /// ```rust ignore
    /// use binary_utils::{VarInt, io::BinaryWriter};
    ///
    /// let mut stream = Vec::new();
    /// stream.write_string_u64::<LittleEndian>("Hello World").unwrap();
    /// ```
    #[inline]
    fn write_string_u64<Str, Endianess>(&mut self, value: Str) -> Result<()>
    where
        Str: Into<String>,
        Endianess: ByteOrder,
    {
        let value: String = value.into();
        self.write_u64::<Endianess>(value.len() as u64)?;
        self.write_all(value.as_bytes())?;
        Ok(())
    }

    /// Writes an array to the stream. This array will
    /// be sized by a short (u16) with the contents being
    /// a vector of `Streamable` types.
    /// ```rust ignore
    /// use binary_utils::{Streamable, io::BinaryWriter};
    /// let my_vec: Vec<String> = vec!["Hello", "World"];
    /// let mut stream = Vec::new();
    /// stream.write_array(my_vec).unwrap();
    /// ```
    fn write_array<Endianess, T>(&mut self, value: Vec<T>) -> Result<()>
    where
        T: Sized + Streamable,
        Endianess: ByteOrder,
    {
        self.write_u16::<Endianess>(value.len() as u16)?;
        for x in value {
            let res = x.parse();
            if let Ok(v) = res {
                self.write_all(&v[..])?;
            } else if let Err(e) = res {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!(
                        "Array Item could not be parsed due to a Binary Error: {}",
                        e
                    ),
                ));
            } else {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Array Item could not be parsed due to an unknown error while parsing.",
                ));
            }
        }
        Ok(())
    }

    /// Writes an array to the stream. This array will
    /// be sized by a `u32` with the contents being
    /// a vector of `Streamable` types.
    fn write_array_u32<Endianess, T>(&mut self, value: Vec<T>) -> Result<()>
    where
        T: Streamable,
        Endianess: ByteOrder,
    {
        self.write_u32::<Endianess>(value.len() as u32)?;
        for x in value {
            let res = x.parse();
            if let Ok(v) = res {
                self.write_all(&v[..])?;
            } else if let Err(e) = res {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!(
                        "Array Item could not be parsed due to a Binary Error: {}",
                        e
                    ),
                ));
            } else {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Array Item could not be parsed due to an unknown error while parsing.",
                ));
            }
        }
        Ok(())
    }

    /// Writes an array to the stream. This array will
    /// be sized by a `u64` with the contents being
    /// a vector of `Streamable` types.
    fn write_array_u64<Endianess, T>(&mut self, value: Vec<T>) -> Result<()>
    where
        T: Streamable,
        Endianess: ByteOrder,
    {
        self.write_u64::<Endianess>(value.len() as u64)?;
        for x in value {
            let res = x.parse();
            if let Ok(v) = res {
                self.write_all(&v[..])?;
            } else if let Err(e) = res {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!(
                        "Array Item could not be parsed due to a Binary Error: {}",
                        e
                    ),
                ));
            } else {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Array Item could not be parsed due to an unknown error while parsing.",
                ));
            }
        }
        Ok(())
    }

    /// Writes a socket addres to the stream.
    #[inline]
    fn write_socket_addr(&mut self, address: SocketAddr) -> Result<()> {
        if let Ok(v) = address.parse() {
            self.write_all(&v[..])?;
            return Ok(());
        } else {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid Socket Address.",
            ));
        }
    }

    /// Writes a bool to the stream.
    /// ```rust ignore
    /// use binary_utils::io::BinaryWriter;
    ///
    /// let mut stream = Vec::new();
    /// stream.write_bool(true)
    /// ```
    #[inline]
    fn write_bool(&mut self, value: bool) -> Result<()> {
        self.write_u8(if value { 1 } else { 0 })?;
        Ok(())
    }
}

/// All types that implement `Write` get methods defined in `BinaryWriter`
/// for free.
impl<W: io::Write + ?Sized> BinaryWriter for W {}
