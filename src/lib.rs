/// The `ByteReader` and `ByteWriter` traits are used to read and write bytes from a buffer.
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
pub mod stream;
