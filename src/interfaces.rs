use std::net::{Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};

use crate::io::{ByteReader, ByteWriter};

pub type LE<T> = std::num::Wrapping<T>;

macro_rules! impl_reader {
    ($($t:ty, $method: tt),*) => {
        $(
            impl Reader<$t> for $t {
                fn read(buf: &mut ByteReader) -> Result<$t, std::io::Error> {
                    buf.$method()
                }
            }
        )*
    };
}

macro_rules! impl_writer {
    ($($t:ty, $method: tt),*) => {
        $(
            impl Writer for $t {
                fn write(&self, buf: &mut ByteWriter) -> Result<(), std::io::Error> {
                    buf.$method(*self)
                }
            }
        )*
    };
}

/// Allows you to read from a `ByteReader` without needing to know the type.
///
/// ```ignore
/// use binary_utils::io::{ByteReader, Reader};
///
/// pub struct MyStruct {
///    pub a: u8,
///    pub b: u8
/// }
///
/// impl Reader for MyStruct {
///     fn read(&self, buf: &mut ByteReader) -> Result<Self, std::io::Error> {
///        let a = buf.read_u8()?;
///        let b = buf.read_u8()?;
///        Ok(Self { a, b })
///     }
/// }
/// ```
pub trait Reader<Output> {
    /// Reads `Self` from a `ByteReader`.
    ///
    /// For automatic implementations, use the `#[derive(BinaryIo)]` macro.
    fn read(buf: &mut ByteReader) -> Result<Output, std::io::Error>;
}

// default implementations on primitive types.
impl_reader!(
    u8,
    read_u8,
    i8,
    read_i8,
    u16,
    read_u16,
    i16,
    read_i16,
    u32,
    read_u32,
    i32,
    read_i32,
    u64,
    read_u64,
    i64,
    read_i64,
    u128,
    read_u128,
    i128,
    read_i128,
    f32,
    read_f32,
    f64,
    read_f64,
    bool,
    read_bool,
    char,
    read_char,
    String,
    read_string
);

// little endian implementations on primitive types.
// impl_reader!(
//     LE<u16>, read_u16_le,
//     LE<u32>, read_u32_le,
//     LE<u64>, read_u64_le,
//     LE<u128>, read_u128_le,
//     LE<i16>, read_i16_le,
//     LE<i32>, read_i32_le,
//     LE<i64>, read_i64_le,
//     LE<i128>, read_i128_le,
//     LE<f32>, read_f32_le,
//     LE<f64>, read_f64_le
// );

impl<T> Reader<Vec<T>> for Vec<T>
where
    T: Reader<T> + Sized,
{
    fn read(buf: &mut ByteReader) -> Result<Vec<T>, std::io::Error> {
        let len = buf.read_var_u32()?;
        let mut vec = Vec::with_capacity(len as usize);
        for _ in 0..len {
            vec.push(T::read(buf)?);
        }
        Ok(vec)
    }
}

impl<T> Reader<Option<T>> for Option<T>
where
    T: Reader<T> + Sized,
{
    fn read(buf: &mut ByteReader) -> Result<Option<T>, std::io::Error> {
        let is_some = buf.read_bool()?;
        if is_some {
            Ok(Some(T::read(buf)?))
        } else {
            Ok(None)
        }
    }
}

impl Reader<SocketAddr> for SocketAddr {
    fn read(buf: &mut ByteReader) -> Result<SocketAddr, std::io::Error> {
        match buf.read_u8()? {
            4 => {
                let parts = (
                    buf.read_u8()?,
                    buf.read_u8()?,
                    buf.read_u8()?,
                    buf.read_u8()?,
                );
                let port = buf.read_u16()?;
                Ok(SocketAddr::V4(SocketAddrV4::new(
                    Ipv4Addr::new(parts.0, parts.1, parts.2, parts.3),
                    port,
                )))
            }
            6 => {
                let _family = buf.read_u16()?;
                let port = buf.read_u16()?;
                let flow = buf.read_u32()?;
                let parts = (
                    buf.read_u16()?,
                    buf.read_u16()?,
                    buf.read_u16()?,
                    buf.read_u16()?,
                    buf.read_u16()?,
                    buf.read_u16()?,
                    buf.read_u16()?,
                    buf.read_u16()?,
                );
                let address = Ipv6Addr::new(
                    parts.0, parts.1, parts.2, parts.3, parts.4, parts.5, parts.6, parts.7,
                );
                let scope = buf.read_u32()?;
                Ok(SocketAddr::V6(SocketAddrV6::new(
                    address, port, flow, scope,
                )))
            }
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Invalid IP version",
            )),
        }
    }
}

/// Allows you to write to a `ByteWriter` without needing to know the type.
///
/// ```ignore
/// use binary_utils::io::{ByteWriter, Writer};
///
/// pub struct MyStruct {
///   pub a: u8,
///   pub b: u8
/// }
///
/// impl Writer for MyStruct {
///     fn write(&self, buf: &mut ByteWriter) -> Result<(), std::io::Error> {
///         buf.write_u8(self.a)?;
///         buf.write_u8(self.b)?;
///         Ok(());
///     }
/// }
/// ```
pub trait Writer {
    /// Writes `Self` to a `ByteWriter`.
    ///
    /// For automatic implementations, use `#[derive(BinaryEncoder]` macro.
    fn write(&self, buf: &mut ByteWriter) -> Result<(), std::io::Error>;

    /// This is a utility function to write `Self` to a `ByteWriter` without
    /// needing to create a `ByteWriter` first.
    fn write_to_bytes(&self) -> Result<ByteWriter, std::io::Error> {
        let mut buf = ByteWriter::new();
        self.write(&mut buf)?;
        Ok(buf)
    }
}

// default implementations on primitive types.
impl_writer!(
    u8,
    write_u8,
    i8,
    write_i8,
    u16,
    write_u16,
    i16,
    write_i16,
    u32,
    write_u32,
    i32,
    write_i32,
    u64,
    write_u64,
    i64,
    write_i64,
    u128,
    write_u128,
    i128,
    write_i128,
    f32,
    write_f32,
    f64,
    write_f64,
    bool,
    write_bool,
    &str,
    write_string
);

impl Writer for String {
    fn write(&self, buf: &mut ByteWriter) -> Result<(), std::io::Error> {
        buf.write_string(self)
    }
}

impl Writer for char {
    fn write(&self, buf: &mut ByteWriter) -> Result<(), std::io::Error> {
        buf.write_char(*self)
    }
}

impl<T> Writer for Vec<T>
where
    T: Writer + Sized,
{
    fn write(&self, buf: &mut ByteWriter) -> Result<(), std::io::Error> {
        buf.write_var_u32(self.len() as u32)?;
        for item in self {
            item.write(buf)?;
        }
        Ok(())
    }
}

impl<T> Writer for Option<T>
where
    T: Writer + Sized,
{
    fn write(&self, buf: &mut ByteWriter) -> Result<(), std::io::Error> {
        match self {
            Some(item) => {
                buf.write_bool(true)?;
                item.write(buf)?;
            }
            None => {
                buf.write_bool(false)?;
            }
        }
        Ok(())
    }
}

impl Writer for SocketAddr {
    fn write(&self, buf: &mut ByteWriter) -> Result<(), std::io::Error> {
        match self {
            SocketAddr::V4(addr) => {
                buf.write_u8(4)?;
                buf.write(&addr.ip().octets())?;
                buf.write_u16(addr.port())?;
            }
            SocketAddr::V6(addr) => {
                buf.write_u8(6)?;
                // family (unused by rust)
                buf.write_u16(0)?;
                // port
                buf.write_u16(addr.port())?;
                // flow
                buf.write_u32(addr.flowinfo())?;
                // address eg: 0:0:0:0:0:ffff:7f00:1
                buf.write(&addr.ip().octets())?;
                // scope
                buf.write_u32(addr.scope_id())?;
            }
        }
        Ok(())
    }
}
/// ## Deprecated
/// __**This trait exists only for backwards compatibility.**__
///
/// If you wish to read and write from a `ByteReader` or `ByteWriter`,
/// use the `Reader` and `Writer` traits.
///
/// ### New Implementation Example
/// ```ignore
/// use binary_utils::io::{ByteReader, ByteWriter};
/// use binary_utils::interfaces::{Reader, Writer};
///
/// pub struct MyStruct;
///
/// impl Reader for MyStruct;
/// impl Writer for MyStruct;
/// ```
///
/// ## `Streamable`
/// A trait to parse and unparse header structs from a given buffer.
///
/// ```ignore
/// use binary_utils::{Streamable, error::BinaryError};
///
/// struct Foo {
///     bar: u8,
///     foo_bar: u16
/// }
/// impl Streamable for Foo {
///     fn parse(&self) -> Result<Vec<u8>, BinaryError> {
///         use std::io::Write;
///         let mut stream = Vec::<u8>::new();
///         stream.write_all(&self.bar.parse()?[..])?;
///         stream.write_all(&self.bar.parse()?[..])?;
///         Ok(stream)
///     }
///
///     fn compose(source: &[u8], position: &mut usize) -> Result<Self, BinaryError> {
///         // Streamable is implemented for all primitives, so we can
///         // just use this implementation to read our properties.
///         Ok(Self {
///             bar: u8::compose(&source, position)?,
///             foo_bar: u16::compose(&source, position)?
///         })
///     }
/// }
/// ```
pub trait Streamable<T>: Reader<T> + Writer {
    /// Writes `self` to the given buffer.
    fn parse(&self) -> Result<Vec<u8>, crate::error::BinaryError> {
        if let Ok(v) = self.write_to_bytes() {
            Ok(v.as_slice().to_vec())
        } else {
            Err(crate::error::BinaryError::RecoverableUnknown)
        }
    }

    /// Writes and unwraps `self` to the given buffer.
    ///
    /// ⚠️ This method is not fail safe, and will panic if result is Err.
    fn fparse(&self) -> Vec<u8> {
        self.parse().unwrap()
    }

    /// Reads `self` from the given buffer.
    fn compose(source: &[u8], position: &mut usize) -> Result<T, crate::error::BinaryError>
    where
        Self: Sized,
    {
        let mut reader = ByteReader::from(&source[*position..]);
        if let Ok(v) = Self::read(&mut reader) {
            Ok(v)
        } else {
            Err(crate::error::BinaryError::RecoverableUnknown)
        }
    }

    /// Reads and unwraps `self` from the given buffer.
    ///
    /// ⚠️ This method is not fail safe, and will panic if result is Err.
    fn fcompose(source: &[u8], position: &mut usize) -> T
    where
        Self: Sized,
    {
        Self::compose(source, position).unwrap()
    }
}
