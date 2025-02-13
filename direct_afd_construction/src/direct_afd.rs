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
    
    pub fn create_states(&self) {
        
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