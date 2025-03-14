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
            // println!("Visitando nodo: {:?}", node.get_value());

            // Obtener los identificadores de los hijos (si existen)
            let left_id = node.get_left().map(|left| {
                let id = traverse(
                    &left,
                    labels,
                    literal_count,
                    union_count,
                    kleene_count,
                    concat_count,
                );
                // println!("Nodo izquierdo: {:?} -> ID: {:?}", left.get_value(), id);
                id
            });
            let right_id = node.get_right().map(|right| {
                let id = traverse(
                    &right,
                    labels,
                    literal_count,
                    union_count,
                    kleene_count,
                    concat_count,
                );
                // println!("Nodo izquierdo: {:?} -> ID: {:?}", right.get_value(), id);
                id
            });

            // Asignar identificador al nodo actual
            let node_id = match node.get_value() {
                Token::Literal(c) => {
                    let id = literal_count.to_string();
                    labels.insert(id.clone(), format!("Literal('{}')", c));
                    *literal_count += 1;
                    id
                }
                Token::Range(c, d) => {
                    let id = literal_count.to_string();
                    labels.insert(id.clone(), format!("Range('{},{}')", c, d));
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
                Token::Empty => {
                    let id = "empty".to_string();
                    labels.insert(id.clone(), "Empty".to_string());
                    id
                }
                _ => unreachable!("Unexpected token type in syntax tree"),
            };

            // println!("Asignando etiquieta: {} -> {:?}", node_id, labels.get(&node_id));

            node_id
        }

        // Llamar a la función de recorrido desde la raíz
        if let Some(root_node) = self.syntax_tree.get_root() {
            // println!("Iniciando recorrido desde la raíz");
            // Realizamos el recorrido y asignamos las etiquetas
            root_key = traverse(
                &root_node,
                &mut labels,
                &mut literal_count,
                &mut union_count,
                &mut kleene_count,
                &mut concat_count,
            );

            // println!("Árbol etiquetado: {:?}", labels);
            // println!("Clave raíz: {}", root_key);
        }

        (labels, root_key)
    }

    pub fn find_nullable(&self) -> HashMap<String, bool> {
        let (tree_map, _key) = self.read_tree();
        let mut nullable_map = HashMap::new();
        // println!("Entrada: {:?}", tree_map);

        // Primera pasada: inicializar literales y Sentinel
        for (key, value) in &tree_map {
            if value.starts_with("Literal") {
                nullable_map.insert(key.clone(), false);
                // println!("Inicializando {} como false (Literal)", key);
            } else if value == "Sentinel" {
                nullable_map.insert(key.clone(), false);
                // println!("Inicializando {} como false (Sentinel)", key);
            } else if value == "Empty" {
                nullable_map.insert(key.clone(), true);
                // println!("Inicializando {} como true (Empty)", key);
            } else if value.starts_with("Range") {
                nullable_map.insert(key.clone(), false);
                // println!("Inicializando {} como false (Range)", key);
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
                    // println!("{} es Kleene, siempre true", key);
                } else if key.starts_with("gama") {
                    if let Some((c1, c2)) = extract_children(value) {
                        let nullable_c1 = *nullable_map.get(&c1).unwrap_or(&false);
                        let nullable_c2 = *nullable_map.get(&c2).unwrap_or(&false);
                        nullable_map.insert(key.clone(), nullable_c1 && nullable_c2);
                        // println!("{} = {} AND {} → {}", key, c1, c2, nullable_c1 && nullable_c2);
                    }
                } else if key.starts_with("alpha") {
                    // Union, si un hijo es nullable
                    if let Some((c1, c2)) = extract_children(value) {
                        let nullable_c1 = *nullable_map.get(&c1).unwrap_or(&false);
                        let nullable_c2 = *nullable_map.get(&c2).unwrap_or(&false);
                        nullable_map.insert(key.clone(), nullable_c1 || nullable_c2);
                        // println!("{} = {} OR {} → {}", key, c1, c2, nullable_c1 || nullable_c2);
                    }
                }

                // Si hubo un cambio, marcamos que hay cambios
                if nullable_map.get(key) != original_nullable.as_ref() {
                    // println!("Cambio detectado en {}", key);
                    changes = true;
                }
            }
            // let expected_map = HashMap::from([
            //     ("4".to_string(), false), ("beta2".to_string(), true), ("gama4".to_string(), false), 
            //     ("alpha1".to_string(), false), ("gama7".to_string(), false), ("6".to_string(), false), 
            //     ("alpha3".to_string(), true), ("gama6".to_string(), false), ("7".to_string(), false), 
            //     ("gama1".to_string(), false), ("2".to_string(), false), ("gama5".to_string(), false), 
            //     ("beta3".to_string(), true), ("3".to_string(), false), ("beta1".to_string(), true), 
            //     ("gama2".to_string(), false), ("5".to_string(), false), ("1".to_string(), false), 
            //     ("9".to_string(), false), ("8".to_string(), false), ("alpha2".to_string(), false), 
            //     ("10".to_string(), false), ("gama8".to_string(), false), ("gama3".to_string(), false), 
            //     ("empty".to_string(), true), ("11".to_string(), false)
            // ]);
            
            // println!("Nullable Map Final: {:?}", nullable_map);
            // println!("Coincide con el mapa esperado: {}", nullable_map == expected_map);
        }

        nullable_map
    }

    pub fn find_first_last_pos(&self,) -> (HashMap<String, Vec<String>>, HashMap<String, Vec<String>>) {
        let (tree_map, _key) = self.read_tree();
        let mut firstpos_map: HashMap<String, Vec<String>> = HashMap::new();
        let mut lastpos_map: HashMap<String, Vec<String>> = HashMap::new();

        // Primera pasada: Inicializar Literales
        for (key, value) in &tree_map {
            if value.starts_with("Literal") || value.starts_with("Sentinel") || value.starts_with("Range") {
                // Para Literals, firstpos y lastpos es solo su propia key
                firstpos_map.insert(key.clone(), vec![key.clone()]);
                lastpos_map.insert(key.clone(), vec![key.clone()]);
                // println!("Inicializando {}: firstpos = {:?}, lastpos = {:?}", key, firstpos_map.get(key), lastpos_map.get(key));
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

                        // println!("beta -> {} [Left: {}]: ", key, c1);
                        // println!("firstpos = {:?}, lastpos = {:?}", firstpos_map.get(key), lastpos_map.get(key));
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

                        // println!("alpha -> {} [Left: {}, Right: {}]: ", key, c1, c2);
                        // println!("firstpos = {:?}, lastpos = {:?}", firstpos_map.get(key), lastpos_map.get(key));
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

                        firstpos_map.insert(key.clone(), firstpos);
                        lastpos_map.insert(key.clone(), lastpos);

                        // println!("gama -> {} [Left: {}, Right: {}]: ", key, c1, c2);
                        // println!("firstpos = {:?}, lastpos = {:?}", firstpos_map.get(key), lastpos_map.get(key));
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

        // let expected_fp_map: HashMap<String, Vec<String>> = HashMap::from([
        //     ("alpha1".to_string(), vec!["1".to_string(), "2".to_string()]), ("5".to_string(), vec!["5".to_string()]), ("gama7".to_string(), vec!["1".to_string(), "2".to_string()]),
        //     ("1".to_string(), vec!["1".to_string()]), ("beta3".to_string(), vec!["10".to_string()]), ("gama6".to_string(), vec!["3".to_string(), "4".to_string(), "5".to_string(), "6".to_string(), "7".to_string()]),
        //     ("8".to_string(), vec!["8".to_string()]), ("gama3".to_string(), vec!["7".to_string()]), ("gama1".to_string(), vec!["9".to_string()]),
        //     ("gama4".to_string(), vec!["6".to_string(), "7".to_string()]), ("10".to_string(), vec!["10".to_string()]), ("beta1".to_string(), vec!["3".to_string()]),
        //     ("7".to_string(), vec!["7".to_string()]), ("gama8".to_string(), vec!["1".to_string(), "2".to_string()]), ("11".to_string(), vec!["11".to_string()]),
        //     ("4".to_string(), vec!["4".to_string()]), ("9".to_string(), vec!["9".to_string()]), ("2".to_string(), vec!["2".to_string()]), ("gama5".to_string(), vec!["4".to_string(), "5".to_string(), "6".to_string(), "7".to_string()]),
        //     ("alpha3".to_string(), vec!["6".to_string()]), ("beta2".to_string(), vec!["4".to_string(), "5".to_string()]), ("3".to_string(), vec!["3".to_string()]),
        //     ("alpha2".to_string(), vec!["4".to_string(), "5".to_string()]), ("6".to_string(), vec!["6".to_string()]), ("gama2".to_string(), vec!["8".to_string()]),
        // ]);
    
        // let expected_lp_map: HashMap<String, Vec<String>> = HashMap::from([
        //     ("2".to_string(), vec!["2".to_string()]), ("10".to_string(), vec!["10".to_string()]), ("1".to_string(), vec!["1".to_string()]),
        //     ("8".to_string(), vec!["8".to_string()]), ("9".to_string(), vec!["9".to_string()]), ("gama3".to_string(), vec!["10".to_string(), "9".to_string()]),
        //     ("gama6".to_string(), vec!["10".to_string(), "9".to_string()]), ("alpha3".to_string(), vec!["6".to_string()]), ("gama1".to_string(), vec!["10".to_string(), "9".to_string()]),
        //     ("gama7".to_string(), vec!["10".to_string(), "9".to_string()]), ("5".to_string(), vec!["5".to_string()]), ("11".to_string(), vec!["11".to_string()]),
        //     ("beta3".to_string(), vec!["10".to_string()]), ("4".to_string(), vec!["4".to_string()]), ("7".to_string(), vec!["7".to_string()]),
        //     ("gama8".to_string(), vec!["11".to_string()]), ("3".to_string(), vec!["3".to_string()]), ("6".to_string(), vec!["6".to_string()]),
        //     ("alpha1".to_string(), vec!["1".to_string(), "2".to_string()]), ("gama4".to_string(), vec!["10".to_string(), "9".to_string()]), ("gama2".to_string(), vec!["10".to_string(), "9".to_string()]),
        //     ("beta2".to_string(), vec!["4".to_string(), "5".to_string()]), ("gama5".to_string(), vec!["10".to_string(), "9".to_string()]), ("alpha2".to_string(), vec!["4".to_string(), "5".to_string()]),
        //     ("beta1".to_string(), vec!["3".to_string()]),
        // ]);
    
        // println!("Firstpos Map Final: {:?}", firstpos_map);
        // println!("Coincide con el mapa esperado: {}", firstpos_map == expected_fp_map);
    
        // println!("Lastpos Map Final: {:?}", lastpos_map);
        // println!("Coincide con el mapa esperado: {}", lastpos_map == expected_lp_map);

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
                                // println!("gama -> {}: followpos = {:?}", num, followpos_map.get(num));
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
                                // println!("beta -> {}: followpos = {:?}", num, followpos_map.get(num));
                            }
                        }
                    }
                }
            }
        }

        // Asegurar que todos los literales y sentinels tengan followpos, aunque sea vacío
        for (key, value) in &tree_map {
            if value.starts_with("Literal") || value.starts_with("Sentinel") || value.starts_with("Range") {
                followpos_map.entry(key.clone()).or_insert(Vec::new());
            }
        }

        // Filtrar y eliminar los nodos que no sean Sentinel o Literal
        followpos_map.retain(|key, _| {
            if let Some(value) = tree_map.get(key) {
                value.starts_with("Literal") || value.starts_with("Sentinel") || value.starts_with("Range") 
            } else {
                false
            }
        });
        let followpos_map = normalize_map(&followpos_map);

        // println!("\n===== FOLLOWPOS FINAL =====");
        // let expected_followpos_map = HashMap::from([
        //     ("3".to_string(), vec!["3".to_string(), "4".to_string(), "5".to_string(), "6".to_string(), "7".to_string()]),
        //     ("9".to_string(), vec!["10".to_string(), "11".to_string()]),
        //     ("6".to_string(), vec!["7".to_string()]),
        //     ("11".to_string(), vec![]),
        //     ("10".to_string(), vec!["10".to_string(), "11".to_string()]),
        //     ("1".to_string(), vec!["3".to_string(), "4".to_string(), "5".to_string(), "6".to_string(), "7".to_string()]),
        //     ("8".to_string(), vec!["9".to_string()]),
        //     ("7".to_string(), vec!["8".to_string()]),
        //     ("5".to_string(), vec!["4".to_string(), "5".to_string(), "6".to_string(), "7".to_string()]),
        //     ("2".to_string(), vec!["3".to_string(), "4".to_string(), "5".to_string(), "6".to_string(), "7".to_string()]),
        //     ("4".to_string(), vec!["4".to_string(), "5".to_string(), "6".to_string(), "7".to_string()]),
        // ]);
    
        // println!("Followpos Map Final: {:?}", followpos_map);
        // println!("¿Coincide con el mapa esperado? {}", followpos_map == expected_followpos_map);

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

        println!("Root FirstPos: {:?}", root_firstpos);

        state_queue.insert(state_letter.to_string(), root_firstpos.clone());

        let followpos_map = self.find_followpos();

        labels_map.retain(|key, _| followpos_map.contains_key(key));

        println!("Labels Map after retaining: {:?}", labels_map);

        let mut columns: HashSet<String> = HashSet::new();
        for (_key, value) in &labels_map {
            if value.starts_with("Literal") || value.starts_with("Range") {
                if let Some(start) = value.find('\'') {
                    if let Some(end) = value[start + 1..].find('\'') {
                        let extracted = &value[start + 1..start + 1 + end];
                        columns.insert(extracted.to_string()); // Insertar el valor extraído en el HashSet
                    }
                }
            }
        }

        println!("Columns: {:?}", columns);

        while !state_queue.is_empty() {
            // Obtener el primer estado y removerlo de state_queue
            let (state_key, state_value) = state_queue.drain().next().unwrap();
            println!("Processing state: {} -> {:?}", state_key, state_value);

            // Agregar el estado a visited_states
            visited_states.insert(state_key.clone(), state_value.clone());

            // Crea los valores de cada columna
            for column in &columns {
                let mut column_vector: Vec<String> = Vec::new();
                println!("Processing column: {}", column);

                // Verificar que números en state_value están asociados a la columna
                for number in &state_value {
                    if let Some(symbol) = labels_map.get(number) {
                        if let Some(c) = symbol.chars().nth(9) {
                            if c.to_string() == *column {
                                if let Some(followpos_values) = followpos_map.get(number) {
                                    let mut set: HashSet<_> =
                                        column_vector.iter().cloned().collect();
                                    for val in followpos_values {
                                        if set.insert(val.clone()) {
                                            // insert() returns false if the value already exists
                                            column_vector.push(val.to_string());
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                println!("Column vector: {:?}", column_vector);

                // Si el column_vector tiene elementos, guardamos en state_map
                if !column_vector.is_empty() {
                    // Verificar si el estado creado ya existe
                    let column_set: HashSet<_> = column_vector.iter().cloned().collect();

                    // Verificar si ya existe en visited_states o en state_queue
                    let assigned_letter = visited_states.iter().chain(state_queue.iter()).find_map(|(key, value)| {
                        let v_set: HashSet<_> = value.iter().cloned().collect();
                        if v_set == column_set {
                            Some(key.clone())
                        } else {
                            None
                        }
                    }).or_else(|| {
                        // Si no existe, avanzar la letra y agregarlo a la queue
                        state_letter = (state_letter as u8 + 1) as char;
                        state_queue.insert(state_letter.to_string(), column_vector.clone());
                        println!("New state added to queue: {}", state_letter);
                        Some(state_letter.to_string())
                    });
                    // Verificar si el estado es de aceptación
                    if column_vector.iter().any(|num| {
                        if let Some(symbol) = labels_map.get(num) {
                            symbol.starts_with("Sentinel")
                        } else {
                            false
                        }
                    }) {
                        acceptance_states.push(state_letter);
                        println!("State {} is acceptance state", state_letter);
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
                        
                        println!("Inserted/Updated in state_map: {} -> {} -> {}", state_key, column, assigned_letter);
                    }
                }
            }
        }

        println!("Mapa de Estados: {:?}", state_map);
        println!("Estados de aceptación: {:?}", acceptance_states);

        (state_map, acceptance_states)
    }

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

fn normalize_map(map: &HashMap<String, Vec<String>>) -> HashMap<String, Vec<String>> {
    let mut normalized = HashMap::new();

    for (key, mut value) in map.clone() {
        value.sort(); // Ordena cada vector
        value.dedup(); // Elimina duplicados si existen
        normalized.insert(key, value);
    }

    normalized
}