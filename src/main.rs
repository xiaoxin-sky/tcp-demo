use std::io::{self, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn handle_client(mut source_stream: TcpStream, target_addr: String) -> io::Result<()> {
    let mut target_stream = TcpStream::connect(target_addr)?;

    // 将源 TcpStream 的数据复制到目标 TcpStream，反之亦然
    let mut source_clone = source_stream.try_clone()?;
    let mut target_clone = target_stream.try_clone()?;
    let a = BufReader::new(source_clone);

    let source_to_target = thread::spawn(move || {
        io::copy(&mut source_clone, &mut target_stream).ok();
    });

    let target_to_source = thread::spawn(move || {
        io::copy(&mut target_clone, &mut source_stream).ok();
    });

    source_to_target.join().unwrap();
    
    target_to_source.join().unwrap();
    // loop {
    //     if target_to_source.is_finished() || source_to_target.is_finished() {
    //         source_clone.shutdown(std::net::Shutdown::Both).unwrap();
    //         break;
    //     }
    // }
    println!("完成");

    Ok(())
}

fn main() -> io::Result<()> {
    // 建立一个 TcpListener 以监听来自客户端的连接
    let listener = TcpListener::bind("127.0.0.1:3363")?;

    // 指定目标服务器的地址
    let target_addr = "43.139.176.137:7001".to_string();

    // 接受来自客户端的连接并为每个连接启动一个线程来处理
    for stream in listener.incoming() {
        match stream {
            Ok(source_stream) => {
                let target_addr = target_addr.clone();
                thread::spawn(move || {
                    if let Err(err) = handle_client(source_stream, target_addr) {
                        eprintln!("Error handling client: {}", err);
                    }
                });
            }
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
            }
        }
    }
    Ok(())
}
