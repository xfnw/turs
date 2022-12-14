use std::env;
use std::error::Error;
use tokio::net::{TcpListener, TcpStream};

mod pages;

fn get_listen() -> String {
    env::args()
        .nth(1)
        .unwrap_or_else(|| "[::]:6923".to_string())
}

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind(get_listen()).await.unwrap();

    loop {
        let (socket, _) = match listener.accept().await {
            Ok(h) => h,
            Err(e) => {
                eprintln!("oh no {}", e);
                continue;
            }
        };

        tokio::spawn(async move {
            let _ = handle_connection(socket).await;
        });
    }
}

async fn handle_connection(socket: TcpStream) -> Result<(), Box<dyn Error>> {
    socket.set_nodelay(true)?;

    pages::enter::switch(socket, "".to_string()).await?;

    Ok(())
}
