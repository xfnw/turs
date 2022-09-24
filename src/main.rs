use std::env;
use tokio;
use tokio::net::TcpListener;

fn get_listen() -> String {
    let addr = env::args().nth(1).unwrap_or_else(|| "[::]:6923".to_string());
    return addr;
}

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind(get_listen()).await.unwrap();
}
