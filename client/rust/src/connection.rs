use crate::{executor, protocol};
use std::io::Write;
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::executor::{TerminalState};
use tokio::{
    io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::tcp::{OwnedReadHalf, OwnedWriteHalf},
    sync::mpsc,
};

pub async fn receive_messages(
    reader: OwnedReadHalf,
    user_id: String,
    tx: mpsc::Sender<String>,
    state: Arc<Mutex<TerminalState>>,
) {
    let mut buf_reader = BufReader::new(reader);
    let mut line = String::new();

    loop {
        line.clear();
        match buf_reader.read_line(&mut line).await {
            Ok(0) => {
                println!("\nServer bilan aloqa uzildi.");
                break;
            }
            Ok(_) => {
                if let Ok(packet) = serde_json::from_str::<protocol::IncomingPacket>(&line) {
                    let sender = packet.from.unwrap_or_else(|| "Noma'lum".to_string());
                    let payload = packet.payload.unwrap_or_default();
                    let _ = std::io::stdout().flush();

                        let reply_text = executor::run_system(&payload, Arc::clone(&state)).await.to_string();

                        let reply_packet = protocol::OutgoingPacket {
                            sender_id: &user_id,
                            target_id: &sender, 
                            payload: &reply_text,
                        };

                        if let Ok(mut json_str) = serde_json::to_string(&reply_packet) {
                            json_str.push('\n');
                            let _ = tx.send(json_str).await;
                        }
                }
            }
            Err(e) => {
                println!("\nXabar o'qishda xatolik: {}", e);
                break;
            }
        }
    }
}

pub async fn send_messages(
    mut writer: OwnedWriteHalf,
    user_id: String,
    mut rx: mpsc::Receiver<String>,
) {
    let init_packet = protocol::OutgoingPacket {
        sender_id: &user_id,
        target_id: "system",
        payload: "online",
    };

    if let Ok(mut json_str) = serde_json::to_string(&init_packet) {
        json_str.push('\n');
        if writer.write_all(json_str.as_bytes()).await.is_ok() {
            let _ = writer.flush().await;
        }
    }

    loop {
        tokio::select! {
            Some(msg_to_send) = rx.recv() => {
                if writer.write_all(msg_to_send.as_bytes()).await.is_ok() {
                    let _ = writer.flush().await;
                }
            }
        }
    }
}