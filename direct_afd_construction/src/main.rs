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
use std::collections::HashSet;
use std::rc::Rc;

fn main() {
    let regex = r"(a(b|c?d+)[A-Z][0-9]*|x(yz)*z)?w+";
    println!("Regex: {}", regex);

    let postfix: Vec<Token> = inf_to_pos::inf_to_pos(regex);
    println!("Postfix tokens: {:?}\n", postfix);

    let mut gtree = grammar_tree::Tree::new();
    let root = gtree.generate(postfix);
    println!(
        "Árbol Sintáctico:\n{}",
        (*root).clone().print_tree(0, "root\n")
    );

    let gtree_ref = Rc::new(gtree);
    let mut afd = DirectAFD::new(gtree_ref);
    afd.generate_afd();
    let (state_map, acceptance_states) = afd.create_states();

    println!("\n===== DFA Original =====");
    for (s, t) in &state_map {
        println!("Estado {} -> {:?}", s, t);
    }
    println!("Aceptación original: {:?}\n", acceptance_states);

    let (minimized_map, minimized_accept_states, minimized_start) =
        minimize_dfa(&state_map, &acceptance_states);

    println!("===== DFA Minimizado =====");
    for (s, t) in &minimized_map {
        println!("Estado {} -> {:?}", s, t);
    }
    println!("Aceptación minimizada: {:?}", minimized_accept_states);
    println!("Estado inicial minimizado: {}\n", minimized_start);

    let tests = ["acdD999ww", "acA0w", "w", "", "acdD123www", "adAZ"];
    for &input in &tests {
        let orig = simulate_afd(&state_map, &acceptance_states, input, 'A');
        let mini = simulate_afd(
            &minimized_map,
            &minimized_accept_states,
            input,
            minimized_start,
        );
        println!("{} → original: {}, minimizado: {}", input, orig, mini);
    }
}
