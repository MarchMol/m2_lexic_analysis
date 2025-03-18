use petgraph::dot::Dot;
use petgraph::Graph;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Write;
use std::process::Command;

pub fn get_all_states(ginfo: &HashMap<char, HashMap<String, char>>)->Vec<String>{
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

pub fn generate_graph(ginfo: &HashMap<char, HashMap<String, char>>, states: &Vec<String>)->Graph<String, String>{
    let mut graph = Graph::<String, String>::new();
    // Creating all nodes
    // println!("{:?}", ginfo);
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
        // println!("nodes: {:?}",from_weight);
    }
    graph
}

pub fn render(ginfo: &HashMap<char, HashMap<String, char>>, accept: &HashSet<char>, dest: &str) {
    let all_states = get_all_states(&ginfo);
    let graph =generate_graph(ginfo, &all_states);
    let start_n  = graph
    .node_indices()
    .find(|&i| graph[i] == "A")
    .expect("Node not found");
    let start = format!("\"\" [style=invisible, width=0, height=0];\n\"\" -> {:?};\n", start_n.index());

    let mut dot_output = format!("{}", Dot::with_config(&graph, &[]));
    dot_output.insert_str(dot_output.len() - 2, &start);

    for node in accept {
        let tem_node = graph
        .node_indices()
        .find(|&i| graph[i] == node.to_string())
        .expect("Node not found");
        let node_label = format!("{} [", tem_node.index());
        let peripheries_attr = format!("{} [peripheries=2, ", tem_node.index());
        if let Some(pos) = dot_output.find(&node_label) {
            dot_output.replace_range(pos..pos + node_label.len(), &peripheries_attr);
        }
    }

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
            eprintln!("Can't render. Failed to execute dot: {}", err);
        }
    }
}
