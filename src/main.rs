use std::env;
use std::error::Error;
use tokio;
use tokio::net::{TcpListener, TcpStream};

mod pages;

fn get_listen() -> String {
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "[::]:6923".to_string());
    return addr;
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
            match handle_connection(socket).await {
                Ok(_) => return,
                Err(_) => return,
            };
        });
    }
}

async fn handle_connection(socket: TcpStream) -> Result<(), Box<dyn Error>> {
    socket.set_nodelay(true)?;

    pages::enter::switch(socket, "".to_string()).await?;

    Ok(())
}
