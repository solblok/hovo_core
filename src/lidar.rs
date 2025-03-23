use std::{thread, time::Duration};
use std::sync::{Arc, Mutex};
use rand::Rng;
use rand::seq::SliceRandom;

fn random_greeting() -> &'static str {
    let frases = [
        "¡Eh tú, tigre! ¿Vienes a contarme algo interesante o qué?",
        "¿Qué pasa, boss? ¿Quién se acerca por ahí?",
        "Qué susto, tronco! Avisa antes de aparecer así de la nada.",
        "¡Chavalote! Ya te estaba oliendo desde lejos.",
        "¿Vienes a liarla o solo a mirar, niño?"
    ];
    let mut rng = rand::thread_rng();
    frases.choose(&mut rng).unwrap()
}

// Simulación: esta función debería conectarse al sensor real (ej. por serial o USB)
pub fn read_lidar_front() -> f32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0.5..3.5)
}

// Lanza un saludo si detecta algo cerca
pub fn start_lidar_scan<F>(greet: F)
where
    F: Fn(&str) + Send + Sync + 'static,
{
    let greet = Arc::new(greet);

    thread::spawn(move || {
        loop {
            let distance = read_lidar_front();

            if distance < 1.5 {
                greet(random_greeting());
                thread::sleep(Duration::from_secs(5)); // Pausa para no greet en bucle
            }

            thread::sleep(Duration::from_millis(500));
        }
    });
}
