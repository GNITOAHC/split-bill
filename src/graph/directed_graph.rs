use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

#[derive(Debug)]
pub struct DirectedGraph<T: Eq + Hash + Debug, W: Debug> {
    adjacency_list: HashMap<T, Vec<(T, W)>>, // (destination_vertex, weight)
}

impl<T: Eq + Hash + Debug, W: Debug> DirectedGraph<T, W> {
    // Creates a new directed graph
    pub fn new() -> Self {
        Self {
            adjacency_list: HashMap::new(),
        }
    }

    // Adds a vertex to the graph
    pub fn add_vertex(&mut self, vertex: T) {
        self.adjacency_list.entry(vertex).or_insert(Vec::new());
    }

    // Adds an edge to the graph
    pub fn add_edge(&mut self, source: T, destination: T, weight: W) {
        self.adjacency_list
            .entry(source)
            .or_insert(Vec::new())
            .push((destination, weight));
    }

    // Displays the graph
    pub fn display(&self) {
        for (vertex, neighbors) in &self.adjacency_list {
            println!("{:?} -> {:?}", vertex, neighbors);
        }
    }
}

use serde::ser::{Serialize, SerializeMap, Serializer};

// Implementing Serialize for DirectedGraph
impl<T: Eq + Hash + Debug + Serialize + Clone, W: Debug + Serialize + Clone> Serialize
    for DirectedGraph<T, W>
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(self.adjacency_list.len()))?;
        for (vertex, edges) in &self.adjacency_list {
            let serialized_edges: Vec<(T, W)> = edges
                .iter()
                .map(|(dest, weight)| (dest.clone(), weight.clone()))
                .collect();
            map.serialize_entry(vertex, &serialized_edges)?;
        }
        map.end()
    }
}
