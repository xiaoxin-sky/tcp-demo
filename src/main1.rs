use std::io::{self, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
mod adapter;
mod connector;
mod parser;
fn main() {
    let listener = TcpListener::bind("127.0.0.1:3363").expect("Failed to bind address");

    println!("Server listening on port 3363...");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("开启一个新的链接{}", stream.peer_addr().unwrap());
                std::thread::spawn(move || {
                    // handle_client(stream);
                    let new_connection =
                        connector::Connector::new("43.139.176.137:7001".to_string());
                    let mut adapt = adapter::Adapter::new(stream, new_connection.get_remote_stream());
                    adapt.transform();
                });
            }
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
            }
        }
    }
}

fn handle_client(stream: TcpStream) -> io::Result<()> {
    let reader = BufReader::new(&stream);
    // 报文首部
    let mut header = Vec::<u8>::new();
    // 报文体
    let mut body = Vec::<u8>::new();
    // 报文首部提取到的 Content-Length 长度
    let mut content_len: Option<usize> = None;

    for byte in reader.bytes() {
        let byte = byte?;

        if content_len.is_some() {
            body.push(byte);
            // body 内容已经填充完毕
            if body.len() >= content_len.unwrap_or(0) {
                break;
            }
        } else {
            header.push(byte);

            // 从报文首部中提取内容长度
            content_len = extract_content_len(&header);
            if let Some(len) = content_len {
                // 如果报文长度是0说明没有 报文体，说明已经读取完成了，退出读取流
                if len == 0 {
                    break;
                }
            }
        }
    }

    // let str = format!("内容长度{}", content_len.unwrap_or(0));
    serve_res(stream, &body)?;
    Ok(())
}

/// 从 header 中提取出 Content-Length 长度
fn extract_content_len(header: &[u8]) -> Option<usize> {
    if header.len() < 4 {
        return None;
    }

    // 最后四个字节
    let last_four = &header[header.len() - 4..];

    match last_four.eq(b"\r\n\r\n") {
        true => {
            let header_str = String::from_utf8_lossy(header);
            for line in header_str.lines() {
                let line = line.trim();
                println!("{line}");
                if line.starts_with("Content-Length:") {
                    let content_size = line
                        .split_whitespace()
                        .nth(1)
                        .unwrap()
                        .parse::<usize>()
                        .unwrap();
                    return Some(content_size);
                }
            }
            None
        }
        false => None,
    }
}

fn serve_res(mut steam: TcpStream, data: &[u8]) -> io::Result<()> {
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: text/html; charset=utf-8\r\n\r\n",
        data.len(),
    );

    steam.write_all(response.as_bytes())?;
    steam.write_all(data)?;
    steam.flush()?;
    steam.shutdown(std::net::Shutdown::Both)?;
    Ok(())
}
