use tokio::{io::AsyncReadExt, net::TcpListener, io::AsyncWriteExt};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("localhost:8080").await.unwrap();

    let (mut socket, _addr) = listener.accept().await.unwrap();
    loop{
        let mut buffer = [0u8; 1024];

        let bytes_read = socket.read(&mut buffer).await.unwrap();

        socket.write_all(&buffer[..bytes_read]).await.unwrap();
        println!{"{:?}", &buffer[..bytes_read]};
    }
}
