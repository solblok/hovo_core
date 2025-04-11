use regex::Regex;
use std::io::Read;
use std::net::TcpListener;
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

pub fn start_face_listener() {
    let last_greeting = Arc::new(Mutex::new(Instant::now() - Duration::from_secs(300)));
    let last_greeting_cloned = Arc::clone(&last_greeting);
    thread::spawn(move || {
        let listener =
            TcpListener::bind("127.0.0.1:5005").expect("No se pudo iniciar el servidor de cara");
        for stream in listener.incoming() {
            if let Ok(mut stream) = stream {
                let mut buffer = [0; 1024];
                if let Ok(bytes_read) = stream.read(&mut buffer) {
                    let message = String::from_utf8_lossy(&buffer[..bytes_read]);

                    if message.contains("HECTOR_DETECTED") {
                        let mut last_time = last_greeting_cloned.lock().unwrap();
                        let now = Instant::now();

                        if now.duration_since(*last_time).as_secs() > 300 {
                            println!("👀 Reconocido: Héctor (primer saludo en 5')");
                            *last_time = now;
                            let prompt = format!(
                                "Acabas de ver a Héctor, salúdale de forma concisa como si fueses su colega vacilón"
                            );
                            let output = Command::new("./llama.cpp/build/bin/llama-run")
                                .args(&[
                                    "--threads",
                                    "8",
                                    "--temp",
                                    "0.7",
                                    "models/hovo-0-6-gemma-q8.gguf",
                                    &prompt,
                                ])
                                .stdout(Stdio::piped())
                                .output()
                                .expect("Error ejecutando el modelo");

                            let result = String::from_utf8_lossy(&output.stdout);
                            let reply = format_response(&result);
                            println!("🤖: {}", reply);
                            say(&reply);
                        } else {
                            println!("⌛ Héctor reconocido pero aún en cooldown");
                        }
                    }
                }
            }
        }
    });
}

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
