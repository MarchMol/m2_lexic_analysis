use std::collections::{HashMap, HashSet};

pub fn simulate_afd(state_map: &HashMap<char, HashMap<String, char>>, acceptance_states: &HashSet<char>, input: &str) -> bool {
    let mut current_state = 'A'; // Estado inicial

    for symbol in input.chars() {
        if let Some(transitions) = state_map.get(&current_state) {
            let mut next_state: Option<&char> = None;

            for (key, state) in transitions {
                if key.len() == 1 && key.chars().next().unwrap() == symbol {
                    // Literales
                    next_state = Some(state);
                    break;
                } else if key.contains('-') {
                    // Rangos
                    let parts: Vec<char> = key.chars().collect();
                    if parts.len() == 3 && parts[1] == '-' {
                        let start = parts[0];
                        let end = parts[2];
                        if start <= symbol && symbol <= end {
                            next_state = Some(state);
                            break;
                        }
                    }
                }
            }

            if let Some(&state) = next_state {
                current_state = state;
            } else {
                // No hay transición válida
                return false;
            }
        } else {
            // Estado no tiene transiciones
            return false;
        }
    }

    // Verificar estado final
    acceptance_states.contains(&current_state)
}

