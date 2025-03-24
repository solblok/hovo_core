use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Debug, Clone)]
pub struct Chunk {
    pub id: String,
    pub content: String,
}

pub fn load_chunks_from_file<P: AsRef<Path>>(file_path: P) -> Vec<Chunk> {
    let file = File::open(file_path).expect("Couldn't open the file");
    let reader = BufReader::new(file);

    let mut chunks = Vec::new();

    for (i, line) in reader.lines().enumerate() {
        if let Ok(text) = line {
            if text.trim().is_empty() {
                continue;
            }

            chunks.push(Chunk {
                id: format!("chunk_{}", i),
                content: text.trim().to_string(),
            });
        }
    }

    chunks
}
