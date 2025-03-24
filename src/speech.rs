use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use hound::{WavSpec, WavWriter};
use reqwest::blocking::Client;
use reqwest::blocking::multipart;
use serde_json;
use std::error::Error;
use std::fs::File;
use std::io::BufWriter;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

/// Graba del micro y guarda un archivo WAV
pub fn record_audio(output_path: &str, duration_secs: u64) {
    let host = cpal::default_host();
    let device = host
        .default_input_device()
        .expect("No se encontró un micro disponible");
    let config = device.default_input_config().unwrap();

    let spec = WavSpec {
        channels: 1,
        sample_rate: config.sample_rate().0,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let writer = Arc::new(Mutex::new(
        WavWriter::new(BufWriter::new(File::create(output_path).unwrap()), spec).unwrap(),
    ));

    let writer_clone = Arc::clone(&writer);

    let stream = match config.sample_format() {
        cpal::SampleFormat::I16 => device
            .build_input_stream(
                &config.into(),
                move |data: &[i16], _: &cpal::InputCallbackInfo| {
                    let mut writer = writer_clone.lock().unwrap();
                    for &sample in data {
                        writer.write_sample(sample).unwrap();
                    }
                },
                err_fn,
                None,
            )
            .unwrap(),
        _ => panic!("Formato de audio no soportado"),
    };

    stream.play().unwrap();
    println!("🎙 Grabando durante {} segundos...", duration_secs);
    thread::sleep(Duration::from_secs(duration_secs));
    println!("🛑 Grabación finalizada");

    match Arc::try_unwrap(writer) {
        Ok(mutex) => match mutex.into_inner() {
            Ok(mut writer) => {
                writer.finalize().unwrap();
            }
            Err(_) => eprintln!("💥 No se pudo obtener el WavWriter del Mutex"),
        },
        Err(_) => eprintln!("💥 No se pudo liberar el Arc del WavWriter"),
    }
}

fn err_fn(err: cpal::StreamError) {
    eprintln!("💥 Error en stream: {}", err);
}

/// Sube un archivo .wav al servidor Whisper y recibe la transcripción
pub fn transcribe_audio(audio_path: &str) -> Result<String, Box<dyn Error>> {
    let client = Client::new();

    let form = multipart::Form::new()
        .file("audio", audio_path)
        .expect("No se pudo crear el form para enviar el audio");

    let response = client
        .post("http://127.0.0.1:8001/transcribe")
        .multipart(form)
        .send()?;

    if response.status().is_success() {
        let json: serde_json::Value = response.json()?;
        Ok(json["text"].as_str().unwrap_or("").to_string())
    } else {
        Err(format!("Error del servidor Whisper: {}", response.status()).into())
    }
}
