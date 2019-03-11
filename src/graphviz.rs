use std::collections::HashMap;
use std::collections::HashSet;

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

pub fn emit_graph(root: &CreatesGraphviz) {
    let mut node_listing:  HashMap<*const CreatesGraphviz, NodeEntry> = HashMap::new();
    let mut connections: Vec<(String, String)> = Vec::new();
    let mut stack: Vec<&CreatesGraphviz> = vec![root];

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
}