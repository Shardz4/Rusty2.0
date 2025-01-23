use async_std::{task, ner::TcpListener, net::TcpStream};
use futures::AsyncWriteExt;

async fn process_request(stream: &mut TcpStream) -> Result<(), std::io::Error>{
    stream.write_all(b"HTTP/1.1 200 OK\r\n\r\n").await?;
    stream.write_all(b"Hello, World!").await?;
    Ok(())
}

#[async_std::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    loop {
        let (mut stream,_) = listener.accept().await.unwrap();
        task::spawn(async move {process_request(&mut stream).await?});
    }
}