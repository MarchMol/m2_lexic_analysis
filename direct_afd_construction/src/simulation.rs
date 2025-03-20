use std::collections::{HashMap, HashSet};

pub fn simulate_afd(
    state_map: &HashMap<char, HashMap<String, char>>, 
    acceptance_states: &HashSet<char>, 
    input: &str,
    first_state: char
) -> bool {
    let mut current_state = first_state;
    let mut chars = input.chars().peekable();
    let mut remaining = input;

    // println!("--- INICIANDO SIMULACIÓN ---");
    // println!("Cadena de entrada: \"{}\"", input);
    // println!("Estado inicial: '{}'\n", current_state);

    while let Some(symbol) = chars.peek().copied() {
        // println!("Estado actual: '{}', símbolo a procesar: '{}'", current_state, symbol);
        // println!("Cadena restante: \"{}\"", remaining);

        if let Some(transitions) = state_map.get(&current_state) {
            // println!("Posibles transiciones desde '{}': {:?}", current_state, transitions);
            let mut next_state: Option<&char> = None;

            for (key, state) in transitions {
                // println!("  Probando clave de transición: \"{}\"", key);

                if key.len() == 1 && key.chars().next().unwrap() == symbol {
                    // Literales
                    // println!("  → Coincidencia exacta con literal '{}'", key);
                    next_state = Some(state);
                    chars.next();
                    remaining = &remaining[1..];
                    break;
                } else if key.contains('-') {
                    // Rangos
                    let parts: Vec<char> = key.chars().collect();
                    if parts.len() == 3 && parts[1] == '-' {
                        let start = parts[0];
                        let end = parts[2];
                        if start <= symbol && symbol <= end {
                            // println!("  → Coincidencia en rango '{}'", key);
                            next_state = Some(state);
                            chars.next();
                            remaining = &remaining[1..];
                            break;
                        }
                    }
                } else if remaining.starts_with(key) {
                    // Tokens completos
                    // println!("  → Coincidencia con token \"{}\"", key);
                    next_state = Some(state);
                    for _ in 0..key.len() {
                        chars.next();
                    }
                    remaining = &remaining[key.len()..];
                    break;
                }
            }

            if let Some(&state) = next_state {
                // println!("Transición exitosa: '{}' → '{}'\n", current_state, state);
                current_state = state;
            } else {
                // println!("No se encontró transición válida desde '{}'", current_state);
                return false;
            }
        } else {
            // println!("Estado '{}' no tiene transiciones definidas", current_state);
            return false;
        }
    }

    let accepted = acceptance_states.contains(&current_state);
    // println!(
    //     "\n--- SIMULACIÓN FINALIZADA ---\nEstado final: '{}'\nResultado: {}\n",
    //     current_state,
    //     if accepted { "ACEPTADO" } else { "RECHAZADO" }
    // );
    
    accepted
}
