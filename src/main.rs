use std::process::{Command, Stdio};
use std::io::{self, Write};

fn main() {
    print!("Prompt > ");
    io::stdout().flush().unwrap();

    let mut prompt_user = String::new();
    io::stdin().read_line(&mut prompt_user).unwrap();

    let prompt = format!("### CONTEXTO: Al usuario le gusta fumar shishuka los jueves de afterwork en Bicai. Le gustan respuestas con chispa, picardía y concisas, como un colega. ### Human: {}\n### Assistant:", prompt_user.trim());

    let output = Command::new("./llama.cpp/build/bin/llama-run")
        .args(&[
            "--threads", "8",
            "--temp", "0.8",
            "models/hovo-0-2-gemma-q8.gguf", 
            &prompt
        ])
        .stdout(Stdio::piped())
        .output()
        .expect("Error ejecutando el modelo");

    let result = String::from_utf8_lossy(&output.stdout);
    println!("\n>> Respuesta del modelo:\n{}", result);
}
