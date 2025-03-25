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

fn generate(regx: String)->(HashMap<char, HashMap<String, char>>,HashSet<char>,char,Vec<String>){
    let postfix: Vec<Token> = inf_to_pos::inf_to_pos(&regx);
    let mut gtree = grammar_tree::Tree::new();
    let root = gtree.generate(postfix);
    println!(
                "Árbol Sintáctico:\n{}",
                (*root).clone().print_tree(0, "root\n")
            );
    let gtree_ref = Rc::new(gtree);
    let mut afd = DirectAFD::new(gtree_ref);
    afd.generate_afd();
    let (state_map, acceptance_states, token_list) = afd.create_states();
    let (minimized_map, minimized_accept_states, minimized_start) =
    minimize_dfa(&state_map, &acceptance_states);
    (minimized_map, minimized_accept_states,minimized_start, token_list)
}

fn get_token_type(input: String, 
    minimized_map: &HashMap<char, HashMap<String, char>>, 
    minimized_accept_states:  &HashSet<char>,
    minimized_start: char,
    token_list: Vec<String>,
)-> String {
        let mini = asignar_token(
            &minimized_map,
            &input,
            minimized_start,
            &minimized_accept_states,
            &token_list,
        );
        mini
}

fn simulate(
    input: String, 
    minimized_map: HashMap<char, HashMap<String, char>>, 
    minimized_accept_states:  HashSet<char>,
    minimized_start: char,
    token_list: Vec<String>,
)->Vec<String>{
    
    let mut char_it = input.chars().into_iter().peekable();
    let mut stack: Vec<char> = Vec::new();
    let mut tk_list: Vec<String> = Vec::new(); 
    let mut pos = 0;
    while let Some(c) = char_it.peek(){
        stack.push(*c);
        let tem_str = String::from_iter(stack.clone());
        let tem = get_token_type( 
        tem_str.to_string(), &minimized_map, &minimized_accept_states, minimized_start, token_list.clone());
        // println!("Pos{:?}, Current {}, tk: {:?}", pos, tem_str, tk_list);
        if !tem.is_empty(){  // Not yet biggest
            char_it.next();
            if pos < tk_list.len() {
                tk_list[pos] =format!("{:?}",tem);
            } else {
                tk_list.push(format!("{:?}",tem));
            }
        } else{ //  Biggest prefix
            stack.drain(..);
            pos+=1;
        }
        // println!("{:?}",tem);
    }
    tk_list
}
fn main(){
    let input = "while num > 2.4";
    let test = compile::gen_reg();
    // println!("{:?}",test);
    let (minimized_map, minimized_accept_states,minimized_start, token_list) = generate(test);
    let toks = simulate(input.to_string(), minimized_map, minimized_accept_states, minimized_start, token_list);
    println!("{:?}",toks);
}

// fn main() {
//     // let regex = r"(a(b|c?d+)[A-Z][0-9]*|x(yz)*z)?w+";
//     let regex = r"((if){IF})|((ab|d){TEST})|([a-z]+{ID})";
//     println!("Regex: {}", regex);

//     let postfix: Vec<Token> = inf_to_pos::inf_to_pos(regex);
//     println!("Postfix tokens: {:?}\n", postfix);

//     let mut gtree = grammar_tree::Tree::new();
//     let root = gtree.generate(postfix);
//     println!(
//         "Árbol Sintáctico:\n{}",
//         (*root).clone().print_tree(0, "root\n")
//     );

//     let gtree_ref = Rc::new(gtree);
//     let mut afd = DirectAFD::new(gtree_ref);
//     afd.generate_afd();
//     let (state_map, acceptance_states, token_list) = afd.create_states();

//     println!("\n===== DFA Original =====");
//     for (s, t) in &state_map {
//         println!("Estado {} -> {:?}", s, t);
//     }
//     println!("Aceptación original: {:?}\n", acceptance_states);
//     println!("Lista de tokens: {:?}", token_list);

//     let (minimized_map, minimized_accept_states, minimized_start) =
//         minimize_dfa(&state_map, &acceptance_states);

//     println!("===== DFA Minimizado =====");
//     for (s, t) in &minimized_map {
//         println!("Estado {} -> {:?}", s, t);
//     }
//     println!("Aceptación minimizada: {:?}", minimized_accept_states);
//     println!("Estado inicial minimizado: {}\n", minimized_start);

//     // let tests = ["acdD999ww", "acA0w", "w", "", "acdD123www", "adAZ"];
//     let tests = ["aabasdfdsjfedsf", "if", "ab", "d", "dif9"];
//     for &input in &tests {
//         let orig = asignar_token(&state_map, input, 'A', &acceptance_states, &token_list);
//         // Se modifica aquí: se pasa el conjunto de aceptación minimizado.
//         let mini = asignar_token(
//             &minimized_map,
//             input,
//             minimized_start,
//             &minimized_accept_states,
//             &token_list,
//         );
//         println!(
//             "Posibles tokens para {} → original: {:?}, minimizado: {:?}",
//             input, orig, mini
//         );
//     }
// }
