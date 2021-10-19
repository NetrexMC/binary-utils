// #![feature(log_syntax)]

use std::io;
use std::convert::{From, Into, TryInto};
use std::net::{IpAddr, Ipv6Addr, SocketAddrV6, SocketAddr};

pub use bin_macro::*;

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::io::{Cursor, Read, Write};

pub mod u24;
pub mod varint;

pub use self::{u24::*, varint::*};

pub type Stream = io::Cursor<Vec<u8>>;

pub trait Streamable {
    /// Writes `self` to the given buffer.
    fn parse(&self) -> Vec<u8>;
    /// Reads `self` from the given buffer.
    fn compose(source: &[u8], position: &mut usize) -> Self
    where
        Self: Sized;
}

/// Little Endian Encoding
pub struct LE<T>(pub T);

impl<T> Streamable for LE<T>
where
    T: Streamable {
    fn parse(&self) -> Vec<u8> {
        reverse_vec(self.0.parse())
    }

    fn compose(source: &[u8], position: &mut usize) -> Self {
        // If the source is expected to be LE we can swap it to BE bytes
        // Doing this makes the byte stream officially BE.
        // hehe...
        let stream = reverse_vec(source.to_vec());
        LE(T::compose(&stream[..], position))
    }
}

impl<T> LE<T> {
    pub fn inner(self) -> T {
        self.0
    }
}

/// Reverses the bytes in a given vector
pub fn reverse_vec(bytes: Vec<u8>) -> Vec<u8> {
    let mut ret: Vec<u8> = Vec::new();

    for x in (0..bytes.len()).rev() {
        ret.push(*bytes.get(x).unwrap());
    }
    ret
}

/// Big Endian Encoding
pub struct BE<T>(pub T);

macro_rules! impl_streamable_primitive {
    ($ty: ty) => {
        impl Streamable for $ty {
            fn parse(&self) -> Vec<u8> {
                self.to_be_bytes().to_vec()
            }

            fn compose(source: &[u8], position: &mut usize) -> Self {
                // get the size
                let size = ::std::mem::size_of::<$ty>();
                let range = position.clone()..(size + position.clone());
                let data = <$ty>::from_be_bytes(source.get(range).unwrap().try_into().unwrap());
                *position += size;
                data
            }
        }
    };
}

impl_streamable_primitive!(u8);
impl_streamable_primitive!(u16);
impl_streamable_primitive!(u32);
impl_streamable_primitive!(u64);
impl_streamable_primitive!(u128);
impl_streamable_primitive!(i8);
impl_streamable_primitive!(i16);
impl_streamable_primitive!(i32);
impl_streamable_primitive!(i64);
impl_streamable_primitive!(i128);

macro_rules! impl_streamable_vec_primitive {
    ($ty: ty) => {
        impl Streamable for Vec<$ty> {
            fn parse(&self) -> Vec<u8> {
                use ::std::io::Write;
                // write the length as a varint
                let mut v: Vec<u8> = Vec::new();
                v.write_all(&VarInt(v.len() as u32).to_be_bytes()[..])
                    .unwrap();
                for x in self.iter() {
                    v.extend(x.parse().iter());
                }
                v
            }

            fn compose(source: &[u8], position: &mut usize) -> Self {
                // use ::std::io::Read;
                // read a var_int
                let mut ret: Vec<$ty> = Vec::new();
                let varint = VarInt::<u32>::from_be_bytes(source);
                let length: u32 = varint.into();

                *position += varint.get_byte_length() as usize;

                // read each length
                for _ in 0..length {
                    ret.push(<$ty>::compose(&source, position));
                }
                ret
            }
        }
    };
}

impl_streamable_vec_primitive!(u8);
impl_streamable_vec_primitive!(u16);
impl_streamable_vec_primitive!(u32);
impl_streamable_vec_primitive!(u64);
impl_streamable_vec_primitive!(u128);
impl_streamable_vec_primitive!(i8);
impl_streamable_vec_primitive!(i16);
impl_streamable_vec_primitive!(i32);
impl_streamable_vec_primitive!(i64);
impl_streamable_vec_primitive!(i128);

// implements bools
impl Streamable for bool {
    fn parse(&self) -> Vec<u8> {
        vec![if *self { 1 } else { 0 }]
    }

    fn compose(source: &[u8], position: &mut usize) -> Self {
        let v = source[*position] == 1;
        *position += 1;
        v
    }
}

impl Streamable for String {
    fn parse(&self) -> Vec<u8> {
        let mut buffer = Vec::<u8>::new();
        buffer.write_u16::<BigEndian>(self.len() as u16).unwrap();
        buffer.write_all(self.as_bytes()).unwrap();
        buffer
    }

    fn compose(source: &[u8], position: &mut usize) -> Self {
        let mut stream = Cursor::new(source);
        stream.set_position(position.clone() as u64);
        // Maybe do this in the future?
        let len: usize = stream.read_u16::<BigEndian>().unwrap().into();
        *position = (stream.position() as usize) + len;

        unsafe {
            // todo: Remove this nasty hack.
            // todo: The hack being, remove the 2 from indexing on read_short
            // todo: And utilize stream.
            String::from_utf8_unchecked(
                stream.get_ref()[2..len + stream.position() as usize].to_vec(),
            )
        }
    }
}

impl Streamable for SocketAddr {
    fn parse(&self) -> Vec<u8> {
        let mut stream = Vec::<u8>::new();
        match *self {
            Self::V4(_) => {
                stream.write_u8(4).unwrap();
                let partstr = self.to_string();
                let parts: Vec<&str> = partstr.split(".").collect();
                for part in parts {
                    let mask = u8::from_str_radix(part, 10).unwrap_or(0);
                    stream.write_u8(mask).unwrap();
                }
                stream
                    .write_u16::<BigEndian>(self.port())
                    .expect("Could not write port to stream.");
                stream
            }
            Self::V6(addr) => {
                stream.write_u8(6).unwrap();
                // family? or length??
                stream.write_u16::<BigEndian>(0).unwrap();
                // port
                stream.write_u16::<BigEndian>(self.port()).unwrap();
                // flow
                stream.write_u32::<BigEndian>(addr.flowinfo()).unwrap();
                // actual address here
                stream.write(&addr.ip().octets()).unwrap();
                // scope
                stream.write_u32::<BigEndian>(addr.scope_id()).unwrap();
                stream
            }
        }
    }

    fn compose(source: &[u8], position: &mut usize) -> Self {
        let mut stream = Cursor::new(source);
        stream.set_position(*position as u64);
        match stream.read_u8().unwrap() {
            4 => {
                let from = stream.position() as usize;
                let to = stream.position() as usize + 4;
                let parts = &source[from..to];
                stream.set_position(to as u64);
                let port = stream.read_u16::<BigEndian>().unwrap();
                *position = stream.position() as usize;
                SocketAddr::new(IpAddr::from([parts[0], parts[1], parts[2], parts[3]]), port)
            }
            6 => {
                let _family = stream.read_u16::<BigEndian>().unwrap();
                let port = stream.read_u16::<BigEndian>().unwrap();
                let flow = stream.read_u32::<BigEndian>().unwrap();
                let mut parts: [u8; 16] = [0; 16];
                stream.read(&mut parts).unwrap();
                // we need to read parts into address
                let address = {
                    let mut s = Cursor::new(parts);
                    let (a, b, c, d, e, f, g, h) = (
                        s.read_u16::<BigEndian>().unwrap_or(0),
                        s.read_u16::<BigEndian>().unwrap_or(0),
                        s.read_u16::<BigEndian>().unwrap_or(0),
                        s.read_u16::<BigEndian>().unwrap_or(0),
                        s.read_u16::<BigEndian>().unwrap_or(0),
                        s.read_u16::<BigEndian>().unwrap_or(0),
                        s.read_u16::<BigEndian>().unwrap_or(0),
                        s.read_u16::<BigEndian>().unwrap_or(0),
                    );
                    Ipv6Addr::new(a, b, c, d, e, f, g, h)
                };
                let scope = stream.read_u32::<BigEndian>().unwrap();
                *position = stream.position() as usize;
                SocketAddr::from(SocketAddrV6::new(address, port, flow, scope))
            }
            _ => panic!("Unknown Address type!"),
        }
        //  let addr_type = self.read_byte();
        //           if addr_type == 4 {
        //                let parts = self.read_slice(Some(4 as usize));
        //                let port = self.read_ushort();
        //                SocketAddr::new(IpAddr::from([parts[0], parts[1], parts[2], parts[3]]), port)
        //           } else {
        //                SocketAddr::new(IpAddr::from([0, 0, 0, 0]), 0)
        //           }
    }
}

/// Writes a vector whose length is written with a short
impl Streamable for Vec<String> {
    fn parse(&self) -> Vec<u8> {
        // write the length as a varint
        let mut v: Vec<u8> = Vec::new();
        v.write_u16::<BigEndian>(v.len() as u16).unwrap();
        for x in self.iter() {
            v.extend(LE(x.clone()).parse().iter());
        }
        v
    }

    fn compose(source: &[u8], position: &mut usize) -> Self {
        // read a var_int
        let mut stream = Cursor::new(source);
        let mut ret: Vec<LE<String>> = Vec::new();
        let length = stream.read_u16::<BigEndian>().unwrap();

        *position = stream.position() as usize;
        // read each length
        for _ in 0..length {
            ret.push(LE::<String>::compose(&source, position));
        }
        ret.iter().map(|v| v.0.clone()).collect()
    }
}
