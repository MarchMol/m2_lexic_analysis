use crate::direct_afd::DirectAFD;
use crate::inf_to_pos::Token;
use crate::inf_to_pos;
use crate::grammar_tree;
use crate::minimize::minimize_dfa;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;
pub fn generate(regx: String)->(HashMap<char, HashMap<String, char>>,HashSet<char>,char,Vec<String>){
    let postfix: Vec<Token> = inf_to_pos::inf_to_pos(&regx);
    let mut gtree = grammar_tree::Tree::new();
    let root = gtree.generate(postfix);
    // println!(
    //             "Árbol Sintáctico:\n{}",
    //             (*root).clone().print_tree(0, "root\n")
    //         );
    let gtree_ref = Rc::new(gtree);
    let mut afd = DirectAFD::new(gtree_ref);
    afd.generate_afd();
    let (state_map, acceptance_states, token_list) = afd.create_states();
    let (minimized_map, minimized_accept_states, minimized_start) =
    minimize_dfa(&state_map, &acceptance_states);
    (minimized_map, minimized_accept_states,minimized_start, token_list)
}