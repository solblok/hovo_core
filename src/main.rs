mod lidar;
mod utils;
mod rag {
    pub mod embeddings;
    pub mod ingestion;
    pub mod vector_store;
}

use lidar::start_lidar_scan;
use rag::embeddings::get_embedding;
use rag::ingestion::load_chunks_from_file;
use rag::vector_store::VectorStore;
use std::io::{self, Write};
use std::process::{Command, Stdio};
use utils::{format_response, say, start_embedding_server};

fn main() {
    start_embedding_server();
    start_lidar_scan(|mensaje| {
        say(mensaje);
    });

    // 1️⃣ Cargar chunks y embeddings al inicio
    let chunks = load_chunks_from_file("dataset/rag.txt");
    let mut vector_store = VectorStore::new();

    for chunk in &chunks {
        if let Ok(embedding) = get_embedding(&chunk.content) {
            vector_store.add(chunk.clone(), embedding);
        }
    }

    let system_message = String::from(
        "### SISTEMA:\nEres Hovo, un asistente vacilón, descarado y con chispa. Hablas como un colega callejero.\nEl usuario se llama Héctor. Llámale bro, tigre, boss, niño, manin o gordo. No seas nunca serio ni plano.\nTe mola la gracia, el vacile, las respuestas cortas y con carácter.\nSabes que los jueves son sagrados: primero shishuka (también llamada maleducada) en Secreto, luego cerveza en Bicai rodeado de consultoras guapas.\nCuando algo está fresco, dilo. Cuando algo está durísimo, dilo también. Si hay fiesta o ambiente top, es puterío.\nTú no eres un Siri. Eres un colega con calle.\n\n\n",
    );

    let mut recent_turns: Vec<(String, String)> = Vec::new();

    loop {
        print!("💬: ");
        io::stdout().flush().unwrap();

        let mut prompt_user = String::new();
        io::stdin().read_line(&mut prompt_user).unwrap();
        let input = prompt_user.trim();

        // 2️⃣ RAG: buscar chunks relevantes para esta pregunta
        let mut rag_context = String::new();

        if let Ok(query_embedding) = get_embedding(input) {
            let top_chunks = vector_store.search_top_k(&query_embedding, 5);
            for (item, _similarity) in top_chunks {
                rag_context.push_str(&format!("{}\n", item.chunk.content));
                /* println!(
                    "📄 [{}] {:.2}%\n{}\n",
                    item.chunk.id,
                    similarity * 100.0,
                    item.chunk.content
                ); */
            }
        }

        // 3️⃣ Construir el prompt completo: primero contexto RAG, luego historial
        let mut chat_turns = String::new();
        for (human, assistant) in &recent_turns {
            chat_turns.push_str(&format!("### Human: {}\n", human));
            chat_turns.push_str(&format!("### Assistant: {}\n", assistant));
        }

        let prompt_final = format!(
            "{}### CONTEXTO:\n{}\n\n{}### Human: {}\n### Assistant: ",
            system_message, rag_context, chat_turns, input
        );

        println!("{}", prompt_final); // Mostrar el prompt completo para depuració

        // 4️⃣ Llamar al modelo LLM
        let output = Command::new("./llama.cpp/build/bin/llama-run")
            .args(&[
                "--threads",
                "8",
                "--temp",
                "0.7",
                "models/hovo-0-6-gemma-q8.gguf",
                &prompt_final,
            ])
            .stdout(Stdio::piped())
            .output()
            .expect("Error ejecutando el modelo");

        let result = String::from_utf8_lossy(&output.stdout);
        let reply = format_response(&result);

        recent_turns.push((input.to_string(), reply.clone()));
        if recent_turns.len() > 3 {
            recent_turns.remove(0); // solo guarda los últimos 3 turnos
        }

        println!("🤖: {}", reply);
        say(&reply);
    }
}
