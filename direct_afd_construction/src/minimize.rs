use std::collections::{HashMap, HashSet};

pub fn minimize_dfa(
    dfa: &HashMap<char, HashMap<String, char>>,
    accept_states: &HashSet<char>,
) -> (HashMap<String, HashMap<String, String>>, String) {
    // Conjunto de todos los estados (Q)
    let mut states: HashSet<char> = dfa.keys().cloned().collect();

    // Calcular el alfabeto: conjunto de símbolos presentes en las transiciones.
    let mut alphabet: HashSet<String> = HashSet::new();
    for transitions in dfa.values() {
        for symbol in transitions.keys() {
            alphabet.insert(symbol.clone());
        }
    }

    // Inicializar la partición P con dos bloques: F (estados de aceptación) y Q \ F (el resto)
    let f: HashSet<char> = accept_states.clone();
    let non_f: HashSet<char> = states.difference(accept_states).cloned().collect();

    let mut P: Vec<HashSet<char>> = Vec::new();
    if !f.is_empty() {
        P.push(f.clone());
    }
    if !non_f.is_empty() {
        P.push(non_f.clone());
    }

    // Inicializar el conjunto de trabajo W con el bloque más pequeño (entre F y Q\F)
    let mut W: Vec<HashSet<char>> = Vec::new();
    if !f.is_empty() && !non_f.is_empty() {
        if f.len() <= non_f.len() {
            W.push(f.clone());
        } else {
            W.push(non_f.clone());
        }
    } else if !f.is_empty() {
        W.push(f.clone());
    } else if !non_f.is_empty() {
        W.push(non_f.clone());
    }

    // --- Algoritmo de Hopcroft ---
    while let Some(a) = W.pop() {
        for symbol in &alphabet {
            // X = { s en Q | δ(s, symbol) ∈ a }
            let mut x: HashSet<char> = HashSet::new();
            for s in &states {
                if let Some(transitions) = dfa.get(s) {
                    if let Some(&t) = transitions.get(symbol) {
                        if a.contains(&t) {
                            x.insert(*s);
                        }
                    }
                }
            }
            // Refinar cada bloque Y en P que tenga una intersección no trivial con X.
            let mut new_P: Vec<HashSet<char>> = Vec::new();
            // Dentro del bucle que recorre cada bloque Y en P:
            for y in P.iter() {
                let intersection: HashSet<char> = y.intersection(&x).cloned().collect();
                let difference: HashSet<char> = y.difference(&x).cloned().collect();
                if !intersection.is_empty() && !difference.is_empty() {
                    // Separamos Y en dos bloques: Y ∩ X y Y \ X.
                    new_P.push(intersection.clone());
                    new_P.push(difference.clone());

                    // Actualizar W: si Y ya estaba en W se reemplaza por ambos bloques,
                    // de lo contrario se agrega el bloque de menor tamaño.
                    let mut found = false;
                    for i in 0..W.len() {
                        if W[i] == *y {
                            // Se elimina el bloque original y se insertan los nuevos.
                            W.remove(i);
                            W.push(intersection.clone());
                            W.push(difference.clone());
                            found = true;
                            break;
                        }
                    }
                    if !found {
                        // Compara los tamaños clonando difference para evitar moverlo.
                        if intersection.len() <= difference.len() {
                            W.push(intersection.clone());
                        } else {
                            W.push(difference.clone());
                        }
                    }
                } else {
                    // Si Y no se ve afectado, se mantiene sin cambios.
                    new_P.push(y.clone());
                }
            }

            P = new_P;
        }
    }
    // --- Fin del algoritmo de Hopcroft ---

    // Ahora cada bloque de P es una clase de equivalencia.
    // Asignamos un nuevo nombre (por ejemplo, "A", "B", ...) a cada bloque.
    let mut state_mapping: HashMap<char, String> = HashMap::new();
    let mut current_name = 'A';
    for block in P.iter() {
        let new_name = current_name.to_string();
        for s in block {
            state_mapping.insert(*s, new_name.clone());
        }
        current_name = ((current_name as u8) + 1) as char;
    }

    // Construir el DFA minimizado.
    // Para cada clase de equivalencia (representada por el nombre asignado), definimos las transiciones.
    let mut minimized: HashMap<String, HashMap<String, String>> = HashMap::new();
    // Se recorre cada bloque (utilizando un representante arbitrario).
    for block in P.iter() {
        // Obtenemos el nombre asignado al bloque.
        let representative = block.iter().next().unwrap();
        let block_name = state_mapping.get(representative).unwrap().clone();

        let mut trans: HashMap<String, String> = HashMap::new();
        // Usamos las transiciones del representante para definir las del bloque.
        if let Some(s_transitions) = dfa.get(representative) {
            for symbol in &alphabet {
                if let Some(&target) = s_transitions.get(symbol) {
                    // La transición irá al bloque al que pertenece el estado destino.
                    if let Some(target_name) = state_mapping.get(&target) {
                        trans.insert(symbol.clone(), target_name.clone());
                    }
                }
            }
        }
        minimized.insert(block_name, trans);
    }

    // Se asume que el estado inicial del DFA original es 'A'
    let start_state = state_mapping.get(&'A').unwrap_or(&"".to_string()).clone();

    (minimized, start_state)
}

/// (Opcional) Función auxiliar para imprimir de forma amigable el DFA minimizado.
pub fn print_minimized_dfa(minimized: &HashMap<String, HashMap<String, String>>) {
    println!("===== DFA Minimizado =====");
    for (state, transitions) in minimized {
        println!("Estado: {}", state);
        for (symbol, target) in transitions {
            println!("  '{}' -> {}", symbol, target);
        }
    }
}
