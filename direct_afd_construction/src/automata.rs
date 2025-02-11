use core::borrow;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::{Rc, Weak};

#[derive(Debug)]
pub struct Node {
    pub name: String,
    pub edges: RefCell<Vec<Rc<Edge>>>,
}

#[derive(Debug)]
pub struct Edge {
    pub destination: Weak<Node>, // Weak reference to avoid memory cycles
    pub transition_char: char,
}

pub struct Graph {
    nodes: Rc<RefCell<HashMap<usize, Rc<Node>>>>,
    root: Option<Rc<Node>>,
}
impl Graph {
    pub fn new() -> Self{
        Self{
            nodes: Rc::new(RefCell::new(HashMap::new())),
            root: None,
        }
    }
    pub fn get_root(&self) ->Option<Rc<Node>>{
        self.root.clone()
    }
    pub fn generate(
        &mut self,
        quantity_states: usize,
        transitions: Vec<(usize, char, usize)>,
    ){
        // Crear los nodos
        for state in 0..quantity_states {
            let name = format!("s{}",state);
            let temp = Rc::new(Node {
                name: name.clone(),
                edges: RefCell::new(vec![]),
            });
            self.nodes.borrow_mut().insert(state,Rc::clone(&temp));
    
            // Debug: Imprimir cuando se crea un nodo
            println!("Nodo creado: {}", name);
    
            // Si es el primer nodo, lo asignamos como root
            if state == 0 {
                self.root = Some(Rc::clone(&temp));
            }
        }
    
        // Crear las transiciones entre los nodos
        // Crear las transiciones entre los nodos
    
            for tr in &transitions {
    
                let nodes_ref = self.nodes.borrow();
                    // Crear la transición con referencia débil al nodo destino
                if let(Some(from_node), Some(to_node)) = (nodes_ref.get(&tr.0), nodes_ref.get(&tr.2)){
                    let edge = Rc::new(Edge {
                        destination: Rc::downgrade(to_node), // Creamos una referencia débil directamente
                        transition_char: tr.1,
                    });
                    // Debug: Imprimir la transición que se crea
                    println!(
                        "Creando transición: {} -> {} con el carácter '{}'",
                        from_node.name,
                        to_node.name,
                        tr.1 // Cambié destination_rc.name a nodes[tr.2].name
                    );
                    // Agregar la transición al nodo actual
                    from_node.edges.borrow_mut().push(edge);
                }
    
                }
            
        
    
        // Debug: Imprimir si se asignó el nodo raíz
        match &self.root {
            Some(r) => println!("Nodo raíz: {}", r.name),
            None => println!("No se asignó un nodo raíz"),
        }// Devolver root directamente, que ahora es Option<Rc<Node>>
    }
    
}
// Cambiamos el tipo de retorno a Option<Rc<Node>> para manejar la posibilidad de que no haya nodo raíz