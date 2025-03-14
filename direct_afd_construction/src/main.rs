mod direct_afd;
mod grammar_tree;
mod inf_to_pos;
mod simulation;
mod view;

use simulation::simulate_afd;
// use std::os::linux::raw::stat;
use std::rc::Rc;

use crate::direct_afd::DirectAFD;
use crate::inf_to_pos::Token;
use std::collections::HashMap;
use std::collections::HashSet;

// Función que convierte el AFD minimizado de HashMap<String, HashMap<char, String>> a HashMap<char, HashMap<char, char>>
fn convert_minimized_afd_to_original(
    minimized_afd: &HashMap<String, HashMap<char, String>>, // AFD minimizado
) -> HashMap<char, HashMap<char, char>> {
    let mut original_afd: HashMap<char, HashMap<char, char>> = HashMap::new();

    // Recorremos cada estado minimizado y sus transiciones
    for (state, transitions) in minimized_afd {
        // Convertimos la clave del estado a char
        let state_char = state
            .chars()
            .next()
            .expect("El estado debe tener al menos un caracter");
        let mut new_transitions: HashMap<char, char> = HashMap::new();

        // Convertimos cada transición: símbolo -> estado destino
        for (symbol, next_state_str) in transitions {
            let next_state_char = next_state_str
                .chars()
                .next()
                .expect("El estado destino debe tener al menos un caracter");
            new_transitions.insert(*symbol, next_state_char);
        }

        original_afd.insert(state_char, new_transitions);
    }

    original_afd
}

fn main() {
    // 1. Convertimos la regex a postfix (a|b)*c(d|e)+f?
    let postfix: Vec<Token> = inf_to_pos::inf_to_pos(r"(a|b)c*(d|e)*f?[g-k][0-5]l+");
    println!("{:?}", postfix);
    // 2. Inicializamos el grammar tree
    let mut gtree = grammar_tree::Tree::new();
    let root = gtree.generate(postfix);
    let tree_str = (*root).clone().print_tree(0, "root\n");
    println!("{}", tree_str);
    let gtree_ref = Rc::new(gtree);

    // Problema 1: no sopora rangos
    // Problema 2: que implemente la libreria de grafos
    // Problema 3: a veces da un resultado y a veces otro

    // // 3. Inicializamos el AFD
    let mut afd = DirectAFD::new(gtree_ref);

    // // 4. Asignamos etiquetas a los nodos del árbol
    let (labels, root_node) = afd.read_tree();

    // // 5. Calculamos los valores de nulabilidad
    let nullable_map = afd.find_nullable();

    // // // 6. Calculamos los firstpos y lastpos
    let (firstpos_map, lastpos_map) = afd.find_first_last_pos();

    // // // 7. Calculamos el followpos
    let followpos_map = afd.find_followpos();

    // // // 8. Generamos los estados y el AFD
    let (state_map, acceptance_states) = afd.create_states();
    // // // Render

    // view::render(&state_map, &acceptance_states, "afd");

    // // // 9. Aplicamos el algoritmo de Hopcroft para minimizar el AFD
    // let partitions = direct_afd::DirectAFD::hopcroft_minimize(
    //     &state_map, // Esto ahora debe ser de tipo HashMap<char, HashMap<char, Vec<String>>>
    //     &acceptance_states,
    // );

    // // // 10. Construimos el AFD minimizado
    // let (minimized_afd, partition_to_state) = direct_afd::DirectAFD::build_minimized_afd(
    //     partitions,
    //     &state_map,
    //     &afd.find_first_last_pos()
    //         .0
    //         .keys()
    //         .flat_map(|s| s.chars())
    //         .collect::<HashSet<char>>(),
    // );

    // println!("===== AFD Minimizado (antes de render) =====");
    // for (state, transitions) in &minimized_afd {
    //     println!("Estado: {}", state);
    //     for (symbol, next_state) in transitions {
    //         println!("  '{}' -> {}", symbol, next_state);
    //     }
    // }
    // println!("==========================================");

    // // // // 11. Imprimimos el AFD minimizado
    // direct_afd::DirectAFD::print_minimized_afd(&minimized_afd); // Pasamos una referencia aquí

    // //13. Simulamos el AFD minimizado con el input "abb"
    // let input = "ababcddf";

    // println!("===== DEPURACIÓN: RECORRIDO DEL AFD =====");

    // // Estado inicial del AFD minimizado (ajustar si es diferente)
    // let mut current_state = 'A'; // Usamos `char` en lugar de `String`
    // println!("Estado inicial: {}", current_state);

    // for symbol in input.chars() {
    //     if let Some(transitions) = state_map.get(&current_state) {
    //         if let Some(&next_state) = transitions.get(&symbol) {
    //             println!("✅ {} --({})--> {}", current_state, symbol, next_state);
    //             current_state = next_state; // Ahora es un `char`, no hay error de tipo
    //         } else {
    //             println!(
    //                 "🚨 Error: No hay transición para '{}' con '{}'",
    //                 current_state, symbol
    //             );
    //             break;
    //         }
    //     } else {
    //         println!("🚨 Error: Estado desconocido '{}'", current_state);
    //         break;
    //     }
    // }

    // println!("===== FIN DEPURACIÓN =====");

    // // Simulación final del AFD minimizado
    // let verificar = simulate_afd(&state_map, &acceptance_states, &input);
    // println!("🔎 La simulación dice que este input es: {}", verificar);

    // let verificar = simulate_afd(&state_map, &acceptance_states, &input);
    // println!("La simulación dice que este input es: {}", verificar);
}
