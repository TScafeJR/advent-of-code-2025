use std::collections::HashMap;
use std::fmt::Debug;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Graph<T> {
    pub nodes: HashMap<T, Vec<T>>,
    pub heads_of_graph: Vec<T>,
    pub tails_of_graph: Vec<T>,
}

#[allow(dead_code)]
impl<T: Eq + std::hash::Hash + Clone + Debug> Graph<T> {
    pub fn new() -> Self {
        Graph {
            nodes: HashMap::new(),
            heads_of_graph: Vec::new(),
            tails_of_graph: Vec::new(),
        }
    }

    pub fn add_node(&mut self, value: T) {
        self.nodes.entry(value).or_insert(Vec::new());
    }

    pub fn add_head(&mut self, value: T) {
        self.heads_of_graph.push(value);
    }

    pub fn get_heads(&self) -> &Vec<T> {
        &self.heads_of_graph
    }

    pub fn get_tails(&self) -> &Vec<T> {
        &self.tails_of_graph
    }

    pub fn add_tail(&mut self, value: T) {
        self.tails_of_graph.push(value);
    }

    pub fn add_edge(&mut self, from: T, to: T) {
        self.nodes
            .entry(from.clone())
            .or_insert(Vec::new())
            .push(to.clone());

        self.nodes.entry(to).or_insert(Vec::new());
    }

    #[cfg(test)]
    pub fn add_bidirectional_edge(&mut self, from: T, to: T) {
        self.add_edge(from.clone(), to.clone());
        self.add_edge(to, from);
    }

    pub fn outgoing_neighbors(&self, node: &T) -> Option<&Vec<T>> {
        self.nodes.get(node)
    }

    pub fn has_node(&self, node: T) -> bool {
        self.nodes.contains_key(&node)
    }

    pub fn find_node(&self, node: T) -> Option<T> {
        match self.nodes.get(&node) {
            Some(_) => Some(node),
            None => None,
        }
    }

    pub fn dfs_with_condition(
        &self,
        start: &T,
        end: &T,
        condition: fn(s: &T, e: &T) -> bool,
        visited: &mut HashMap<T, bool>,
    ) -> bool {
        if visited.contains_key(start) {
            return false;
        }

        visited.insert(start.clone(), true);

        if start == end {
            return true;
        }

        if let Some(neighbors) = self.outgoing_neighbors(start) {
            for neighbor in neighbors {
                if !condition(start, neighbor) {
                    continue;
                }

                if self.dfs_with_condition(neighbor, end, condition, visited) {
                    return true;
                }
            }
        }

        false
    }

    pub fn count_distinct_paths_with_condition(
        &self,
        start: &T,
        end: &T,
        condition: fn(s: &T, e: &T) -> bool,
    ) -> u64 {
        if start == end {
            return 1;
        }

        let mut total = 0;
        if let Some(neighbors) = self.outgoing_neighbors(start) {
            for neighbor in neighbors {
                if !condition(start, neighbor) {
                    continue;
                }

                total += self.count_distinct_paths_with_condition(neighbor, end, condition);
            }
        }

        total
    }

    pub fn concat(&mut self, other: &Graph<T>) {
        let new_nodes = other.nodes.clone();
        let new_heads = other.heads_of_graph.clone();
        let new_tails = other.tails_of_graph.clone();

        self.nodes.extend(new_nodes);
        self.heads_of_graph.extend(new_heads);
        self.tails_of_graph.extend(new_tails);
    }

    pub fn size(&self) -> usize {
        self.nodes.len()
    }
}
