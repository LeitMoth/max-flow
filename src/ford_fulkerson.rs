use std::collections::HashMap;

use crate::{FlowNetwork, residual_network::{ResidualNetwork, Direction}, bfs};

/*
 * Actual implementation of Ford-Fulkerson (specifically Edmonds-Karp)
 *
 * Tests for this are in flow_network.rs to keep this file pretty
 */

type Flow = HashMap<(usize, usize), usize>;

impl FlowNetwork {

    pub fn ford_fulkerson(&self) -> (Flow, usize) {
        let mut g_f = ResidualNetwork::new(self.size());
        let mut flow: Flow = HashMap::new();

        for &edge in self.capacities.keys() {
            flow.insert(edge, 0);
        }

        fn update_g_f(g_f: &mut ResidualNetwork, g: &FlowNetwork, flow: &Flow) {
            for (&edge, &capacity) in &g.capacities {
                let flow = flow[&edge];
                g_f.update_forward(edge, capacity.saturating_sub(flow));
                g_f.update_backward(edge, flow);
            }
        }

        update_g_f(&mut g_f, &self, &flow);

        while let Some(path) = bfs::find_path(&g_f.adj_list, self.start, self.end) {
            let min = path
                .windows(2)
                .map(|l| (l[0],l[1]))
                .map(|e| g_f.edges[&e].0)
                .min()
                .unwrap();


            for edge in path.windows(2).map(|l| (l[0],l[1])) {
                let res_edge = &g_f.edges[&edge];

                use Direction as D;
                match res_edge.1 {
                    D::Forwards  => {
                        let cur_flow = flow[&edge];
                        flow.insert(edge, cur_flow + min)
                    },
                    D::Backwards => {
                        // flow is indexed with regard to the forwards edges
                        // if we move along a backwards edge we have to look up the flow for the
                        // corresponding forwards edge and update that

                        let cur_flow = flow[&(edge.1, edge.0)];
                        flow.insert((edge.1, edge.0), cur_flow - min)
                    },
                };
            }

            update_g_f(&mut g_f, &self, &flow);
        }

        // Sum the flows of all edges coming out of self.start
        let max_flow = self.adj_list[self.start].iter().map(|&v| flow[&(self.start,v)]).sum();

        (flow, max_flow)
    }

}
