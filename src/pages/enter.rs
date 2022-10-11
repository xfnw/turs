use format_bytes::format_bytes;
use std::error::Error;
use tokio;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

pub async fn switch(mut socket: TcpStream, mut user: String) -> Result<(), Box<dyn Error>> {
    socket
        .write_all(&format_bytes!(b"hewwo {}", user.as_bytes()) as &[u8])
        .await?;
    Ok(())
}
