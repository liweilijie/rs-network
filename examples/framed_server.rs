use anyhow::Result;
use bytes::Bytes;
use futures::{SinkExt, StreamExt};
use tokio::net::TcpListener;
use tokio_util::codec::{Framed, LengthDelimitedCodec};

#[tokio::main]
async fn main() -> Result<()> {
    let listener = TcpListener::bind("0.0.0.0:9527").await?;
    loop {
        let (stream, addr) = listener.accept().await?;
        println!("accept: {:?}", addr);
        // LengthDelimitedCodec 默认4字节长度
        let mut stream = Framed::new(stream, LengthDelimitedCodec::new());

        tokio::spawn(async move {
            // 接收到的消息会只包含消息主体( 不包含长度)
            while let Some(Ok(data)) = stream.next().await {
                println!("Got: {:?}", String::from_utf8_lossy(&data));
                // 发送的消息也只需要发送消息主体，不需要提供长度
                // Framed/LengthDelimitedCodec 会自动计算并添加长度
                stream.send(Bytes::from("goodbye world!")).await.unwrap();
            }
        });
    }
}
