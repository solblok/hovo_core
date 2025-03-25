use crate::rag::ingestion::Chunk;

/// Cada elemento del vector store tiene su chunk original + su embedding
#[derive(Debug, Clone)]
pub struct VectorStoreItem {
    pub chunk: Chunk,
    pub embedding: Vec<f32>,
}

/// Estructura que guarda todos los embeddings
#[derive(Debug)]
pub struct VectorStore {
    pub store: Vec<VectorStoreItem>,
}

impl VectorStore {
    pub fn new() -> Self {
        VectorStore { store: Vec::new() }
    }

    /// Añadir un chunk + su embedding al vector store
    pub fn add(&mut self, chunk: Chunk, embedding: Vec<f32>) {
        self.store.push(VectorStoreItem { chunk, embedding });
    }

    /// Buscar los k chunks más parecidos al query embedding
    pub fn search_top_k(&self, query_embedding: &[f32], k: usize) -> Vec<(&VectorStoreItem, f32)> {
        let mut scored_items: Vec<(&VectorStoreItem, f32)> = self
            .store
            .iter()
            .map(|item| {
                let similarity = cosine_similarity(&item.embedding, query_embedding);
                (item, similarity)
            })
            .collect();

        scored_items.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        scored_items.into_iter().take(k).collect()
    }
}

/// Similaridad coseno entre dos vectores
fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    let dot_product: f32 = a.iter().zip(b).map(|(x, y)| x * y).sum();
    let norm_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
    let norm_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();

    if norm_a == 0.0 || norm_b == 0.0 {
        return 0.0;
    }

    dot_product / (norm_a * norm_b)
}
