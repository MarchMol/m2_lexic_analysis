mod automata; // Importamos el módulo automata.rs
mod direct_afd;
mod grammar_tree;
mod inf_to_pos;
mod simulation;
mod view;
use simulation::simulate_afd;
use std::rc::Rc;

use crate::direct_afd::DirectAFD;

use crate::inf_to_pos::Token;
use std::collections::HashSet;

fn main() {
    // 1. Convertimos la regex a postfix
    let postfix: Vec<Token> = inf_to_pos::inf_to_pos(r"1234#");

    // 2. Inicializamos el grammar tree
    let mut gtree = grammar_tree::Tree::new();

    // Generamos el árbol a partir de la postfix
    let root = gtree.generate(postfix);
    let gtree_ref = Rc::new(gtree);

    // 3. Inicializamos el AFD
    let mut afd = DirectAFD::new(gtree_ref);

    // 4. Asignamos etiquetas a los nodos del árbol
    let (labels, root_node) = afd.read_tree();

    // 5. Calculamos los valores de nulabilidad
    let nullable_map = afd.find_nullable();

    // 6. Calculamos los firstpos y lastpos
    let (firstpos_map, lastpos_map) = afd.find_first_last_pos();

    // 7. Calculamos el followpos
    let followpos_map = afd.find_followpos();

    // 8. Generamos los estados y el AFD
    let (state_map, acceptance_states) = afd.create_states();
    
    // Render
    view::render(&state_map, "afd");
    
    // // 9. Aplicamos el algoritmo de Hopcroft para minimizar el AFD
    let partitions = direct_afd::DirectAFD::hopcroft_minimize(
        &state_map,
        &acceptance_states,
        &afd.find_first_last_pos()
            .0
            .keys()
            .flat_map(|s| s.chars())
            .collect::<HashSet<char>>(),
    );

    // // 10. Construimos el AFD minimizado


    // // 11. Imprimimos el AFD minimizado
    direct_afd::DirectAFD::print_minimized_afd(minimized_afd);

    // // // 12. Simulamos el AFD minimizado con el input "abb"
    let input = "abb";
    let verificar = simulate_afd(&minimized_afd, &acceptance_states, &input);
    println!("La simulación dice que este input es: {}", verificar);
}
