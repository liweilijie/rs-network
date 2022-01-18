use std::{
    io::{Read, Write},
    net::TcpStream,
};

// 从客户端的代码中可以看到，我们无需显式地关闭 TcpStream，
// 因为 TcpStream 的内部实现也处理了 Drop trait，使得其离开作用域时会被关闭。
fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:9527").unwrap();
    // 一共写了12个字节
    stream.write_all(b"hello world!").unwrap();

    let mut buf = [0u8; 17];
    stream.read_exact(&mut buf).unwrap();
    println!("data: {:?}", String::from_utf8_lossy(&buf));
}
