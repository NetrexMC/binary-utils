
use std::net::SocketAddr;
use binary_utils::*;

#[test]
fn test_socket() {
    let socket: SocketAddr = "127.0.0.1:19132".parse().unwrap();
    dbg!(&socket.ip());
    assert_eq!(socket.fparse(), vec![4, 127, 0, 0, 1, 74, 188]);
}