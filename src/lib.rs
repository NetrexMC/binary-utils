//! # Binary Util
//! A panic-free way to read and write binary data over the wire.
//!
//! BinaryUtils provides the following features:
//! * [`binary_util::io`], to read and write to streams manually.
//! * [`binary_util::interfaces`], to allow automation of reading data structures.
//! * [`binary_util::BinaryIo`], to automatically implement [`binary_util::interfaces::Reader`]
//!   and [`binary_util::interfaces::Writer`] .
//!
//! [`binary_util::io`]: crate::io
//! [`binary_util::interfaces`]: crate::interfaces
//! [`binary_util::BinaryIo`]: crate::BinaryIo
//! [`binary_util::interfaces::Reader`]: crate::interfaces::Reader
//! [`binary_util::interfaces::Writer`]: crate::interfaces::Writer
//!
//! # Getting Started
//! Binary Utils is available on [crates.io](https://crates.io/crates/binary_util), add the following to your `Cargo.toml`:
//! ```toml
//! [dependencies]
//! binary_util = "0.3.0"
//! ```
//!
//! Optionally, if you wish to remove the `macros` feature, you can add the following to your `Cargo.toml`:
//! ```toml
//! [dependencies]
//! binary_util = { version = "0.3.0", default-features = false }
//! ```
//!
//! # Binary IO
//! The [`io`] module provides a way to contingiously write and read binary data with the garauntees of being panic-free.
//! This module provides two structs, [`ByteReader`] and [`ByteWriter`], which are both wrappers
//! around [`bytes::Buf`] and [`bytes::BufMut`] respectively.
//!
//! Generally, you will want to use [`ByteReader`] and [`ByteWriter`] when you are reading and writing binary data manually.
//!
//! **Read Example:**
//!
//! The following example shows how to read a varint from a stream:
//! ```no_run
//! use binary_util::io::ByteReader;
//!
//! const BUFFER: &[u8] = &[255, 255, 255, 255, 7]; // 2147483647
//!
//! fn main() {
//!     let mut buf = ByteReader::from(&BUFFER[..]);
//!     buf.read_var_u32().unwrap();
//! }
//! ```
//!
//! **Write Example:**
//!
//! The following is an example of how to write a string to a stream:
//! ```no_run
//! use binary_util::io::ByteWriter;
//!
//! fn main() {
//!     let mut buf = ByteWriter::new();
//!     buf.write_string("Hello world!");
//! }
//! ```
//!
//! **Real-world example:**
//!
//! A more real-world use-case of this module could be a simple pong server,
//! where you have two packets, `Ping` and `Pong`, that respectively get relayed
//! over udp.
//! This is an example using both [`ByteReader`] and [`ByteWriter`] utilizing [`std::net::UdpSocket`]
//! to send and receive packets.
//! ```ignore
//! use binary_util::io::{ByteReader, ByteWriter};
//! use std::net::UdpSocket;
//!
//! pub struct PingPacket {
//!     pub time: u64
//! }
//!
//! pub struct PongPacket {
//!     pub time: u64,
//!     pub ping_time: u64
//! }
//!
//! fn main() -> std::io::Result<()> {
//!     let socket = UdpSocket::bind("127.0.0.1:5000")?;
//!     let mut buf = [0; 1024];
//!
//!     loop {
//!         let (amt, src) = socket.recv_from(&mut buf)?;
//!         let mut buf = ByteReader::from(&buf[..amt]);
//!
//!         match buf.read_u8()? {
//!             0 => {
//!                 let ping = PingPacket {
//!                     time: buf.read_var_u64()?
//!                 };
//!
//!                 println!("Received ping from {}", src);
//!
//!                 let mut writer = ByteWriter::new();
//!                 let pong = PongPacket {
//!                     time: std::time::SystemTime::now()
//!                             .duration_since(
//!                                 std::time::UNIX_EPOCH
//!                             )
//!                             .unwrap()
//!                             .as_millis() as u64,
//!                     ping_time: ping.time
//!                 };
//!
//!                 // Write pong packet
//!                 writer.write_u8(1);
//!                 writer.write_var_u64(pong.time);
//!                 writer.write_var_u64(pong.ping_time);
//!                 socket.send_to(writer.as_slice(), src)?;
//!             },
//!             1 => {
//!                 let pong = PongPacket {
//!                     time: buf.read_var_u64()?,
//!                     ping_time: buf.read_var_u64()?
//!                 };
//!                 println!(
//!                     "Received pong from {} with ping time of {}ms",
//!                     src,
//!                     pong.time - pong.ping_time
//!                 );
//!             }
//!             _ => {
//!                 println!("Received unknown packet from {}", src);
//!             }
//!         }
//!     }
//! }
//! ```
//!
//! [`io`]: crate::io
//! [`ByteReader`]: crate::io::ByteReader
//! [`ByteWriter`]: crate::io::ByteWriter
//! [`bytes::Buf`]: bytes::Buf
//! [`bytes::BufMut`]: bytes::BufMut
//! [`std::net::UdpSocket`]: std::net::UdpSocket
//!
//! # Interfaces
//! The [`interfaces`] module provides a way to implement reading and writing binary data with
//! two traits, [`Reader`] and [`Writer`].
//!
//! Generally, you will refer to using [`BinaryIo`] when you are implementing or enum; However in the
//! scenario you are implementing a type that may not be compatible with [`BinaryIo`], you can use
//! these traits instead.
//!
//! **Example:**
//! The following example implements the [`Reader`] and [`Writer`] traits for a `HelloPacket` allowing
//! it to be used with [`BinaryIo`]; this example also allows you to read and write the packet with an
//! easier convention.
//!
//! ```ignore
//! use binary_util::interfaces::{Reader, Writer};
//! use binary_util::io::{ByteReader, ByteWriter};
//!
//! pub struct HelloPacket {
//!     pub name: String,
//!     pub age: u8,
//!     pub is_cool: bool,
//!     pub friends: Vec<String>
//! }
//!
//! impl Reader<HelloPacket> for HelloPacket {
//!     fn read(buf: &mut ByteReader) -> std::io::Result<Self> {
//!         Ok(Self {
//!             name: buf.read_string()?,
//!             age: buf.read_u8()?,
//!             is_cool: buf.read_bool()?,
//!             friends: Vec::<String>::read(buf)?
//!         })
//!     }
//! }
//!
//! impl Writer<HelloPacket> for HelloPacket {
//!     fn write(&self, buf: &mut ByteWriter) -> std::io::Result<()> {
//!         buf.write_string(&self.name);
//!         buf.write_u8(self.age);
//!         buf.write_bool(self.is_cool);
//!         self.friends.write(buf)?;
//!         Ok(())
//!     }
//! }
//! ```
//!
//! With the example above, you now are able to read and write the packet with [`BinaryIo`],
//! as well as the added functionality of being able to read and write the packet with
//! easier with the `read` and `write` methods that are now implemented.
//!
//! ```ignore
//! fn main() {
//!     let mut buf = ByteWriter::new();
//!     let packet = HelloPacket {
//!         name: "John".to_string(),
//!         age: 18,
//!         is_cool: true,
//!         friends: vec!["Bob".to_string(), "Joe".to_string()]
//!     };
//!     buf.write_type(&packet).unwrap();
//! }
//! ```
//!
//! [`interfaces`]: crate::interfaces
//! [`Reader`]: crate::interfaces::Reader
//! [`Writer`]: crate::interfaces::Writer
//! [`BinaryIo`]: crate::BinaryIo
//!
//! # Codegen
//! The [`BinaryIo`] derive macro provides a way to implement both [`Reader`] and [`Writer`] for a type.
//! This macro is extremely useful when you are trying to implement multiple data structures that you want
//! to seemlessly read and write with the [`io`] module.
//!
//! **Example:**
//! The following example implements the [`BinaryIo`] trait for a `HelloPacket`, shortening the previous
//! example to just a few lines of code.
//! ```ignore
//! use binary_util::BinaryIo;
//!
//! #[derive(BinaryIo)]
//! pub struct HelloPacket {
//!     pub name: String,
//!     pub age: u8,
//!     pub is_cool: bool,
//!     pub friends: Vec<String>
//! }
//!
//! fn main() {
//!     let mut buf = ByteWriter::new();
//!     let packet = HelloPacket {
//!         name: "John".to_string(),
//!         age: 18,
//!         is_cool: true,
//!         friends: vec!["Bob".to_string(), "Joe".to_string()]
//!     };
//!     buf.write_type(&packet).unwrap();
//! }
//! ```
//!
//! You can view additional implementations of the derive macro by looking at the examples on the [module](crate::BinaryIo) page.
//!
//! [`BinaryIo`]: crate::BinaryIo
//! [`io`]: crate::io
//! [`Reader`]: crate::interfaces::Reader
//! [`Writer`]: crate::interfaces::Writer
//!
/// Provides a panic-free way to read and write binary data.
/// All of the methods within this module follow the protobuf specification at <https://protobuf.dev/programming-guides/encoding/>.
///
/// ## Example
/// ```no_run
/// use binary_util::io::ByteReader;
///
/// const VARINT: &[u8] = &[255, 255, 255, 255, 7]; // 2147483647
/// fn main() {
///     let mut buf = ByteReader::from(&VARINT[..]);
///     assert_eq!(buf.read_var_u32().unwrap(), 2147483647);
/// }
/// ```
pub mod interfaces;
/// Provides a derive macro that implements `::binary_util::interfaces::Reader<T>` and `::binary_util::interfaces::Writer<T>`.
///
pub use codegen::{BinaryIo, BinaryStream};
/// The io module contains implementations of these traits for `bytes::Buf` and `bytes::BufMut`.
///
/// Example:
/// ```no_run
/// use binary_util::io::ByteReader;
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
pub use io::{ByteReader, ByteWriter};
