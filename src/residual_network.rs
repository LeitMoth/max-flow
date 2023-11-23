use std::collections::{HashSet, HashMap};

/*
 * We store an adjacency list (which we change from time to time when we
 * completely fill up an edges capacity), and a edges hashmap
 *
 * The map goes from (u,v) to (value, Direction).
 * We match on this Direction in the main loop of the algorithm
 */

pub enum Direction {
    Forwards, Backwards
}

pub struct ResidualNetwork {
    pub adj_list: Vec<HashSet<usize>>,
    pub edges: HashMap<(usize, usize), (usize, Direction)>,
}

impl ResidualNetwork {
    pub fn new(size: usize) -> Self {
        ResidualNetwork {
            adj_list: vec![HashSet::new(); size],
            edges: HashMap::new(),
        }
    }

    pub fn update_forward(&mut self, edge: (usize, usize), weight: usize) {
        if weight == 0 {
            self.adj_list[edge.0].remove(&edge.1);
            self.edges.remove(&edge);
        } else {
            self.adj_list[edge.0].insert(edge.1);
            self.edges.insert(edge, (weight, Direction::Forwards));
        }
    }

    pub fn update_backward(&mut self, edge: (usize, usize), weight: usize) {
        if weight == 0 {
            self.adj_list[edge.1].remove(&edge.0);
            self.edges.remove(&(edge.1, edge.0));
        } else {
            self.adj_list[edge.1].insert(edge.0);
            self.edges.insert((edge.1, edge.0), (weight, Direction::Backwards));
        }

    }
}

// impl std::fmt::Debug for ResidualNetwork {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         for ((u,v), (w, d)) in self.edges.iter() {
//             write!(f, "({},{})=>{}{} ", u, v, w, match d { Direction::Forwards => 'F', _ => 'B' })?
//         }
//         Ok(())
//     }
// }
