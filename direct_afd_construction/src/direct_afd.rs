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

    pub fn read_tree(&self) -> HashMap<String, Token> {
        let mut labels = HashMap::new();
        let mut literal_count = 1;
        let mut union_count = 1;
        let mut kleene_count = 1;
        let mut concat_count = 1;
    
        // Función recursiva que recorre el árbol y asigna los valores
        pub fn traverse(
            
            node: &TreeNode,
            labels: &mut HashMap<String, Token>,
            literal_count: &mut usize,
            union_count: &mut usize,
            kleene_count: &mut usize,
            concat_count: &mut usize,
        ) {
            // Primero recorrer el subárbol izquierdo
            if let Some(left) = node.get_left() {
                traverse(&left, labels, literal_count, union_count, kleene_count, concat_count);
            }
    
            // Luego recorrer el subárbol derecho
            if let Some(right) = node.get_right() {
                traverse(&right, labels, literal_count, union_count, kleene_count, concat_count);
            }
    
            // Asignar etiquetas según el tipo de nodo
            match node.get_value() {
                Token::Literal(c) => {
                    labels.insert(literal_count.to_string(), node.get_value().clone());
                    *literal_count += 1;
                }
                Token::Union => {
                    labels.insert(format!("alpha{}", *union_count), node.get_value().clone());
                    *union_count += 1;
                }
                Token::Kleene => {
                    labels.insert(format!("beta{}", *kleene_count), node.get_value().clone());
                    *kleene_count += 1;
                }
                Token::Concat => {
                    labels.insert(format!("gama{}", *concat_count), node.get_value().clone());
                    *concat_count += 1;
                }
                Token::Sentinel => {
                    labels.insert(literal_count.to_string(), node.get_value().clone());
                    *literal_count += 1;
                }
                _ => unreachable!("Unexpected token type in syntax tree"),
            }
        }
        
        // Llamar a la función de recorrido desde la raíz
        if let Some(root_node) = self.syntax_tree.get_root() {
            
            traverse(&root_node, &mut labels, &mut literal_count, &mut union_count, &mut kleene_count, &mut concat_count);
        }
    
        labels
    }
     

    fn find_nullable(&mut self) {
        // Encontrar anulables en el árbol sintáctico
    }

    fn find_first_last_pos(&mut self) {
        // Encontrar firstpos y lastpos de las hojas y transiciones
    }

    fn find_followpos(&mut self) {
        // Encontrar nodos de concatenación y kleene para followpos
    }

    fn create_states(&mut self) {
        // Poner el firstpos del nodo raíz como primer estado
        // Asociar números con columnas
        // Colocar la unión de los followpos en la transición de dicho símbolo
    }
}
