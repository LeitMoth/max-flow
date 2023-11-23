use std::collections::{HashMap, HashSet};
use std::hash::Hash;

/*
 * This file is mainly boilerplate for getting a FlowNetwork struct to do Ford-Fulkerson on
 *
 * We save the start and end index, an adjacently list (without weights), and a hashmap
 * to associate edges ( in the form of (u,v) tuples ) with capacities
 */

pub struct FlowNetwork {
    pub start: usize,
    pub end: usize,
    pub adj_list: Vec<HashSet<usize>>,
    pub capacities: HashMap<(usize, usize), usize>,
}

#[derive(Copy, Clone)]
pub enum Node<T> {
    Start,
    End,
    Id(T),
}

impl FlowNetwork {
    pub fn size(&self) -> usize {
        self.adj_list.len()
    }

    /*
     * TODO(Colin): add some way to get the nice Node Identifiers back out
     * we could easily do this just by storing a lookup array in the struct
     * but it isn't really necessary if all we care about is the max
     */
    pub fn from_edges_nice<T: Hash + Eq + PartialEq + Copy>(edge_capacities: &[((Node<T>, Node<T>), usize)]) -> Self {
        let mut lookup = HashMap::new();
        let mut next_index = 2;

        let mut tmp = Vec::new();
        let mut get_index = | n: Node<T> | {
            match n {
                Node::Start => 0,
                Node::End => 1,
                Node::Id(id) => {
                    match lookup.get(&id) {
                        Some(&index) => index,
                        None => {
                            let index = next_index;
                            lookup.insert(id, index);
                            next_index += 1;
                            index
                        }
                    }
                }
            }
        };

        for &(edge, capacity) in edge_capacities {
            let (u, v) = edge;

            let u = get_index(u);
            let v = get_index(v);

            tmp.push(((u,v),capacity));
        }

        Self::from_edges(0, 1, &tmp)
    }

    /*
    * Right now I'm trusting the user to not create an invalid flow network
    * (E.g. no 2 vert cycles, no edges going into s, no edges coming out of t)
    *
    * TODO(Colin): Add some basic checks to make sure the input graph is valid
    */
    pub fn from_edges(start: usize, end: usize, edge_capacities: &[((usize, usize), usize)]) -> Self {
        let mut adj_list = vec![HashSet::new(); edge_capacities.len()];
        let mut capacities = HashMap::new();

        for &(edge, capacity) in edge_capacities {
            assert!(edge.0 < edge_capacities.len() && edge.1 < edge_capacities.len(), "Indices of verts must be within list");
            adj_list[edge.0].insert(edge.1);
            capacities.insert(edge, capacity);
        }

        Self {
            adj_list,
            capacities,
            start,
            end,
        }
    }
}

#[cfg(test)]
mod test {
    use std::collections::{HashMap, HashSet};

    use super::FlowNetwork;

    /*
     * https://doc.rust-lang.org/std/collections/struct.HashMap.html
     *
     * When you iterate over the values of a HashMap, they can be in a different
     * order on different runs of the program, or even in subsequent function calls!
     *
     * Apparently there is some interesting security reasons this happens
     * For us though, it just means that routes that tie in BFS won't have a consistent winner,
     * Or in other words, we get a sort of fuzzing for free
     *
     * Was tough to figure out why I was getting tests crashing only half of the time though,
     * good to know
     */
    #[test]
    fn super_packet_example() {
        for _ in 0..1_000 {
            packet_example();
            nice_packet();
        }
    }

    #[test]
    fn packet_example() {
        let adj_list =  vec![
                vec![1,2,3], //0 - s
                vec![2,5,4], //1 - a
                vec![3],     //2 - b
                vec![6],     //3 - c
                vec![5,7],   //4 - d
                vec![6,7],   //5 - e
                vec![2,7],   //6 - f
                vec![],      //7 - t
            ];

        // I started out with a vec of vecs, so I wrote this quick converter

        let adj_list = adj_list.iter().map(|i| {
            let mut h = HashSet::new();
            for &x in i {
                h.insert(x);
            }
            h
        }).collect();

        let mut capacities = HashMap::new();
        capacities.insert((0,1), 10);
        capacities.insert((0,2), 5);
        capacities.insert((0,3), 15);
        capacities.insert((1,2), 4);
        capacities.insert((1,5), 15);
        capacities.insert((1,4), 9);
        capacities.insert((2,3), 4);
        capacities.insert((2,5), 8);
        capacities.insert((3,6), 30);
        capacities.insert((4,5), 15);
        capacities.insert((4,7), 10);
        capacities.insert((5,6), 15);
        capacities.insert((5,7), 10);
        capacities.insert((6,2), 6);
        capacities.insert((6,7), 10);

        let g = FlowNetwork {
            adj_list,
            capacities,
            start: 0,
            end: 7,
        };

        let (_flow, max_flow) = g.ford_fulkerson();

        assert_eq!(max_flow, 28);
    }

    #[test]
    fn nice_packet() {

        use super::Node::*;

        let g = FlowNetwork::from_edges_nice(&[
            ((Start, Id('a')), 10),
            ((Start, Id('b')), 5),
            ((Start, Id('c')), 15),

            ((Id('a'), Id('b')), 4),
            ((Id('a'), Id('d')), 9),
            ((Id('a'), Id('e')), 15),

            ((Id('b'), Id('c')), 4),
            ((Id('b'), Id('e')), 8),

            ((Id('c'), Id('f')), 30),

            ((Id('d'), Id('e')), 15),
            ((Id('d'), End), 10),

            ((Id('e'), End), 10),
            ((Id('e'), Id('f')), 15),

            ((Id('f'), Id('b')), 6),
            ((Id('f'), End), 10),
        ]);

        let (_flow, max_flow) = g.ford_fulkerson();
        assert_eq!(max_flow, 28);
    }
}
