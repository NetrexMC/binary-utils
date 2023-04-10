use crate::io::{ByteReader, ByteWriter};

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
pub trait Reader<T> {
    /// Reads `Self` from a `ByteReader`.
    ///
    /// For automatic implementations, use `#[derive(BinaryDecoder]` macro.
    fn read(buf: &mut ByteReader) -> Result<T, std::io::Error>;
}

pub trait Writer {
    /// Writes `Self` to a `ByteWriter`.
    ///
    /// For automatic implementations, use `#[derive(BinaryEncoder]` macro.
    fn write(&self, buf: &mut ByteWriter) -> Result<(), std::io::Error>;

    /// This is a utility function to write `Self` to a `ByteWriter` without
    /// needing to create a `ByteWriter` first.
    fn init_write(&self) -> Result<ByteWriter, std::io::Error> {
        let mut buf = ByteWriter::new();
        self.write(&mut buf)?;
        Ok(buf)
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
    fn parse(&self) -> Result<Vec<u8>, crate::error::BinaryError>
    where
        T: Sized,
    {
        if let Ok(v) = self.init_write() {
            Ok(v.as_slice().to_vec())
        } else {
            Err(crate::error::BinaryError::RecoverableUnknown)
        }
    }

    /// Writes and unwraps `self` to the given buffer.
    ///
    /// ⚠️ This method is not fail safe, and will panic if result is Err.
    fn fparse(&self) -> Vec<u8>
    where
        T: Sized,
    {
        self.parse().unwrap()
    }

    /// Reads `self` from the given buffer.
    fn compose(source: &[u8], position: &mut usize) -> Result<T, crate::error::BinaryError>
    where
        T: Sized,
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
        T: Sized,
    {
        Self::compose(source, position).unwrap()
    }
}
