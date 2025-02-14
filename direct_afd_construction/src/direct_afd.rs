use std::clone;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use crate::grammar_tree::{Tree, TreeNode};
use crate::inf_to_pos::Token;

pub struct DirectAFD {
    syntax_tree: Rc<Tree>,
    followpos: HashMap<usize, HashSet<usize>>,
    states: Vec<HashSet<usize>>,
    transitions: HashMap<(usize, char), usize>,
}

impl DirectAFD {
    pub fn new(tree: Rc<Tree>) -> Self {
        Self {
            syntax_tree: tree,
            followpos: HashMap::new(),
            states: Vec::new(),
            transitions: HashMap::new(),
        }
    }

    pub fn generate_afd(&mut self) {
        self.read_tree();
        self.find_nullable();
        self.find_first_last_pos();
        self.find_followpos();
        self.create_states();
    }

    // Lee el árbol y guarda sus labels
    pub fn read_tree(&self) -> (HashMap<String, String>, String) {
        let mut labels = HashMap::new();
        let mut literal_count = 1;
        let mut union_count = 1;
        let mut kleene_count = 1;
        let mut concat_count = 1;
        let mut root_key = String::new();

        // Función recursiva que recorre el árbol y asigna etiquetas
        fn traverse(
            node: &TreeNode,
            labels: &mut HashMap<String, String>,
            literal_count: &mut usize,
            union_count: &mut usize,
            kleene_count: &mut usize,
            concat_count: &mut usize,
        ) -> String {
            // Obtener los identificadores de los hijos (si existen)
            let left_id = node.get_left().map(|left| {
                traverse(
                    &left,
                    labels,
                    literal_count,
                    union_count,
                    kleene_count,
                    concat_count,
                )
            });
            let right_id = node.get_right().map(|right| {
                traverse(
                    &right,
                    labels,
                    literal_count,
                    union_count,
                    kleene_count,
                    concat_count,
                )
            });

            // Asignar identificador al nodo actual
            let node_id = match node.get_value() {
                Token::Literal(c) => {
                    let id = literal_count.to_string();
                    labels.insert(id.clone(), format!("Literal('{}')", c));
                    *literal_count += 1;
                    id
                }
                Token::Union => {
                    let id = format!("alpha{}", *union_count);
                    *union_count += 1;
                    if let (Some(c1), Some(c2)) = (left_id.clone(), right_id.clone()) {
                        labels.insert(id.clone(), format!("({}, {})", c1, c2));
                    }
                    id
                }
                Token::Kleene => {
                    let id = format!("beta{}", *kleene_count);
                    *kleene_count += 1;
                    if let Some(c1) = left_id.clone() {
                        labels.insert(id.clone(), format!("({})", c1));
                    }
                    id
                }
                Token::Concat => {
                    let id = format!("gama{}", *concat_count);
                    *concat_count += 1;
                    if let (Some(c1), Some(c2)) = (left_id.clone(), right_id.clone()) {
                        labels.insert(id.clone(), format!("({}, {})", c1, c2));
                    }
                    id
                }
                Token::Sentinel => {
                    let id = literal_count.to_string();
                    labels.insert(id.clone(), "Sentinel".to_string());
                    *literal_count += 1;
                    id
                }
                _ => unreachable!("Unexpected token type in syntax tree"),
            };

            node_id
        }

        // Llamar a la función de recorrido desde la raíz
        if let Some(root_node) = self.syntax_tree.get_root() {
            // Realizamos el recorrido y asignamos las etiquetas
            root_key = traverse(
                &root_node,
                &mut labels,
                &mut literal_count,
                &mut union_count,
                &mut kleene_count,
                &mut concat_count,
            );
        }

        (labels, root_key)
    }

    pub fn find_nullable(&self) -> HashMap<String, bool> {
        let (tree_map, _key) = self.read_tree();
        let mut nullable_map = HashMap::new();

        // Primera pasada: inicializar literales y Sentinel
        for (key, value) in &tree_map {
            if value.starts_with("Literal") {
                let content = value.trim_start_matches("Literal('").trim_end_matches("')");
                nullable_map.insert(key.clone(), content == "ε");
            } else if value == "Sentinel" {
                nullable_map.insert(key.clone(), false);
            }
        }

        // Fijación: Realizar múltiples pasadas hasta que no haya cambios
        let mut changes = true;
        while changes {
            changes = false;

            // Segunda pasada: calcular valores de Kleene, Concat y Union
            for (key, value) in &tree_map {
                let original_nullable = nullable_map.get(key).cloned();

                if key.starts_with("beta") {
                    // Kleene (beta): El valor siempre es true
                    nullable_map.insert(key.clone(), true);
                } else if key.starts_with("gama") {
                    if let Some((c1, c2)) = extract_children(value) {
                        let nullable_c1 = *nullable_map.get(&c1).unwrap_or(&false);
                        let nullable_c2 = *nullable_map.get(&c2).unwrap_or(&false);
                        nullable_map.insert(key.clone(), nullable_c1 && nullable_c2);
                    }
                } else if key.starts_with("alpha") {
                    if let Some((c1, c2)) = extract_children(value) {
                        let nullable_c1 = *nullable_map.get(&c1).unwrap_or(&false);
                        let nullable_c2 = *nullable_map.get(&c2).unwrap_or(&false);
                        nullable_map.insert(key.clone(), nullable_c1 || nullable_c2);
                    }
                }

                // Si hubo un cambio, marcamos que hay cambios
                if nullable_map.get(key) != original_nullable.as_ref() {
                    changes = true;
                }
            }
        }

        nullable_map
    }

    pub fn find_first_last_pos(
        &self,
    ) -> (HashMap<String, Vec<String>>, HashMap<String, Vec<String>>) {
        let (tree_map, _key) = self.read_tree();
        let mut firstpos_map: HashMap<String, Vec<String>> = HashMap::new();
        let mut lastpos_map: HashMap<String, Vec<String>> = HashMap::new();

        // Primera pasada: Inicializar Literales
        for (key, value) in &tree_map {
            if value.starts_with("Literal") || value.starts_with("Sentinel") {
                // Para Literals, firstpos y lastpos es solo su propia key
                firstpos_map.insert(key.clone(), vec![key.clone()]);
                lastpos_map.insert(key.clone(), vec![key.clone()]);
            }
        }

        // Fijación: Realizar múltiples pasadas hasta que no haya cambios
        let mut changes = true;
        while changes {
            changes = false;

            // Segunda pasada: Procesar los nodos no Literales
            for (key, value) in &tree_map {
                let original_firstpos = firstpos_map.get(key).cloned();
                let original_lastpos = lastpos_map.get(key).cloned();

                if key.starts_with("beta") {
                    // Kleene (beta): Igualar firstpos y lastpos al nodo al que está conectado
                    if let Some(c1) = extract_single_child(value) {
                        let firstpos = firstpos_map.get(&c1).cloned().unwrap_or_default();
                        let lastpos = lastpos_map.get(&c1).cloned().unwrap_or_default();
                        firstpos_map.insert(key.clone(), firstpos);
                        lastpos_map.insert(key.clone(), lastpos);
                    }
                } else if key.starts_with("alpha") {
                    // Union (alpha): Unir firstpos y lastpos de los nodos izquierdo y derecho
                    if let Some((c1, c2)) = extract_children(value) {
                        let firstpos_c1 = firstpos_map.get(&c1).cloned().unwrap_or_default();
                        let firstpos_c2 = firstpos_map.get(&c2).cloned().unwrap_or_default();
                        let lastpos_c1 = lastpos_map.get(&c1).cloned().unwrap_or_default();
                        let lastpos_c2 = lastpos_map.get(&c2).cloned().unwrap_or_default();

                        // Unión de firstpos y lastpos
                        let firstpos = [&firstpos_c1[..], &firstpos_c2[..]].concat();
                        let lastpos = [&lastpos_c1[..], &lastpos_c2[..]].concat();

                        firstpos_map.insert(key.clone(), firstpos);
                        lastpos_map.insert(key.clone(), lastpos);
                    }
                } else if key.starts_with("gama") {
                    // Concat (gama): Verificar nullabilidad para firstpos y lastpos
                    if let Some((c1, c2)) = extract_children(value) {
                        let firstpos_c1 = firstpos_map.get(&c1).cloned().unwrap_or_default();
                        let firstpos_c2 = firstpos_map.get(&c2).cloned().unwrap_or_default();
                        let lastpos_c1 = lastpos_map.get(&c1).cloned().unwrap_or_default();
                        let lastpos_c2 = lastpos_map.get(&c2).cloned().unwrap_or_default();

                        let nullable_c1 = *self.find_nullable().get(&c1).unwrap_or(&false);
                        let nullable_c2 = *self.find_nullable().get(&c2).unwrap_or(&false);

                        // println!(
                        //     "Procesando nodo {} (Concat):\n  - Hijo izquierdo: {}\n  - Hijo derecho: {}\n  - Nullable Izq: {}\n  - Nullable Der: {}",
                        //     key, c1, c2, nullable_c1, nullable_c2
                        // );

                        // println!(
                        //     "  - Firstpos izquierdo: {:?}\n  - Firstpos derecho: {:?}",
                        //     firstpos_c1, firstpos_c2
                        // );
                        // println!(
                        //     "  - Lastpos izquierdo: {:?}\n  - Lastpos derecho: {:?}",
                        //     lastpos_c1, lastpos_c2
                        // );

                        // Firstpos: Si el izquierdo es nullable, hacer unión con el derecho
                        let firstpos = if nullable_c1 {
                            [&firstpos_c1[..], &firstpos_c2[..]].concat()
                        } else {
                            firstpos_c1
                        };

                        // Lastpos: Si el derecho es nullable, hacer unión con el izquierdo
                        let lastpos = if nullable_c2 {
                            [&lastpos_c2[..], &lastpos_c1[..]].concat()
                        } else {
                            lastpos_c2
                        };

                        // println!(
                        //     "  - Firstpos final: {:?}\n  - Lastpos final: {:?}",
                        //     firstpos, lastpos
                        // );

                        firstpos_map.insert(key.clone(), firstpos);
                        lastpos_map.insert(key.clone(), lastpos);
                    }
                }

                // Si hubo un cambio, marcamos que hay cambios
                if firstpos_map.get(key) != original_firstpos.as_ref()
                    || lastpos_map.get(key) != original_lastpos.as_ref()
                {
                    changes = true;
                }
            }
        }

        // Retornar los dos diccionarios: firstpos_map y lastpos_map
        (firstpos_map, lastpos_map)
    }

    pub fn find_followpos(&self) -> HashMap<String, Vec<String>> {
        let (tree_map, _key) = self.read_tree();
        let mut followpos_map: HashMap<String, Vec<String>> = HashMap::new();

        // Obtener firstpos y lastpos con la función existente
        let (firstpos_map, lastpos_map) = self.find_first_last_pos();

        // Inicializar followpos con listas vacías para todos los nodos
        for key in tree_map.keys() {
            followpos_map.insert(key.clone(), Vec::new());
        }

        // Iterar sobre los nodos para encontrar "gama" (concatenación) y "beta" (Kleene)
        for (key, value) in &tree_map {
            if key.starts_with("gama") {
                // Concat (gama): Followpos de lastpos del hijo izquierdo es firstpos del hijo derecho
                if let Some((c1, c2)) = extract_children(value) {
                    if let Some(lastpos_c1) = lastpos_map.get(&c1) {
                        if let Some(firstpos_c2) = firstpos_map.get(&c2) {
                            for num in lastpos_c1 {
                                followpos_map
                                    .entry(num.clone())
                                    .and_modify(|e| e.extend(firstpos_c2.clone()))
                                    .or_insert(firstpos_c2.clone());
                            }
                        }
                    }
                }
            } else if key.starts_with("beta") {
                // Kleene (beta): Followpos de lastpos del nodo es firstpos del mismo nodo
                if let Some(c1) = extract_single_child(value) {
                    if let Some(lastpos_c1) = lastpos_map.get(&c1) {
                        if let Some(firstpos_c1) = firstpos_map.get(&c1) {
                            for num in lastpos_c1 {
                                followpos_map
                                    .entry(num.clone())
                                    .and_modify(|e| e.extend(firstpos_c1.clone()))
                                    .or_insert(firstpos_c1.clone());
                            }
                        }
                    }
                }
            }
        }

        // Asegurar que todos los literales y sentinels tengan followpos, aunque sea vacío
        for (key, value) in &tree_map {
            if value.starts_with("Literal") || value.starts_with("Sentinel") {
                followpos_map.entry(key.clone()).or_insert(Vec::new());
            }
        }

        // Filtrar y eliminar los nodos que no sean Sentinel o Literal
        followpos_map.retain(|key, _| {
            if let Some(value) = tree_map.get(key) {
                value.starts_with("Literal") || value.starts_with("Sentinel")
            } else {
                false
            }
        });

        followpos_map
    }

    pub fn create_states(&mut self) -> (HashMap<char, HashMap<char, char>>, Vec<char>) {
        let mut state_map: HashMap<char, HashMap<char, char>> = HashMap::new(); // Mapa de estados y sus transiciones
        let mut acceptance_states: Vec<char> = Vec::new(); // Lista de estados de aceptación
        let mut state_queue: HashMap<String, Vec<String>> = HashMap::new(); // Cola de estados por procesar
        let mut visited_states: HashMap<String, Vec<String>> = HashMap::new(); // Para evitar procesar estados duplicados
        let mut state_letter = 'A';
        
        // Obtener el firstpos del nodo raíz
        let (mut labels_map, root_key) = self.read_tree();
        let root_firstpos = self
            .find_first_last_pos()
            .0
            .get(&root_key)
            .unwrap_or(&Vec::new())
            .clone();
        
        state_queue.insert(state_letter.to_string(), root_firstpos.clone());
        
        let followpos_map = self.find_followpos();
       
        labels_map.retain(|key, _| followpos_map.contains_key(key));
        
        let mut columns: HashSet<String> = HashSet::new();
        for (_key, value) in &labels_map {
            if value.starts_with("Literal") {
                if let Some(c) = value.chars().nth(9) {
                    columns.insert(c.to_string()); // Insertar el valor extraído en el HashSet
                }
            }
        }
        
        while !state_queue.is_empty() {
            // Obtener el primer estado y removerlo de state_queue
            let (state_key, state_value) = state_queue.drain().next().unwrap();
            // Agregar el estado a visited_states
            visited_states.insert(state_key.clone(), state_value.clone());
            // Crea los valores de cada columna
            for column in &columns {
                let mut column_vector: Vec<String> = Vec::new();
                let mut assigned_letter = None;

                // Verificar que números en state_value están asociados a la columna
                for number in &state_value {
                    if let Some(symbol) = labels_map.get(number) {
                        if let Some(c) = symbol.chars().nth(9) {
                            if c.to_string() == *column {
                                if let Some(followpos_values) = followpos_map.get(number) {
                                    let mut set: HashSet<_> = column_vector.iter().cloned().collect();
                                    for val in followpos_values {
                                        if set.insert(val.clone()) { // insert() returns false if the value already exists
                                            column_vector.push(val.to_string());
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                // Si el column_vector tiene elementos, guardamos en state_map
                if !column_vector.is_empty() {
                    // Verificar si el estado creado ya existe
                    if !visited_states.values().any(|v: &Vec<String>| {
                        
                        let mut v_sorted = v.clone();
                        v_sorted.sort();
                        let mut column_vector_sorted = column_vector.clone();
                        column_vector_sorted.sort();

                        v_sorted == column_vector_sorted
                    }) {
                        state_letter = (state_letter as u8 + 1) as char;
                        println!("column vector: {:?}",column_vector);
                        state_queue.insert(state_letter.to_string(), column_vector.clone());
                    }

                    // Busca la letra asignada a un estado
                    if let Some(existing_letter) = visited_states.iter().find_map(|(key, value)| {
                        if value == &column_vector {
                            Some(key)
                        } else {
                            None
                        }
                    }) {
                        assigned_letter = Some(existing_letter.clone());
                    }
                    if assigned_letter.is_none() {
                        if let Some(existing_letter) =
                            state_queue.iter().find_map(|(key, value)| {
                                if value == &column_vector {
                                    Some(key)
                                } else {
                                    None
                                }
                            })
                        {
                            assigned_letter = Some(existing_letter.clone());
                        }
                    }

                    // Verificar si el estado es de aceptación
                    if column_vector.iter().any(|num| {
                        if let Some(symbol) = labels_map.get(num) {
                            symbol.starts_with("Sentinel")
                        } else {
                            false
                        }
                    }) {
                        acceptance_states.push(state_letter);
                    }

                    // Insertar o actualizar el valor en state_map
                    if let Some(assigned_letter) = assigned_letter {
                        state_map
                            .entry(state_key.chars().next().unwrap())
                            .or_insert_with(HashMap::new)
                            .insert(
                                column.chars().next().unwrap(),
                                assigned_letter.chars().next().unwrap(),
                            );
                    }
                }
                // println!("Estado actual del visited_states {:?}", visited_states);
                // println!("Visitando la columna {}", column);
                // println!("Followpos de la columna: {:?}", column_vector);
                // println!("Estado actual del state_queue {:?}", state_queue);
                // println!("Estado actual del acceptance_states {:?}", acceptance_states);
            }
        }

        (state_map, acceptance_states)
    }

    pub fn hopcroft_minimize(
        state_map: &HashMap<char, HashMap<char, char>>,
        acceptance_states: &Vec<char>,
    ) -> HashMap<String, Vec<String>> {
        let mut partitions: HashMap<String, Vec<String>> = HashMap::new();

        // Partición inicial: aceptación vs no aceptación
        let mut accept_states: Vec<String> = Vec::new();
        for state in acceptance_states {
            accept_states.push(state.to_string());
        }
        let mut reject_states: Vec<String> = Vec::new();
        for state in state_map.keys() {
            if !accept_states.contains(&state.to_string()) {
                reject_states.push(state.to_string());
            }
        }

        partitions.insert("accept".to_string(), accept_states);
        partitions.insert("reject".to_string(), reject_states);

        println!("Particiones iniciales:");
        for (key, value) in &partitions {
            println!("{}: {:?}", key, value);
        }

        let mut stable = false;
        while !stable {
            stable = true;
            let mut new_partitions: HashMap<String, Vec<String>> = HashMap::new();

            for (partition_key, partition_states) in partitions.iter() {
                let mut transition_map: HashMap<char, Vec<String>> = HashMap::new();

                // Para cada estado de la partición, obtenemos sus transiciones
                for state in partition_states {
                    if let Some(transitions) = state_map.get(&state.chars().next().unwrap()) {
                        for (symbol, next_state) in transitions {
                            transition_map
                                .entry(*symbol)
                                .or_insert_with(Vec::new)
                                .push(next_state.to_string());
                        }
                    }
                }

                for (symbol, grouped_states) in &transition_map {
                    println!("  Símbolo: {} -> Estados: {:?}", symbol, grouped_states);
                }

                // Si no hay transiciones para esta partición, la copiamos tal cual
                if transition_map.is_empty() {
                    new_partitions.insert(partition_key.clone(), partition_states.clone());
                } else {
                    // Agrupar los estados en nuevas subparticiones según sus transiciones
                    for (symbol, mut grouped_states) in transition_map {
                        grouped_states.sort();
                        let group_key = format!("{:?}", grouped_states);
                        new_partitions
                            .entry(group_key)
                            .or_insert_with(Vec::new)
                            .extend(grouped_states);
                    }
                }
            }

            for (key, value) in &new_partitions {}

            if new_partitions != partitions {
                stable = false;
                partitions = new_partitions;
            }
        }

        for (key, value) in &partitions {}

        partitions
    }

    pub fn refine_partitions(
        partitions: &mut HashMap<String, Vec<String>>, // Particiones actuales
        state_map: &HashMap<char, HashMap<char, Vec<String>>>, // Mapa de transiciones
        symbols: &HashSet<char>,                       // Símbolos de entrada
    ) {
        let mut new_partitions: HashMap<String, Vec<String>> = HashMap::new();

        // Depuración: Verificar particiones antes de la refinación
        for (key, value) in partitions.iter() {}

        // Para cada partición, vamos a verificar las transiciones
        for (partition_key, partition_states) in partitions.iter() {
            let mut transition_map: HashMap<char, Vec<String>> = HashMap::new();

            // Para cada estado en la partición, verificamos su transición bajo cada símbolo
            for state in partition_states {
                if let Some(transitions) = state_map.get(&state.chars().next().unwrap()) {
                    for symbol in symbols {
                        if let Some(next_states) = transitions.get(symbol) {
                            // Asignar el estado de transición a la subpartición correcta
                            for next_state in next_states {
                                transition_map
                                    .entry(*symbol)
                                    .or_insert_with(Vec::new)
                                    .push(next_state.clone());
                            }
                        }
                    }
                }
            }

            // Depuración: Verificar las transiciones de los estados en cada partición
            for (symbol, grouped_states) in &transition_map {}

            // Agrupar los estados en nuevas subparticiones según sus transiciones
            for (symbol, grouped_states) in transition_map {
                let group_key = format!("{:?}", grouped_states);
                new_partitions
                    .entry(group_key)
                    .or_insert_with(Vec::new)
                    .extend(grouped_states);
            }
        }

        // Depuración: Verificar las particiones refinadas
        for (key, value) in &new_partitions {}

        // Actualizar las particiones con las nuevas subparticiones
        *partitions = new_partitions;
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

        let mut new_state_id = 'A';

        // Asignamos un estado minimizado a cada partición usando la clave de la partición
        for (partition_key, _) in &partitions {
            partition_to_state.insert(partition_key.clone(), new_state_id.to_string());
            new_state_id = (new_state_id as u8 + 1) as char;
        }

        // Crear un mapeo de cada estado individual a su partición (clave de partición)
        let mut state_to_partition: HashMap<char, String> = HashMap::new();
        for (part_key, states) in &partitions {
            for state in states {
                let state_char = state.chars().next().unwrap();
                state_to_partition.insert(state_char, part_key.clone());
            }
        }

        // Asignar las transiciones para cada partición
        for (partition_key, partition_states) in &partitions {
            let minimized_state = partition_to_state.get(partition_key).unwrap();
            let mut transitions: HashMap<char, String> = HashMap::new();

            // Recorremos cada estado de la partición y sus transiciones
            for state in partition_states {
                let state_char = state.chars().next().unwrap();

                if let Some(transitions_for_state) = state_map.get(&state_char) {
                    for symbol in symbols {
                        if let Some(next_state) = transitions_for_state.get(symbol) {
                            // Usamos el mapeo state_to_partition para obtener la partición del siguiente estado
                            if let Some(next_partition_key) = state_to_partition.get(next_state) {
                                if let Some(minimized_next_state) =
                                    partition_to_state.get(next_partition_key)
                                {
                                    transitions.insert(*symbol, minimized_next_state.clone());
                                }
                            }
                        }
                    }
                }
            }

            minimized_afd.insert(minimized_state.clone(), transitions);
        }

        // Depuración: Imprimir el AFD minimizado resultante
        for (state, transitions) in &minimized_afd {
            for (symbol, next_state) in transitions {}
        }

        (minimized_afd, partition_to_state)
    }

    pub fn print_minimized_afd(minimized_afd: &HashMap<String, HashMap<char, String>>) {
        // Iterar sobre los estados minimizados
        for (state, transitions) in minimized_afd {
            // Iterar sobre las transiciones de cada estado
            for (symbol, next_state) in transitions {}
        }
    }
}

fn extract_children(value: &str) -> Option<(String, String)> {
    let content = value.trim_start_matches('(').trim_end_matches(')');
    let parts: Vec<&str> = content.split(", ").collect();
    if parts.len() == 2 {
        Some((parts[0].to_string(), parts[1].to_string()))
    } else {
        None
    }
}

fn extract_single_child(value: &str) -> Option<String> {
    let content = value.trim_start_matches('(').trim_end_matches(')');
    Some(content.to_string())
}
