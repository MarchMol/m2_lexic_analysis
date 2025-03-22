use std::collections::{HashMap, HashSet};

fn leer_cadena(
    state_map: &HashMap<char, HashMap<String, char>>,
    input: &str,
    first_state: char,
) -> HashSet<char> {
    let mut current_states = HashSet::new();
    let mut next_state = HashSet::new();
    current_states.insert(first_state);
    let mut chars = input.chars().peekable();
    let mut remaining = input;

    // println!("--- INICIANDO SIMULACIÓN ---");
    // println!("Cadena de entrada: \"{}\"", input);
    // println!("Estado inicial: '{:?}'\n", current_states);

    while let Some(symbol) = chars.peek().copied() {
        // println!("Símbolo a procesar: '{}'", symbol);
        // println!("Cadena restante: \"{}\"", remaining);

        for &current_state in &current_states {
            if let Some(transitions) = state_map.get(&current_state) {
                // println!("Posibles transiciones desde '{}': {:?}", current_state, transitions);

                for (key, &state) in transitions {
                    // println!("  Probando clave de transición: \"{}\"", key);

                    if key.len() == 1 && key.chars().next().unwrap() == symbol {
                        // Literales
                        // println!("  → Coincidencia exacta con literal '{}'", key);
                        next_state.insert(state);
                    } else if key.contains('-') {
                        // Rangos
                        let parts: Vec<char> = key.chars().collect();
                        if parts.len() == 3 && parts[1] == '-' {
                            let start = parts[0];
                            let end = parts[2];
                            if start <= symbol && symbol <= end {
                                // println!("  → Coincidencia en rango '{}'", key);
                                next_state.insert(state);
                            }
                        }
                    }
                }
            }
        }

        if next_state.is_empty() {
            // println!("No se encontraron más transiciones.");
            return HashSet::new();
        }

        chars.next();
        remaining = &remaining[1..];
        current_states = next_state.clone();
        next_state.clear();
    }
    // println!("\n--- SIMULACIÓN FINALIZADA ---\n");
    // println!("Estados alcanzados: {:?}", current_states);

    current_states
}

pub fn asignar_token(
    state_map: &HashMap<char, HashMap<String, char>>,
    input: &str,
    first_state: char,
    acceptance_states: &HashSet<char>,
) -> HashSet<String> {
    let last_state_list = leer_cadena(state_map, input, first_state);
    let mut valid_transitions = HashSet::new();

    for &state in &last_state_list {
        if let Some(transitions) = state_map.get(&state) {
            for (transition, &next_state) in transitions {
                if acceptance_states.contains(&next_state) {
                    valid_transitions.insert(transition.clone()); // Guardamos la transición en lugar del estado
                }
            }
        }
    }

    valid_transitions
}
