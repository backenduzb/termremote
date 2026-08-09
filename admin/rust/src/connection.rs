use crate::protocol;
use std::io::Write;

use tokio::{
    io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::tcp::{OwnedReadHalf, OwnedWriteHalf},
};

pub async fn receive_messages(reader: OwnedReadHalf) {
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
                    print!("\n\r[Xabar - {}]: {}\n> ", sender, payload);
                    let _ = std::io::stdout().flush();
                }
            }
            Err(e) => {
                println!("\nXabar o'qishda xatolik: {}", e);
                break;
            }
        }
    }
}

pub async fn send_messages(mut writer: OwnedWriteHalf, user_id: String) {
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

    let mut stdin_reader = BufReader::new(io::stdin());
    let mut input_buf = String::new();

    loop {
        input_buf.clear();
        if stdin_reader.read_line(&mut input_buf).await.is_err() {
            break;
        }

        let msg = input_buf.trim();
        if msg.eq_ignore_ascii_case("exit") {
            break;
        }

        if !msg.contains(':') {
            println!("⚠️ Xato format! Format ushbu ko'rinishda bo'lsin -> target_id:xabar");
            print!("> ");
            let _ = std::io::stdout().flush();
            continue;
        }

        let mut parts = msg.splitn(2, ':');
        let target_id = parts.next().unwrap_or("").trim();
        let payload = parts.next().unwrap_or("").trim();

        let packet = protocol::OutgoingPacket {
            sender_id: &user_id,
            target_id,
            payload,
        };

        if let Ok(mut json_str) = serde_json::to_string(&packet) {
            json_str.push('\n');
            let send_result = async {
                writer.write_all(json_str.as_bytes()).await?;
                writer.flush().await?;
                Ok::<(), std::io::Error>(())
            }
            .await;

            if send_result.is_err() {
                println!("Xabar yuborishda xatolik yuz berdi.");
                break;
            }
        }
    }
}
