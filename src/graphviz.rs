use std::collections::HashMap;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;

// Trait system needs to allow for graph construction.
// Main data structure holds basic data, like node
// looks.

// start with the root node. Add the node to the stack.

// Get the node at the top of the stack. For each of its children, 

// Pop the node at the top of the stack. First check if
// the refernce has a name. If not, name it, and bind the
// the reference to the name. Then, look at the references.

pub trait CreatesGraphviz {
    fn get_name(&self) -> String;
    fn get_connections(&self) -> Vec<&CreatesGraphviz>;
}

struct NodeEntry <'a> {
    name: String,
    children: Vec<&'a CreatesGraphviz>
}

pub struct Graphviz {
    labels: HashMap<String, String>,
    connections: Vec<(String, String)>,
}

impl Graphviz {
    pub fn write_file(&self, filename: String) {
        let output_file = File::create(&filename).expect(&format!("Could not open file {}", &filename));
        let mut writer = BufWriter::new(output_file);

        writeln!(&mut writer, "digraph output {{").expect(&format!("Could not write to file {}", &filename));

        for (name, label) in &self.labels {
            writeln!(&mut writer, "    {} [label=\"{}\"];", name, label).expect(&format!("Could not write to file {}", &filename));
        }

        for (from, to) in &self.connections {
            writeln!(&mut writer, "    {} -> {};", from, to).expect(&format!("Could not write to file {}", &filename));
        }

        writeln!(&mut writer, "}}").expect(&format!("Could not write to file {}", &filename));
        writer.flush().expect(&format!("Error while finalizing {}", &filename));
    } 
}

impl From<&CreatesGraphviz> for Graphviz {
    fn from(item: &CreatesGraphviz) -> Self {
        let mut node_listing:  HashMap<*const CreatesGraphviz, NodeEntry> = HashMap::new();
        let mut labeling: HashMap<String, String> = HashMap::new();
        let mut connections: Vec<(String, String)> = Vec::new();
        let mut stack: Vec<&CreatesGraphviz> = vec![item];

        while stack.len() > 0 {
            let top_node: &CreatesGraphviz = stack.pop().unwrap();
            if let Some(entry) = node_listing.get(&(top_node as *const CreatesGraphviz)) {
                for child in &entry.children {
                    let connected_name = &node_listing.get(&(*child as *const CreatesGraphviz)).unwrap().name;
                    connections.push((entry.name.clone(), connected_name.clone()));
                }
            }
            else {
                let name = format!("_{}", node_listing.len());
                labeling.insert(name.clone(), top_node.get_name());
                let children = top_node.get_connections();
                let mut unvisited: Vec<&CreatesGraphviz> = Vec::new();
                for child in children {
                    if let Some(connected_node) = node_listing.get(&(top_node as *const CreatesGraphviz)) {
                        connections.push((name.clone(), connected_node.name.clone()));
                    }
                    else {
                        unvisited.push(child);
                    }
                }

                if unvisited.len() > 0 {
                    stack.push(top_node);
                    for unvisited_node in &unvisited {
                        stack.push(*unvisited_node)
                    }
                }

                let node = NodeEntry {
                    name: name,
                    children: unvisited
                };
                node_listing.insert(top_node as *const CreatesGraphviz, node);
            }
        }

        let result = Graphviz {
            labels: labeling,
            connections: connections
        };
        return result;
    }
}