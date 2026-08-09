pub mod connection;
pub mod protocol;
use std::io::Write; 
use tokio::net::TcpStream;

const SERVER_HOST: &str = "zephyr.proxy.rlwy.net";
const SERVER_PORT: u16 = 14533;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut user_id = String::new();
    print!("O'z ID-ingizni kiriting (masalan, user_a): ");
    std::io::stdout().flush()?;
    std::io::stdin().read_line(&mut user_id)?;
    let user_id = user_id.trim().to_string();

    if user_id.is_empty() {
        println!("ID kiritilmadi!");
        return Ok(());
    }

    let addr = format!("{}:{}", SERVER_HOST, SERVER_PORT);
    let stream = TcpStream::connect(&addr).await?;

    println!("=== Serverga ({}) {} sifatida ulandingiz ===", addr, user_id);
    println!("Xabar yuborish formati: TARGET_ID:XABAR (masalan, user_b:Salom)");
    println!("Chiqish uchun 'exit' deb yozing.\n");
    print!("> ");
    std::io::stdout().flush()?;

    let (reader, writer) = stream.into_split();

    tokio::select! {
        _ = connection::receive_messages(reader) => {},
        _ = connection::send_messages(writer, user_id) => {},
    }
    Ok(())
}