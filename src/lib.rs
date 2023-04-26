/// Provides a panic-free way to read and write binary data.
/// All of the methods within this module follow the protobuf specification at <https://protobuf.dev/programming-guides/encoding/>.
///
/// ## Example
/// ```no_run
/// use binary_utils::io::ByteReader;
///
/// const VARINT: &[u8] = &[255, 255, 255, 255, 7]; // 2147483647
/// fn main() {
///     let mut buf = ByteReader::from(&VARINT[..]);
///     assert_eq!(buf.read_var_u32().unwrap(), 2147483647);
/// }
/// ```
pub mod interfaces;
/// Provides a derive macro that implements `::binary_utils::interfaces::Reader<T>` and `::binary_utils::interfaces::Writer<T>`.
/// 
pub use codegen::*;
/// The io module contains implementations of these traits for `bytes::Buf` and `bytes::BufMut`.
///
/// Example:
/// ```no_run
/// use binary_utils::io::ByteReader;
/// use bytes::{Buf, BufMut, BytesMut, Bytes};
///
/// fn main() {
///    const VARINT: &[u8] = &[255, 255, 255, 255, 7]; // 2147483647
///    let mut buf = ByteReader::from(&VARINT[..]);
///    assert_eq!(buf.read_var_u32().unwrap(), 2147483647);
/// }
/// ```
pub mod io;
pub mod pool;
/// This is a legacy module that will be removed in the future.
/// This module has been replaced in favor of `std::io::Error`.
///
/// # This module is deprecated
pub mod error {
    /// An enum consisting of a Binary Error
    /// (recoverable)
    #[derive(Debug, PartialEq)]
    pub enum BinaryError {
        /// Offset is out of bounds
        ///
        /// **Tuple Values:**
        /// - `usize` = Given Offset.
        /// - `usize` = Stream length.
        /// - `&'static str` = Message to add on to the error.
        OutOfBounds(usize, usize, &'static str),

        /// Similar to `OutOfBounds` except it means;
        /// the stream tried to read more than possible.
        ///
        /// **Tuple Values:**
        /// - `usize` = Stream length.
        EOF(usize),

        /// A known error that was recoverable to safely proceed the stack.
        RecoverableKnown(String),

        /// An unknown error occurred, but it wasn't critical,
        /// we can safely proceed on the stack.
        RecoverableUnknown,
    }

    impl BinaryError {
        pub fn get_message(&self) -> String {
            match self {
                Self::OutOfBounds(offset, length, append) => {
                    format!("Offset {} out of range for a buffer size with: {}. {}", offset, length, append)
                },
                Self::EOF(length) => format!("Buffer reached End Of File at offset: {}", length),
                Self::RecoverableKnown(msg) => msg.clone(),
                Self::RecoverableUnknown => "An interruption occurred when performing a binary operation, however this error was recovered safely.".to_string()
            }
        }
    }

    impl From<std::io::Error> for BinaryError {
        fn from(_error: std::io::Error) -> Self {
            Self::RecoverableUnknown
        }
    }

    impl std::fmt::Display for BinaryError {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "{}", self.get_message())
        }
    }
}

pub use interfaces::Streamable;
