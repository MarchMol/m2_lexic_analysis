mod direct_afd;
mod grammar_tree;
mod inf_to_pos;
mod minimize;
mod token_identifier;
mod view;

use crate::direct_afd::DirectAFD;
use crate::inf_to_pos::Token;
use minimize::minimize_dfa;
use token_identifier::asignar_token;
use std::collections::HashSet;
use std::rc::Rc;

fn main() {
    // let regex = r"(a(b|c?d+)[A-Z][0-9]*|x(yz)*z)?w+";
    let regex = r"((if){IF})|([a-z]+{ID})|((ab|d){TEST})";
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

    // let tests = ["acdD999ww", "acA0w", "w", "", "acdD123www", "adAZ"];
    let tests = ["aabasdfdsjfedsf", "if", "ab", "d", "dif9"];
    for &input in &tests {
        let orig = asignar_token(&state_map, input, 'A', &acceptance_states);
        let mini = asignar_token(
            &minimized_map,
            input,
            minimized_start,
            &acceptance_states
        );
        println!("Posibles tokens para {} → original: {:?}, minimizado: {:?}", input, orig, mini);
    }
}
