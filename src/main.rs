use futures::stream::StreamExt;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:6142";
    let mut listener = TcpListener::bind(addr).await.unwrap();

    let server = async move {
        let mut incoming = listener.incoming();
        while let Some(connection) = incoming.next().await {
            match connection {
                Ok(mut socket) => {
                    tokio::spawn(async move {
                        let (mut reader, mut writer) = socket.split();

                        match tokio::io::copy(&mut reader, &mut writer).await {
                            Ok(amount) => {
                                println!("wrote {} bytes", amount);
                            }
                            Err(err) => {
                                eprintln!("IO error {:?}", err);
                            }
                        }
                    });
                }
                Err(err) => {
                    eprintln!("accept error {:?}", err);
                }
            }
        }
    };

    println!("Server running on localhost:6142");

    server.await;
}
