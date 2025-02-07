use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
mod inf_to_pos;
mod automata;

#[derive(Deserialize, Debug)]
struct AFD {
    Q: Vec<String>,             // Lista de estados
    Σ: Vec<String>,             // Alfabeto
    q0: String,                 // Estado inicial
    F: Vec<String>,             // Conjunto de estados finales
    δ: HashMap<String, String>, // Función de transición (mapeo de transiciones)
}

fn main() {
    // let file = File::open("./automatas/afd_example.json").unwrap();
    // let reader = BufReader::new(file);
    // let afd: AFD = serde_json::from_reader(reader).unwrap();

    // // Imprime los campos de la estructura AFD
    // println!("Q: {:?}", afd.Q);
    // println!("Σ: {:?}", afd.Σ);
    // println!("q0: {}", afd.q0);
    // println!("F: {:?}", afd.F);
    // printl!("δ: {:?}", afd.δ);
    inf_to_pos::inf_to_pos(r"[a-z]xd[0-9]\*");
}
