use std::{
    io::{Read, Write},
    net::TcpListener,
    thread,
};
fn main() {
    let listener = TcpListener::bind("0.0.0.0:9527").unwrap();
    loop {
        let (mut stream, addr) = listener.accept().unwrap();
        println!("Accepted a new connection: {}", addr);
        thread::spawn(move || {
            // 目前客户端和服务器都需要硬编码要接收数据的大小，这样不够灵活，
            // 后续我们会看到如何通过使用消息帧（frame）更好地处理。
            let mut buf = [0u8; 12];
            stream.read_exact(&mut buf).unwrap();
            println!("data: {:?}", String::from_utf8_lossy(&buf));
            // 一共写了17个字节
            stream.write_all(b"glad to meet you!").unwrap();
        });
    }
}
