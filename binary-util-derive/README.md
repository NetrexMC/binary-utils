# binary-util-derive

This crate provides the derive macros for `binary-util` crate.

# Binary IO

The [`io`](https://docs.rs/binary-util/latest/binary_util/io) module provides a way to contingiously write and read binary data with the garauntees of being panic-free.
This module provides two structs, [`ByteReader`](https://docs.rs/binary-util/latest/binary_util/interfaces) and [`ByteWriter`](https://docs.rs/binary-util/latest/binary_util/interfaces), which are both wrappers
around [`bytes::Buf`](https://docs.rs/bytes/1.4.0/bytes/buf/trait.Buf.html) and [`bytes::BufMut`](https://docs.rs/bytes/1.4.0/bytes/buf/trait.BufMut.html) respectively.
Generally, you will want to use [`ByteReader`](https://docs.rs/binary-util/latest/binary_util/io/struct.ByteReader.html) and [`ByteWriter`](https://docs.rs/binary-util/latest/binary_util/io/struct.ByteWriter.html) when you are reading and writing binary data manually.

**Read Example:**
The following example shows how to read a varint from a stream:

```rust
use binary_util::io::ByteReader;
const BUFFER: &[u8] = &[255, 255, 255, 255, 7]; // 2147483647
fn main() {
    let mut buf = ByteReader::from(&BUFFER[..]);
    buf.read_var_u32().unwrap();
}
```

**Write Example:**
The following is an example of how to write a string to a stream:

```rust
use binary_util::io::ByteWriter;
fn main() {
    let mut buf = ByteWriter::new();
    buf.write_string("Hello world!");
}
```

**Real-world example:**
A more real-world use-case of this module could be a simple pong server,
where you have two packets, `Ping` and `Pong`, that respectively get relayed
over udp.
This is an example using both [`ByteReader`](https://docs.rs/binary-util/latest/binary_util/io/struct.ByteReader.html) and [`ByteWriter`](https://docs.rs/binary-util/latest/binary_util/io/struct.ByteWriter.html) utilizing [`std::net::UdpSocket`](https://docs.rs/binary-util/latest)
to send and receive packets.

```rust
use binary_util::io::{ByteReader, ByteWriter};
use std::net::UdpSocket;
pub struct PingPacket {
    pub time: u64
}
pub struct PongPacket {
    pub time: u64,
    pub ping_time: u64
}
fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:5000")?;
    let mut buf = [0; 1024];
    loop {
        let (amt, src) = socket.recv_from(&mut buf)?;
        let mut buf = ByteReader::from(&buf[..amt]);
        match buf.read_u8()? {
            0 => {
                let ping = PingPacket {
                    time: buf.read_var_u64()?
                };
                println!("Received ping from {}", src);
                let mut writer = ByteWriter::new();
                let pong = PongPacket {
                    time: std::time::SystemTime::now()
                            .duration_since(
                                std::time::UNIX_EPOCH
                            )
                            .unwrap()
                            .as_millis() as u64,
                    ping_time: ping.time
                };
                // Write pong packet
                writer.write_u8(1);
                writer.write_var_u64(pong.time);
                writer.write_var_u64(pong.ping_time);
                socket.send_to(writer.as_slice(), src)?;
            },
            1 => {
                let pong = PongPacket {
                    time: buf.read_var_u64()?,
                    ping_time: buf.read_var_u64()?
                };
                println!(
                    "Received pong from {} with ping time of {}ms",
                    src,
                    pong.time - pong.ping_time
                );
            }
            _ => {
                println!("Received unknown packet from {}", src);
            }
        }
    }
}
```

# Interfaces

The [`interfaces`](https://docs.rs/binary-util/latest/binary_utils/interfaces) module provides a way to implement reading and writing binary data with
two traits, [`Reader`](https://docs.rs/binary-util/latest/binary_util/interfaces/trait.Reader.html) and [`Writer`](https://docs.rs/binary-util/latest/binary_util/interfaces/trait.Writer.html).
Generally, you will refer to using [`BinaryIo`](https://docs.rs/binary-util-derive/latest) when you are implementing or enum; However in the
scenario you are implementing a type that may not be compatible with [`BinaryIo`](https://docs.rs/binary-util-derive/latest), you can use
these traits instead.

**Example:**
The following example implements the [`Reader`](https://docs.rs/binary-util/latest/binary_util/interfaces/trait.Reader.html) and [`Writer`](https://docs.rs/binary-util/latest/binary_util/interfaces/trait.Writer.html) traits for a `HelloPacket` allowing
it to be used with [`BinaryIo`](https://docs.rs/binary-util-derive/latest); this example also allows you to read and write the packet with an
easier convention.

```rust
use binary_util::interfaces::{Reader, Writer};
use binary_util::io::{ByteReader, ByteWriter};
pub struct HelloPacket {
    pub name: String,
    pub age: u8,
    pub is_cool: bool,
    pub friends: Vec<String>
}
impl Reader<HelloPacket> for HelloPacket {
    fn read(buf: &mut ByteReader) -> std::io::Result<Self> {
        Ok(Self {
            name: buf.read_string()?,
            age: buf.read_u8()?,
            is_cool: buf.read_bool()?,
            friends: Vec::<String>::read(buf)?
        })
    }
}
impl Writer<HelloPacket> for HelloPacket {
    fn write(&self, buf: &mut ByteWriter) -> std::io::Result<()> {
        buf.write_string(&self.name);
        buf.write_u8(self.age);
        buf.write_bool(self.is_cool);
        self.friends.write(buf)?;
        Ok(())
    }
}
```

With the example above, you now are able to read and write the packet with [`BinaryIo`](https://docs.rs/binary-util-derive/latest),
as well as the added functionality of being able to read and write the packet with
easier with the `read` and `write` methods that are now implemented.

```rust
fn main() {
    let mut buf = ByteWriter::new();
    let packet = HelloPacket {
        name: "John".to_string(),
        age: 18,
        is_cool: true,
        friends: vec!["Bob".to_string(), "Joe".to_string()]
    };
    buf.write_type(&packet).unwrap();
}
```
