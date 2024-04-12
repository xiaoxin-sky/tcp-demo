use std::{
    io::{self, BufReader, Read, Write},
    net::TcpStream,
};

pub struct Adapter {
    incoming_stream: TcpStream,
    remote_stream: TcpStream,
}

impl Adapter {
    /// Creates a new instance of the Adapter.
    pub fn new(incoming_stream: TcpStream, remote_stream: TcpStream) -> Self {
        Self {
            incoming_stream,
            remote_stream,
        }
    }

    /// Transforms the data from the incoming stream to the remote stream.
    pub fn transform(&mut self) {
        // let reader = BufReader::new(&self.incoming_stream);
        // match self.remote_stream.write(reader.buffer()) {
        //     Ok(res) => {
        //         println!("成功{}", res);
        //         self.incoming_stream.shutdown(std::net::Shutdown::Both).unwrap();
        //         self.remote_stream.shutdown(std::net::Shutdown::Both).unwrap();
        //     }
        //     Err(err) => {
        //         eprintln!("失败{}", err);
        //         self.incoming_stream.shutdown(std::net::Shutdown::Both).unwrap();
        //         self.remote_stream.shutdown(std::net::Shutdown::Both).unwrap();
        //     }
        // };

        std::io::copy(&mut self.incoming_stream, &mut self.remote_stream);
        std::io::copy(&mut self.remote_stream, &mut self.incoming_stream);
    }
}
