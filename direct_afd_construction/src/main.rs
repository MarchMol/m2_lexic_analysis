use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

#[derive(Deserialize, Debug)]
struct AFD {
    Q: Vec<String>,             // Lista de estados
    Σ: Vec<String>,             // Alfabeto
    q0: String,                 // Estado inicial
    F: Vec<String>,             // Conjunto de estados finales
    δ: HashMap<String, String>, // Función de transición (mapeo de transiciones)
}

fn main() {
    let file = File::open("./automatas/afd_example.json").unwrap();
    let reader = BufReader::new(file);
    let afd: AFD = serde_json::from_reader(reader).unwrap();

    // Imprime los campos de la estructura AFD
    println!("Q: {:?}", afd.Q);
    println!("Σ: {:?}", afd.Σ);
    println!("q0: {}", afd.q0);
    println!("F: {:?}", afd.F);
    println!("δ: {:?}", afd.δ);
}
