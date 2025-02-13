mod automata; // Importamos el módulo automata.rs
mod inf_to_pos;
mod grammar_tree;
mod direct_afd;
mod simulation;

use std::rc::Rc;
use simulation::simulate_afd;

use crate::inf_to_pos::Token;
use crate::direct_afd::DirectAFD;

fn main() {

    //  Primero, convertimos la regex a postfix :[a-z]|(bc)*\*
    let postfix:Vec<Token> = inf_to_pos::inf_to_pos(r"(a|b)*abb#");
    // println!("{:?}",&postfix);

    // Despues inicializamos el grammar tree
    let mut gtree = grammar_tree::Tree::new();
    
    // Ya con el gtree, podemos generar el arbol a partir de la postfix
    let root = gtree.generate(postfix);
    // Este value, solo es el resultado de llamar a la función printTree, que te regresa una string con masomenos el arbol
    let value = (*root).clone().printTree(0, "root: ");
    // Si imprimimos value, nos va a salir el arbol
    // print!("{}",value);
    let gtree_ref = Rc::new(gtree);

    // Inicializamos el AFD
    let mut afd = DirectAFD::new(gtree_ref);
    // Asignamos etiquetas a los nodos del árbol
    
    // Asignamos etiquetas a los nodos del árbol
    let (labels, root_node) = afd.read_tree();
    // println!("Etiquetas de los nodos: {:?}", labels);
    // println!("Nodo raíz: {}", root_node);

    // Calculamos los valores de nulabilidad
    let nullable_map = afd.find_nullable();
    // println!("Nullable de los nodos: {:?}", nullable_map);
 
    // Calculamos los firstpos y lastpos
    let (firstpos_map, lastpos_map) = afd.find_first_last_pos();
     
    // Imprimimos los resultados de firstpos
    // println!("Firstpos de los nodos:");
    // for (key, firstpos) in firstpos_map {
    //     println!("Nodo: {} => Firstpos: {:?}", key, firstpos);
    // }
 
    // Imprimimos los resultados de lastpos
    // println!("Lastpos de los nodos:");
    // for (key, lastpos) in lastpos_map {
    //     println!("Nodo: {} => Lastpos: {:?}", key, lastpos);
    // }

    // Calculamos el followpos
    let followpos_map = afd.find_followpos();
    // println!("Followpos de los nodos:");
    // for (key, followpos) in &followpos_map {
    //     println!("Nodo: {} => Followpos: {:?}", key, followpos);
    // }

    // Hacemos los estados y el AFD
    let (state_map, acceptance_states) = afd.create_states();
    // println!("Estado de AFD: ");
    // for (state, transitions) in &state_map {
    //     println!("Estado: {}", state);
    //     for (column, column_vector) in transitions {
    //         println!("  Columna: {}", column);
    //         println!("    Valores de followpos: {:?}", column_vector);
    //     }
    // }
    // println!("\nEstados de aceptación: {:?}", acceptance_states);

    // Llamamos a la simulación
    let input = "abb";
    let verificar = simulate_afd(&state_map, &acceptance_states, &input);
    println!("La simulación dice que este input es: {}", verificar)

}

fn automata_stuff(){
    let mut grafo = automata::Graph::new();
    let quantity_states = 3;
    let transitions = vec![
        (0, 'a', 1), // Transición de s0 a s1 con el carácter 'a'
        (1, 'b', 2), // Transición de s1 a s2 con el carácter 'b'
        (2, 'a', 0), // Transición de s2 a s0 con el carácter 'a'
    ];
    grafo.generate(quantity_states, transitions);
    // Generamos el AFD usando la función `generate` de automata.rs
    let root = grafo.get_root();

        match root {
        Some(r) => {
            // Imprimimos el nodo raíz y sus transiciones
    
            println!("Nodo raíz: {}", r.name);
            // Imprimir las transiciones del nodo raíz
            for edge in r.edges.borrow().iter() {
                match edge.destination.upgrade() {
                    Some(destination) => {
                        println!(
                            "Transición desde {} a {} con el carácter '{}'",
                            r.name, destination.name, edge.transition_char
                        );
                    }
                    None => {
                        println!("La transición desde {} no tiene un destino válido.", r.name);
                    }
                }
            }
        }
        None => {
            println!("No se pudo generar el AFD.");
        }
    }
}
