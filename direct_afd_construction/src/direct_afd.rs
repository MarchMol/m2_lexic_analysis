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
    pub fn read_tree(&self) -> HashMap<String, String> {
        let mut labels = HashMap::new();
        let mut literal_count = 1;
        let mut union_count = 1;
        let mut kleene_count = 1;
        let mut concat_count = 1;
    
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
            let left_id = node.get_left().map(|left| traverse(&left, labels, literal_count, union_count, kleene_count, concat_count));
            let right_id = node.get_right().map(|right| traverse(&right, labels, literal_count, union_count, kleene_count, concat_count));
    
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
            traverse(&root_node, &mut labels, &mut literal_count, &mut union_count, &mut kleene_count, &mut concat_count);
        }
    
        labels
    }

    pub fn find_nullable(&self) -> HashMap<String, bool> {
        let tree_map = self.read_tree();
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
      
    pub fn find_first_last_pos(&self) -> (HashMap<String, Vec<String>>, HashMap<String, Vec<String>>) {
        let tree_map = self.read_tree();
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
                if firstpos_map.get(key) != original_firstpos.as_ref() || lastpos_map.get(key) != original_lastpos.as_ref() {
                    changes = true;
                }
            }
        }
        
        // Retornar los dos diccionarios: firstpos_map y lastpos_map
        (firstpos_map, lastpos_map)
    }

    pub fn find_followpos(&self) -> HashMap<String, Vec<String>> {
        let tree_map = self.read_tree();
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
                                followpos_map.entry(num.clone())
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
                                followpos_map.entry(num.clone())
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
    
    pub fn create_states(&mut self) -> (HashMap<char, HashMap<char, Vec<String>>>, Vec<char>) {
        let mut state_map: HashMap<char, HashMap<char, Vec<String>>> = HashMap::new();  // Mapa de estados y sus transiciones
        let mut acceptance_states: Vec<char> = Vec::new();  // Lista de estados de aceptación
        let mut state_queue: HashMap<String, Vec<String>> = HashMap::new();  // Cola de estados por procesar
        let mut visited_states: HashMap<String, Vec<String>> = HashMap::new();  // Para evitar procesar estados duplicados
        let mut state_letter = 'A';
        
        // Obtener el firstpos del nodo raíz
        let root_node = "gama4";
        let root_firstpos = self.find_first_last_pos().0.get(root_node).unwrap_or(&Vec::new()).clone();
        state_queue.insert(state_letter.to_string(), root_firstpos.clone());

        let followpos_map = self.find_followpos();
        let mut labels_map = self.read_tree();
        labels_map.retain(|key, _| followpos_map.contains_key(key));
        let mut columns: HashSet<String> = HashSet::new();
        for (_key, value) in &labels_map {
                if value.starts_with("Literal") {
                    if let Some(c) = value.chars().nth(9) {
                    columns.insert(c.to_string());  // Insertar el valor extraído en el HashSet
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
                
                // Verificar que números en state_value están asociados a la columna
                for number in &state_value {
                    if let Some(symbol) = labels_map.get(number) {
                        if let Some(c) = symbol.chars().nth(9) {
                            if c.to_string() == *column {
                                if let Some(followpos_values) = followpos_map.get(number) {
                                    column_vector.extend(followpos_values.clone());
                                }
                            }
                        }
                    }
                }

                // Si el column_vector tiene elementos, guardamos en state_map
                if !column_vector.is_empty() {
                    // Verificar si el estado creado ya existe 
                    if !visited_states.values().any(|v| v == &column_vector) {
                        state_letter = (state_letter as u8 + 1) as char;
                        state_queue.insert(state_letter.to_string(), column_vector.clone());
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
                    state_map
                        .entry(state_key.chars().next().unwrap()) // Primer char es state_key
                        .or_insert_with(HashMap::new)             // Crear el HashMap si no existe
                        .insert(column.chars().next().unwrap(), column_vector); // Insertar columna y vector
                }
                // println!("Estado actual del visited_states {:?}", visited_states);
                // println!("Estado actual del state_queue {:?}", state_queue);
                // println!("Estado actual del acceptance_states {:?}", acceptance_states);
            }
            
        }

        (state_map, acceptance_states)
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