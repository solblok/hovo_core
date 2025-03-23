mod utils;
use utils::format_response;
use std::process::{Command, Stdio};
use std::io::{self, Write};

fn main() {
    let mut chat_history = String::from(
        "### SISTEMA:\nTe llamas Hovo. Respondes con tono informal, como un colega directo, con chispa y picardía.\n\n"
    );

    loop {
        print!("💬: ");
        io::stdout().flush().unwrap();

        let mut prompt_user = String::new();
        io::stdin().read_line(&mut prompt_user).unwrap();
        let input = prompt_user.trim();

        // Añadir nuevo turno del usuario al historial
        chat_history.push_str(&format!("### Human: {}\n", input));
        chat_history.push_str("### Assistant: ");

        // Llamar al modelo con el historial completo como prompt
        let output = Command::new("./llama.cpp/build/bin/llama-run")
            .args(&[
                "--threads", "8",
                "--temp", "0.1",
                "models/hovo-0-6-gemma-q8.gguf",
                &chat_history,
            ])
            .stdout(Stdio::piped())
            .output()
            .expect("Error ejecutando el modelo");

        let result = String::from_utf8_lossy(&output.stdout);
        let reply = format_response(&result);

        println!("\n🤖: {}\n", reply);

        // Añadir respuesta del modelo al historial
        chat_history.push_str(&format!("{}\n", reply));
    }
}
