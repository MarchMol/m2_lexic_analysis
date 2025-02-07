use std::fmt::format;
use std::rc::{Rc, Weak};
use std::cell::RefCell;

use serde_json::to_string;

#[derive(Debug)]
struct Node {
    name: String,
    edges: RefCell<Vec<Rc<Edge>>>,
}

#[derive(Debug)]
struct Edge {
    destination: Weak<Node>, // Weak reference to avoid memory cycles
    transition_char: char,
}

pub fn generate(quantity_states: usize, transitions: Vec<(usize, char, usize)>)->Rc<Node>{
    let mut root Rc<Node>; // Inicializae el Rc vació para que se pueda guardar la direcci+ona root
    let mut nodes: Vec<Rc<Node>> = Vec::new();
    for state in 0..quantity_states{
        if state == 0{
            let name = format!("{}{}","s",state);
            let root = Rc::new( Node {
                name,
                edges: RefCell::new(vec![]),
            });
            nodes.push(root);
        } else{
            let name = format!("{}{}","s",state);
            let temp = Rc::new(Node {
                name,
                edges: RefCell::new(vec![]),
            });
            nodes.push(temp);
        }

    }
    for (index,nd) in nodes.iter().enumerate(){
        for tr in &transitions{
            if tr.0==index{
                // create transition
                let destination_rc = Rc::clone(&nodes[tr.2]);
                let edge = Rc::new(Edge {
                    destination: Rc::downgrade(&destination_rc),
                    transition_char: tr.1,
                });

                nd.edges.borrow_mut().push(edge.clone());
            }
        }
    }

    Rc::clone(root)
}