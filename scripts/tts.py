import os
import sys
from io import BytesIO
from elevenlabs import VoiceSettings
from elevenlabs.client import ElevenLabs
from pydub import AudioSegment
from pydub.playback import play

ELEVENLABS_API_KEY = os.getenv("ELEVENLABS_API_KEY")
client = ElevenLabs(api_key="sk_436d68db1001539003a3645cf5f418870e2dbb88aec027c8")

def text_to_speech_stream(text: str) -> BytesIO:
    response = client.text_to_speech.convert(
        voice_id="MVE1ueDWDIdoIS3VEQXS",  # Cambia si usas otro
        output_format="mp3_22050_32",
        text=text,
        model_id="eleven_multilingual_v2",
        voice_settings=VoiceSettings(
            stability=0.7,
            similarity_boost=0.8,
            style=0.2,
            use_speaker_boost=True,
            speed=0.8,
        ),
    )

    audio_stream = BytesIO()
    for chunk in response:
        if chunk:
            audio_stream.write(chunk)
    audio_stream.seek(0)
    return audio_stream

def speak(text: str):
    audio_stream = text_to_speech_stream(text)
    audio = AudioSegment.from_file(audio_stream, format="mp3")
    play(audio)

# Si se ejecuta directamente desde Rust
if __name__ == "__main__":
    texto = sys.argv[1] if len(sys.argv) > 1 else "Hola, soy Hovo"
    speak(texto)