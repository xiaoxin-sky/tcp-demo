use std::net::TcpStream;

pub struct Connector {
    remote_addr: String,
    remote_stream: TcpStream,
}

impl Connector {
    pub fn new(remote_addr: String) -> Self {
        let remote_stream = TcpStream::connect(&remote_addr).unwrap();
        Self {
            remote_addr,
            remote_stream,
        }
    }

    pub fn get_remote_stream(&self) -> TcpStream {
        self.remote_stream.try_clone().unwrap()
    }
}
