use std::collections::HashMap;

pub fn simulate_afd(state_map: &HashMap<char, HashMap<char, char>>, acceptance_states: &Vec<char>, input: &str) -> bool {
    let mut current_state = 'A';  // Comenzamos en el estado inicial
    for symbol in input.chars() {
        // Verificar si hay una transición para el símbolo actual
        if let Some(transitions) = state_map.get(&current_state) {
            if let Some(next_state) = transitions.get(&symbol) {
                current_state = *next_state;  // Actualizar el estado actual con la transición
            } else {
                // Si no hay transición definida para el símbolo, el AFD no acepta la cadena
                return false;
            }
        } else {
            // Si no hay transiciones definidas para el estado actual, el AFD no acepta la cadena
            return false;
        }
    }
    
    // Verificar si el estado final es un estado de aceptación
    acceptance_states.contains(&current_state)
}
