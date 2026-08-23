pub mod connection;
pub mod protocol;
use std::io::Write; 
use tokio::net::TcpStream;

const SERVER_HOST: &str = "switchback.proxy.rlwy.net";
const SERVER_PORT: u16 = 26527;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = format!("{}:{}", SERVER_HOST, SERVER_PORT);
    let stream = TcpStream::connect(&addr).await?;

    print!("> ");
    std::io::stdout().flush()?;

    let (reader, writer) = stream.into_split();

    tokio::select! {
        _ = connection::receive_messages(reader) => {},
        _ = connection::send_messages(writer, "105445".to_string()) => {},
    }
    Ok(())
}