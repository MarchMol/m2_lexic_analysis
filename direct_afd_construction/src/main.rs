mod direct_afd;
mod grammar_tree;
mod inf_to_pos;
mod minimize;
mod token_identifier;
mod view;

use crate::direct_afd::DirectAFD;
use crate::inf_to_pos::Token;
use minimize::minimize_dfa;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use token_identifier::asignar_token;
mod lex_reader;
mod reader;

mod compile;

fn generate(regx: String)->(HashMap<char, HashMap<String, char>>,HashSet<char>,char){
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
    let (state_map, acceptance_states) = afd.create_states();
    let (minimized_map, minimized_accept_states, minimized_start) =
    minimize_dfa(&state_map, &acceptance_states);
    (minimized_map, minimized_accept_states,minimized_start)
}

fn getTtype(input: String, 
    minimized_map: &HashMap<char, HashMap<String, char>>, 
    minimized_accept_states:  &HashSet<char>,
    minimized_start: char)->HashSet<String>{
        let mini = asignar_token(
            &minimized_map,
            &input,
            minimized_start,
            &minimized_accept_states,
        );
        mini
}

fn simulate(
    input: String, 
    minimized_map: HashMap<char, HashMap<String, char>>, 
    minimized_accept_states:  HashSet<char>,
    minimized_start: char)->Vec<String>{
    let mut tk_list: Vec<String> = Vec::new();
    let len = input.len();
    let mut last_start = 0;
    let mut condition = false;

    while !condition{
        let mut lexem = String::new();
        let mut greedy_match: HashSet<String>= HashSet::new();
        let mut greedy_end = 0;
        for i in last_start..len{
            let c = input.char_indices().nth(i).map(|(_, c)| c).unwrap();
            lexem.push(c);
            let cmatch = getTtype( 
                lexem.to_string(), &minimized_map, &minimized_accept_states, minimized_start);
            if !cmatch.is_empty(){
                greedy_match = cmatch;
                greedy_end = i
            }
        }
        greedy_end+=1;
        if last_start>=greedy_end{
            panic!("Unexpected token {}",lexem);
        } else{
            let biggest_lex:String = input.chars().skip(last_start).take(greedy_end - last_start).collect();
            println!("FINAL ({}-{}) Lex: \"{}\", match: {:?}", last_start, greedy_end,biggest_lex, greedy_match);
        }
        
        if greedy_match.is_empty(){
            last_start+=1;
        } else{
            tk_list.push(format!("{:?}",greedy_match));
            last_start = greedy_end;
        }
        if greedy_end == len{
            condition=true;
        }
    }
    tk_list
}
fn main(){
    let input = "while -1 23 0. 523.523 -0.0";
    let test = compile::gen_reg();
    // println!("{:?}",test);
    let (minimized_map, minimized_accept_states,minimized_start) = generate(test);
    let toks = simulate(input.to_string(), minimized_map, minimized_accept_states, minimized_start);
    println!("{:?}",toks);
}