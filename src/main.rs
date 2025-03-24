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

    let mut chat_history = String::from(
        "### SISTEMA:\nTe llamas Hovo. Respondes con tono informal, como un colega directo, con chispa y respuestas concisas.\n\n",
    );

    loop {
        print!("💬: ");
        io::stdout().flush().unwrap();

        let mut prompt_user = String::new();
        io::stdin().read_line(&mut prompt_user).unwrap();
        let input = prompt_user.trim();

        // 2️⃣ RAG: buscar chunks relevantes para esta pregunta
        let mut rag_context = String::new();

        if let Ok(query_embedding) = get_embedding(input) {
            let top_chunks = vector_store.search_top_k(&query_embedding, 3);
            for item in top_chunks {
                rag_context.push_str(&format!("{}\n", item.chunk.content));
            }
        }

        // 3️⃣ Construir el prompt completo: primero contexto RAG, luego historial
        let prompt_final = format!(
            "### CONTEXTO:\n{}\n\n{}### Human: {}\n### Assistant: ",
            rag_context, chat_history, input
        );

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

        say(&reply);
        println!("🤖: {}", reply);

        // 5️⃣ Añadir respuesta al historial
        chat_history.push_str(&format!("### Human: {}\n", input));
        chat_history.push_str(&format!("### Assistant: {}\n", reply));
    }
}
