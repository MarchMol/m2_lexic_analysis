use std::cell::RefCell;
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

// Cambiamos el tipo de retorno a Option<Rc<Node>> para manejar la posibilidad de que no haya nodo raíz
pub fn generate(
    quantity_states: usize,
    transitions: Vec<(usize, char, usize)>,
) -> Option<Rc<Node>> {
    let mut root: Option<Rc<Node>> = None; // Inicializa como Option<Rc<Node>> para evitar problemas con la referencia vacía
    let mut nodes: Vec<Rc<Node>> = Vec::new();

    // Crear los nodos
    for state in 0..quantity_states {
        let name = format!("{}{}", "s", state);
        let temp = Rc::new(Node {
            name: name.clone(),
            edges: RefCell::new(vec![]),
        });
        nodes.push(temp);

        // Debug: Imprimir cuando se crea un nodo
        println!("Nodo creado: {}", name);

        // Si es el primer nodo, lo asignamos como root
        if state == 0 {
            root = Some(Rc::clone(&nodes[state]));
        }
    }

    // Crear las transiciones entre los nodos
    // Crear las transiciones entre los nodos
    for (index, nd) in nodes.iter().enumerate() {
        for tr in &transitions {
            if tr.0 == index {
                // Crear la transición con referencia débil al nodo destino
                let edge = Rc::new(Edge {
                    destination: Rc::downgrade(&nodes[tr.2]), // Creamos una referencia débil directamente
                    transition_char: tr.1,
                });

                // Debug: Imprimir la transición que se crea
                println!(
                    "Creando transición: {} -> {} con el carácter '{}'",
                    nd.name,
                    nodes[tr.2].name,
                    tr.1 // Cambié destination_rc.name a nodes[tr.2].name
                );

                // Agregar la transición al nodo actual
                nd.edges.borrow_mut().push(edge);
            }
        }
    }

    // Debug: Imprimir si se asignó el nodo raíz
    match &root {
        Some(r) => println!("Nodo raíz: {}", r.name),
        None => println!("No se asignó un nodo raíz"),
    }

    root // Devolver root directamente, que ahora es Option<Rc<Node>>
}
