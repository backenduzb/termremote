use std::path::PathBuf;
use std::sync::Arc;
use tokio::process::Command;
use tokio::sync::Mutex;

pub struct TerminalState {
    pub current_dir: PathBuf,
}

impl TerminalState {
    pub fn new() -> Self {
        Self {
            current_dir: std::env::current_dir().unwrap_or_else(|_| PathBuf::from("/")),
        }
    }
}

pub async fn run_system(cmd: &str, state: Arc<Mutex<TerminalState>>) -> String {
    let mut state_guard = state.lock().await;
    let trimmed_cmd = cmd.trim();

    if trimmed_cmd.starts_with("cd ") || trimmed_cmd == "cd" {
        let target = if trimmed_cmd == "cd" {
            dirs::home_dir().unwrap_or_else(|| PathBuf::from("/"))
        } else {
            PathBuf::from(trimmed_cmd[3..].trim())
        };

        let new_path = if target.is_relative() {
            state_guard.current_dir.join(target)
        } else {
            target
        };

        if new_path.exists() && new_path.is_dir() {
            state_guard.current_dir = new_path.canonicalize().unwrap_or(new_path);
            return format!("{}", state_guard.current_dir.display());
        } else {
            return format!("cd: no such file or directory: {}", trimmed_cmd[3..].trim());
        }
    }

    if is_interactive_or_long_running(trimmed_cmd) {
        let child = Command::new("sh")
            .arg("-c")
            .arg(trimmed_cmd)
            .current_dir(&state_guard.current_dir)
            .spawn();

        match child {
            Ok(_) => format!("Started process in background: {}", trimmed_cmd),
            Err(e) => format!("Failed to start process: {}", e),
        }
    } else {
        let output = Command::new("sh")
            .arg("-c")
            .arg(trimmed_cmd)
            .current_dir(&state_guard.current_dir)
            .output()
            .await;

        match output {
            Ok(out) => {
                let stdout = String::from_utf8_lossy(&out.stdout);
                let stderr = String::from_utf8_lossy(&out.stderr);

                if !stdout.is_empty() {
                    stdout.trim().to_string()
                } else if !stderr.is_empty() {
                    stderr.trim().to_string()
                } else {
                    "Completed with no output".to_string()
                }
            }
            Err(e) => format!("Error: {}", e),
        }
    }
}

fn is_interactive_or_long_running(cmd: &str) -> bool {
    let interactive_cmds = ["cava", "htop", "top", "vim", "nvim", "nano", "less"];
    let first_word = cmd.split_whitespace().next().unwrap_or("");
    interactive_cmds.contains(&first_word)
}
