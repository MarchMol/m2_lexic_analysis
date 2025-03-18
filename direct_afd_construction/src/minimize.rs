pub struct Minimize {

}

impl Minimize {
    pub fn hopcroft_minimize(
        state_map: &HashMap<char, HashMap<char, char>>,
        acceptance_states: &Vec<char>,
    ) -> HashMap<String, Vec<String>> {
        let mut partitions: HashMap<String, Vec<String>> = HashMap::new();
    
        // Separar estados de aceptación y rechazo
        let accept_states: Vec<String> = acceptance_states.iter().map(|s| s.to_string()).collect();
        let reject_states: Vec<String> = state_map
            .keys()
            .filter(|s| !acceptance_states.contains(s))
            .map(|s| s.to_string())
            .collect();
    
        partitions.insert("accept".to_string(), accept_states);
        partitions.insert("reject".to_string(), reject_states);
    
        println!("\n===== Particiones Iniciales =====");
        for (key, states) in &partitions {
            println!("{} -> {:?}", key, states);
        }
    
        let mut stable = false;
        while !stable {
            stable = true;
            println!("\n===== Iteración de refinamiento =====");
            println!("Particiones actuales: {:?}", partitions);
    
            let mut new_partitions: HashMap<String, Vec<String>> = HashMap::new();
    
            for (partition_key, partition_states) in &partitions {
                let mut transition_map: HashMap<Vec<String>, Vec<String>> = HashMap::new();
    
                for state in partition_states {
                    let mut signature: Vec<String> = Vec::new();
                    if let Some(transitions) = state_map.get(&state.chars().next().unwrap()) {
                        for (symbol, next_state) in transitions {
                            signature.push(format!("{}->{}", symbol, next_state));
                        }
                    }
                    signature.sort();
                    transition_map
                        .entry(signature.clone())
                        .or_insert_with(Vec::new)
                        .push(state.clone());
    
                    println!(
                        "Estado '{}' con transiciones {:?} -> Grupo {:?}",
                        state, signature, transition_map
                    );
                }
    
                for (sig, grouped_states) in transition_map {
                    let group_key = format!("{:?}", sig);
                    new_partitions
                        .entry(group_key.clone())
                        .or_insert_with(Vec::new)
                        .extend(grouped_states);
                    println!(
                        "Nueva subpartición '{}' -> {:?}",
                        group_key, new_partitions[&group_key]
                    );
                }
            }
    
            println!(
                "\nNuevas particiones después de refinamiento: {:?}",
                new_partitions
            );
    
            if new_partitions != partitions {
                stable = false;
                partitions = new_partitions;
            }
        }
    
        println!("\n===== Particiones Finales =====");
        for (key, states) in &partitions {
            println!("{} -> {:?}", key, states);
        }
    
        partitions
    }
    
    pub fn build_minimized_afd(
        partitions: HashMap<String, Vec<String>>,
        state_map: &HashMap<char, HashMap<char, char>>,
        symbols: &HashSet<char>,
    ) -> (
        HashMap<String, HashMap<char, String>>,
        HashMap<String, String>,
    ) {
        let mut minimized_afd: HashMap<String, HashMap<char, String>> = HashMap::new();
        let mut partition_to_state: HashMap<String, String> = HashMap::new();
        let mut state_to_partition: HashMap<char, String> = HashMap::new();
        let mut new_state_id = 'A';
    
        println!("\n===== Construcción del AFD Minimizado =====");
    
        // **1️⃣ Asignamos un nuevo estado a cada partición minimizada**
        for (partition_key, states) in &partitions {
            partition_to_state.insert(partition_key.clone(), new_state_id.to_string());
            new_state_id = (new_state_id as u8 + 1) as char;
    
            for state in states {
                let state_char = state.chars().next().unwrap();
                state_to_partition.insert(state_char, partition_key.clone());
            }
        }
    
        // **2️⃣ Construimos las transiciones del AFD Minimizado**
        for (partition_key, partition_states) in &partitions {
            let minimized_state = partition_to_state
                .get(partition_key)
                .expect("❌ ERROR: No se encontró la partición en partition_to_state")
                .clone();
    
            let mut transitions: HashMap<char, String> = HashMap::new();
    
            for state in partition_states {
                let state_char = state.chars().next().unwrap();
    
                if let Some(transitions_for_state) = state_map.get(&state_char) {
                    for symbol in symbols {
                        if let Some(next_state) = transitions_for_state.get(symbol) {
                            if let Some(next_partition_key) = state_to_partition.get(next_state) {
                                if let Some(minimized_next_state) =
                                    partition_to_state.get(next_partition_key)
                                {
                                    transitions.insert(*symbol, minimized_next_state.clone());
                                } else {
                                    println!(
                                        "❌ ERROR: Estado destino '{}' no encontrado en partition_to_state",
                                        next_partition_key
                                    );
                                }
                            } else {
                                println!(
                                    "❌ ERROR: No hay mapeo en state_to_partition para estado '{}'",
                                    next_state
                                );
                            }
                        }
                    }
                }
            }
    
            minimized_afd.insert(minimized_state.clone(), transitions);
        }
    
        println!("\n===== AFD Minimizado Construido =====");
        for (state, transitions) in &minimized_afd {
            println!("Estado: {}", state);
            for (symbol, next_state) in transitions {
                println!("  '{}' -> {}", symbol, next_state);
            }
        }
        println!("==========================================");
    
        (minimized_afd, partition_to_state)
    }
    
    pub fn print_minimized_afd(minimized_afd: &HashMap<String, HashMap<char, String>>) {
        println!("\n===== AFD Minimizado =====");
        for (state, transitions) in minimized_afd {
            println!("Estado: {}", state);
            for (symbol, next_state) in transitions {
                println!("  '{}' -> {}", symbol, next_state);
            }
        }
    }
}