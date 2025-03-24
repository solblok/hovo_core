use regex::Regex;
use std::process::{Command, Stdio};

pub fn start_embedding_server() {
    let result = Command::new("uvicorn")
        .arg("scripts.embed_server:app")
        .arg("--host")
        .arg("127.0.0.1")
        .arg("--port")
        .arg("8000")
        .arg("--reload")
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn();

    match result {
        Ok(child) => {
            println!(
                "🔥 Servidor de embeddings lanzado vía uvicorn (PID: {})",
                child.id()
            );
            std::thread::sleep(std::time::Duration::from_secs(1)); // espera a que arranque
        }
        Err(e) => {
            eprintln!("💥 Error al lanzar uvicorn: {}", e);
        }
    }
}

pub fn format_response(text: &str) -> String {
    let no_emoji = Regex::new(r"[^\p{L}\p{N}\p{P}\p{Zs}\n]")
        .unwrap()
        .replace_all(text, "");
    let no_bold = Regex::new(r"\*\*(.*?)\*\*")
        .unwrap()
        .replace_all(&no_emoji, "$1");
    no_bold.to_string().replace("[0m", "")
}

pub fn say(text: &str) {
    Command::new("python3")
        .arg("scripts/tts.py")
        .arg(text)
        .status()
        .expect("Error ejecutando TTS en Python");
}
