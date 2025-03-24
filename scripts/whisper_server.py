from fastapi import FastAPI, File, UploadFile
import tempfile
import whisper

app = FastAPI()
model = whisper.load_model("base")

@app.post("/transcribe")
async def transcribe(audio: UploadFile = File(...)):
    with tempfile.NamedTemporaryFile(delete=True, suffix=".wav") as tmp:
        tmp.write(await audio.read())
        tmp.flush()
        result = model.transcribe(tmp.name)
        return {"text": result["text"]}