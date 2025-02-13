use petgraph::dot::{Dot, Config};
use petgraph::Graph;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::os::linux::raw::stat;
use std::process::Command;
use std::path::Path;

pub fn get_all_states(ginfo: &HashMap<char, HashMap<char, char>>)->Vec<String>{
    let states = ginfo.keys();
    let mut all_states: Vec<String> = Vec::new();
    for from in states{
        if !all_states.contains(&from.to_string()){
            all_states.push(from.to_string());
        }
        match ginfo.get(from){
            Some(ts)=>{
                let dest = ts.values();
                for to in dest{
                    if !all_states.contains(&to.to_string()){
                        all_states.push(to.to_string());
                    }
                }
            },
            _=>{}
        }
    }
    all_states
}

pub fn generate_graph(ginfo: &HashMap<char, HashMap<char, char>>, states: &Vec<String>)->Graph<String, String>{
    let mut graph = Graph::<String, String>::new();
    // Creating all nodes
    println!("{:?}", ginfo);
    for st in states{
        graph.add_node(st.to_string());
    }
    // Creating edges
    for n_index in graph.node_indices(){
        let from_weight = &graph[n_index].chars().next().unwrap();
        match ginfo.get(from_weight){
            Some(hash) =>{
                let trans = hash.keys();
                for tr in trans{
                    match hash.get(tr){
                        Some(dest_weight)=>{
                            let to_node  = graph
                            .node_indices()
                            .find(|&i| graph[i] == dest_weight.to_string())
                            .expect("Node not found");
                            graph.add_edge(n_index, to_node, tr.to_string());
                        },
                        _=>{

                        }
                    }
                    
                }
            }
            _=>{

            }
        }
        println!("nodes: {:?}",from_weight);
    }
    graph
}

pub fn render(ginfo: &HashMap<char, HashMap<char, char>>, dest: &str) {
    let all_states = get_all_states(&ginfo);
    let graph =generate_graph(ginfo, &all_states);

    let dot_output = format!("{}", Dot::with_config(&graph, &[]));
    let dot_file = "graph.dot";
    let png_file = &format!("{}.png", dest);
    let mut file = File::create(dot_file).expect("Failed to create .dot file");
    file.write_all(dot_output.as_bytes())
        .expect("Failed to write to .dot file");

    let output = Command::new("dot")
        .arg("-Tpng") // Output format: PNG
        .arg(dot_file) // Input file
        .arg("-o") // Output file flag
        .arg(png_file) // Output file name
        .output();

    match output {
        Ok(output) if output.status.success() => {
            println!("Graph saved as '{}'", png_file);
        }
        Ok(output) => {
            eprintln!(
                "dot command failed: {}",
                String::from_utf8_lossy(&output.stderr)
            );
        }
        Err(err) => {
            eprintln!("Failed to execute dot: {}", err);
        }
    }
}
