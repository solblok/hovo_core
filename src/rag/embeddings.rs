use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Serialize)]
struct EmbeddingRequest {
    text: String,
}

#[derive(Debug, Deserialize)]
pub struct EmbeddingResponse {
    pub embedding: Vec<f32>,
}

/// Función para obtener embedding desde servicio externo
pub fn get_embedding(text: &str) -> Result<Vec<f32>, Box<dyn Error>> {
    let client = Client::new();

    let request_body = EmbeddingRequest {
        text: text.to_string(),
    };

    let response = client
        .post("http://127.0.0.1:8000/embed")
        .json(&request_body)
        .send()?
        .json::<EmbeddingResponse>()?;

    Ok(response.embedding)
}
