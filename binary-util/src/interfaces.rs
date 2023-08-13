// todo: remove this in 4.0.0
#![allow(deprecated)]

use std::net::{Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};

use crate::io::{ByteReader, ByteWriter};
use crate::types::{i24, u24, vari32, vari64, varu32, varu64, BE, LE};

macro_rules! impl_reader {
    ($(LE<$t:ty>, $method:ident),*) => {
        $(
            impl Reader<LE<$t>> for LE<$t> {
                fn read(buf: &mut ByteReader) -> Result<LE<$t>, std::io::Error> {
                    buf.$method().map(LE::new)
                }
            }
        )*
    };
    ($(BE<$t:ty>, $method:ident),*) => {
        $(
            impl Reader<BE<$t>> for BE<$t> {
                fn read(buf: &mut ByteReader) -> Result<BE<$t>, std::io::Error> {
                    buf.$method().map(BE::new)
                }
            }
        )*
    };
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
    ($(LE<$t:ty>, $method:ident),*) => {
        $(
            impl Writer for LE<$t> {
                fn write(&self, buf: &mut ByteWriter) -> Result<(), std::io::Error> {
                    buf.$method(**self)
                }
            }
        )*
    };
    ($(BE<$t:ty>, $method:ident),*) => {
        $(
            impl Writer for BE<$t> {
                fn write(&self, buf: &mut ByteWriter) -> Result<(), std::io::Error> {
                    buf.$method(**self)
                }
            }
        )*
    };
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

macro_rules! impl_streamable {
    ($($t:ty),*) => {
        $(
            impl Streamable<$t> for $t {}
        )*
    };
}

/// Allows you to read from a `ByteReader` without needing to know the type.
///
/// ```ignore
/// use binary_util::io::{ByteReader, Reader};
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

    /// Reads `Self` from a `&[u8]`.
    ///
    /// This is a convenience method that creates a `ByteReader` from the slice and calls `read`.
    fn read_from_slice(buf: &[u8]) -> Result<Output, std::io::Error> {
        let mut reader = ByteReader::from(buf);
        Self::read(&mut reader)
    }
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
impl_reader!(
    LE<u16>,
    read_u16_le,
    LE<u32>,
    read_u32_le,
    LE<u64>,
    read_u64_le,
    LE<u128>,
    read_u128_le,
    LE<i16>,
    read_i16_le,
    LE<i32>,
    read_i32_le,
    LE<i64>,
    read_i64_le,
    LE<i128>,
    read_i128_le,
    LE<f32>,
    read_f32_le,
    LE<f64>,
    read_f64_le
);

// big endian explicit implementations on primitive types.
impl_reader!(
    BE<u16>,
    read_u16,
    BE<u32>,
    read_u32,
    BE<u64>,
    read_u64,
    BE<u128>,
    read_u128,
    BE<i16>,
    read_i16,
    BE<i32>,
    read_i32,
    BE<i64>,
    read_i64,
    BE<i128>,
    read_i128,
    BE<f32>,
    read_f32,
    BE<f64>,
    read_f64
);

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

impl Reader<varu32> for varu32 {
    fn read(buf: &mut ByteReader) -> Result<varu32, std::io::Error> {
        Ok(varu32(buf.read_var_u32()?))
    }
}

impl Reader<vari32> for vari32 {
    fn read(buf: &mut ByteReader) -> Result<vari32, std::io::Error> {
        Ok(vari32(buf.read_var_i32()?))
    }
}

impl Reader<varu64> for varu64 {
    fn read(buf: &mut ByteReader) -> Result<varu64, std::io::Error> {
        Ok(varu64(buf.read_var_u64()?))
    }
}

impl Reader<vari64> for vari64 {
    fn read(buf: &mut ByteReader) -> Result<vari64, std::io::Error> {
        Ok(vari64(buf.read_var_i64()?))
    }
}

impl Reader<LE<u24>> for LE<u24> {
    fn read(buf: &mut ByteReader) -> Result<LE<u24>, std::io::Error> {
        Ok(LE(buf.read_u24()?.into()))
    }
}

impl Reader<BE<u24>> for BE<u24> {
    fn read(buf: &mut ByteReader) -> Result<BE<u24>, std::io::Error> {
        Ok(BE(buf.read_u24()?.into()))
    }
}

impl Reader<LE<i24>> for LE<i24> {
    fn read(buf: &mut ByteReader) -> Result<LE<i24>, std::io::Error> {
        Ok(LE(buf.read_i24()?.into()))
    }
}

impl Reader<BE<i24>> for BE<i24> {
    fn read(buf: &mut ByteReader) -> Result<BE<i24>, std::io::Error> {
        Ok(BE(buf.read_i24()?.into()))
    }
}

impl Reader<u24> for u24 {
    fn read(buf: &mut ByteReader) -> Result<u24, std::io::Error> {
        Ok(u24(buf.read_u24()?))
    }
}

impl Reader<i24> for i24 {
    fn read(buf: &mut ByteReader) -> Result<i24, std::io::Error> {
        Ok(i24(buf.read_i24()?))
    }
}

/// Allows you to write to a `ByteWriter` without needing to know the type.
///
/// ```ignore
/// use binary_util::io::{ByteWriter, Writer};
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
    u24,
    write_u24,
    i24,
    write_i24,
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

// little endian implementations on primitive types.
impl_writer!(
    LE<u16>,
    write_u16_le,
    LE<u32>,
    write_u32_le,
    LE<u64>,
    write_u64_le,
    LE<u128>,
    write_u128_le,
    LE<i16>,
    write_i16_le,
    LE<i32>,
    write_i32_le,
    LE<i64>,
    write_i64_le,
    LE<i128>,
    write_i128_le,
    LE<f32>,
    write_f32_le,
    LE<f64>,
    write_f64_le
);

// big endian explicit implementations on primitive types.
impl_writer!(
    BE<u16>,
    write_u16,
    BE<u32>,
    write_u32,
    BE<u64>,
    write_u64,
    BE<u128>,
    write_u128,
    BE<i16>,
    write_i16,
    BE<i32>,
    write_i32,
    BE<i64>,
    write_i64,
    BE<i128>,
    write_i128,
    BE<f32>,
    write_f32,
    BE<f64>,
    write_f64
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

impl Writer for LE<u24> {
    fn write(&self, buf: &mut ByteWriter) -> Result<(), std::io::Error> {
        buf.write_u24_le(self.0)
    }
}

impl Writer for BE<u24> {
    fn write(&self, buf: &mut ByteWriter) -> Result<(), std::io::Error> {
        buf.write_u24(self.0)
    }
}

impl Writer for LE<i24> {
    fn write(&self, buf: &mut ByteWriter) -> Result<(), std::io::Error> {
        buf.write_i24_le(self.0)
    }
}

impl Writer for BE<i24> {
    fn write(&self, buf: &mut ByteWriter) -> Result<(), std::io::Error> {
        buf.write_i24(self.0)
    }
}

impl Writer for varu32 {
    fn write(&self, buf: &mut ByteWriter) -> Result<(), std::io::Error> {
        buf.write_var_u32(self.0)
    }
}

impl Writer for varu64 {
    fn write(&self, buf: &mut ByteWriter) -> Result<(), std::io::Error> {
        buf.write_var_u64(self.0)
    }
}

impl Writer for vari32 {
    fn write(&self, buf: &mut ByteWriter) -> Result<(), std::io::Error> {
        buf.write_var_i32(self.0)
    }
}

impl Writer for vari64 {
    fn write(&self, buf: &mut ByteWriter) -> Result<(), std::io::Error> {
        buf.write_var_i64(self.0)
    }
}

///
/// __**This trait exists only for backwards compatibility.**__
///
/// If you wish to read and write from a `ByteReader` or `ByteWriter`,
/// use the `Reader` and `Writer` traits.
///
/// ### New Implementation Example
/// ```ignore
/// use binary_util::io::{ByteReader, ByteWriter};
/// use binary_util::interfaces::{Reader, Writer};
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
/// use binary_util::{Streamable, error::BinaryError};
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
/// <p style="background:rgba(255,181,77,0.16);padding:0.75em;border-left: 2px solid orange;">
///     <strong>Warning:</strong> This module is deprecated and will be removed in <strong>v0.4.0</strong>.
/// </p>
#[deprecated(
    since = "0.3.0",
    note = "This module is deprecated and will be removed in v0.4.0. Use the `Reader` and `Writer` traits instead."
)]
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

impl_streamable!(u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, f32, f64, bool, char, String);
