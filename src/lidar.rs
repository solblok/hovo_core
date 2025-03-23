use std::{thread, time::Duration};
use std::sync::{Arc, Mutex};
use std::process::Command;
use rand::Rng;
use rand::seq::SliceRandom;

fn random_greeting() -> &'static str {
    let frases = [
        "¡Pero bueno, si ahí está mi colega! Ya te estaba oliendo desde lejos.",
    ];
    let mut rng = rand::thread_rng();
    frases.choose(&mut rng).unwrap()
}

// Simulación: esta función debería conectarse al sensor real (ej. por serial o USB)
pub fn read_lidar_front() -> f32 {
    let output = Command::new("python3")
        .arg("scripts/lidar_reader.py") // o la ruta donde tengas el script
        .output()
        .expect("Error ejecutando lidar_reader.py");
    let result = String::from_utf8_lossy(&output.stdout);
    result.trim().parse::<f32>().unwrap_or(9999.0)
}

// Lanza un saludo si detecta algo cerca
pub fn start_lidar_scan<F>(greet: F)
where
    F: Fn(&str) + Send + Sync + 'static,
{
    let greet = Arc::new(greet);

    thread::spawn(move || {
        loop {
            if distance < 1000.0 {
                greet(random_greeting());
                thread::sleep(Duration::from_secs(5)); // Pausa para no greet en bucle
            }

            thread::sleep(Duration::from_millis(500));
        }
    });
}
