

mod automata; // Importamos el módulo automata.rs
mod inf_to_pos;
use crate::inf_to_pos::Token;
fn main() {
    // Definimos el número de estados y las transiciones (estado_origen, carácter_transición, estado_destino)
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

    let postfix:Vec<Token> = inf_to_pos::inf_to_pos(r"[a-z]|(bc)*\*");
    println!("{:?}",postfix);
    // match root {
    //     Some(r) => {
    //         // Imprimimos el nodo raíz y sus transiciones
    
    //         println!("Nodo raíz: {}", r.name);
    //         // Imprimir las transiciones del nodo raíz
    //         for edge in r.edges.borrow().iter() {
    //             match edge.destination.upgrade() {
    //                 Some(destination) => {
    //                     println!(
    //                         "Transición desde {} a {} con el carácter '{}'",
    //                         r.name, destination.name, edge.transition_char
    //                     );
    //                 }
    //                 None => {
    //                     println!("La transición desde {} no tiene un destino válido.", r.name);
    //                 }
    //             }
    //         }
    //     }
    //     None => {
    //         println!("No se pudo generar el AFD.");
    //     }
    // }
}
