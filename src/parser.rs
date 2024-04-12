use std::net::{SocketAddr, TcpStream};

pub struct Parser {
    peer: SocketAddr,
}
impl Parser {
    pub fn new(stream: &TcpStream) -> Self {
        Parser {
            peer: stream.peer_addr().unwrap(),
        }
    }

    pub fn write_x_forward_ip() {
      
    }
}
