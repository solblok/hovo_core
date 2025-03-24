from fastapi import FastAPI
from pydantic import BaseModel
from sentence_transformers import SentenceTransformer
from fastapi.middleware.cors import CORSMiddleware

# Inicializa el modelo de embeddings
model = SentenceTransformer("all-MiniLM-L6-v2")

# Crea la app FastAPI
app = FastAPI()

# CORS para peticiones desde otros orígenes
app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],
    allow_methods=["*"],
    allow_headers=["*"],
)

# Estructura del body de entrada
class EmbeddingRequest(BaseModel):
    text: str

# Estructura de respuesta
class EmbeddingResponse(BaseModel):
    embedding: list[float]

# Ruta principal para generar embedding
@app.post("/embed", response_model=EmbeddingResponse)
def embed(req: EmbeddingRequest):
    vector = model.encode(req.text).tolist()
    return {"embedding": vector}
