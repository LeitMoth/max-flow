use ford_fulkerson::{FlowNetwork, Node};

fn main() {
    use Node::*;
    let g = FlowNetwork::from_edges_nice(&[
        ((Start, Id('a')), 1),
        ((Id('a'), Id('b')), 1),
        ((Id('b'), Id('c')), 1),
        ((Id('c'), End), 1),


        ((Id('a'), Id('x')), 1),


        ((Start, Id('z')), 1),
        ((Id('z'), Id('y')), 1),
        ((Id('y'), Id('x')), 1),
        ((Id('x'), End), 1),
    ]);

    let (flows, max_flow) = g.ford_fulkerson();

    assert_eq!(max_flow, 2);

    // The indices won't make sense for now, there isn't a way to translate
    // back to the nice letter ids (for now at least)
    println!("{:?}\n{}", flows, max_flow);
}
