mod direct_afd;
mod grammar_tree;
mod inf_to_pos;
mod minimize;
mod simulation;
mod view;

use crate::direct_afd::DirectAFD;
use crate::inf_to_pos::Token;
use minimize::minimize_dfa;
use simulation::simulate_afd;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

fn main() {
    // 1. Convertir la expresión regular a tokens en notación postfix.
    let regex = r"(a(b|c?d+)[A-Z][0-9]*|x(yz)*z)?w+";
    println!("Regex: {}", regex);
    let postfix: Vec<Token> = inf_to_pos::inf_to_pos(regex);
    println!("Postfix tokens: {:?}\n", postfix);

    // 2️. Generar el grammar tree e imprimirlo.
    let mut gtree = grammar_tree::Tree::new();
    let root = gtree.generate(postfix);
    println!(
        "Árbol Sintáctico:\n{}",
        (*root).clone().print_tree(0, "root\n")
    );

    // 3️. Inicializar el AFD (DFA) directo usando el árbol generado.
    let gtree_ref = Rc::new(gtree);
    let mut afd = DirectAFD::new(gtree_ref);
    afd.generate_afd();
    let (state_map, acceptance_states) = afd.create_states();
    println!("\n===== DFA Original =====");
    for (state, trans) in &state_map {
        println!("Estado {} -> {:?}", state, trans);
    }
    println!(
        "Estados de aceptación originales: {:?}\n",
        acceptance_states
    );

    // 4️. Minimización del DFA.
    let (minimized_map, minimized_accept_states, minimized_start) =
        minimize_dfa(&state_map, &acceptance_states);
    println!("===== DFA Minimizado =====");
    for (state, trans) in &minimized_map {
        println!("Estado {} -> {:?}", state, trans);
    }
    println!(
        "Estados de aceptación minimizados: {:?}",
        minimized_accept_states
    );
    println!("Estado inicial minimizado: {}\n", minimized_start);

    // 5️. Simulación sobre ambos DFA.
    let test_inputs = ["acdD999ww", "acA0w", "w", "W", "", "acdD123www", "adAZ"];
    for &input in &test_inputs {
        println!("--- Probando input: \"{}\" ---", input);
        let original_result = simulate_afd(&state_map, &acceptance_states, input, 'A');
        let minimized_result = simulate_afd(
            &minimized_map,
            &minimized_accept_states,
            input,
            minimized_start,
        );
        println!("Original acepta?    {}", original_result);
        println!("Minimizado acepta? {}", minimized_result);
        println!();
    }
}
