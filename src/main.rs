fn main() {
}

#[cfg(test)]
mod tests {
    use petgraph::{Graph, Undirected};
    use petgraph::algo::dijkstra;

    #[test]
    fn create_graph_2() {
        let mut graph: Graph<(), (), Undirected> = Graph::new_undirected();
        let a = graph.add_node(());
        let b = graph.add_node(());
        let c = graph.add_node(());
        let d = graph.add_node(());
        let e = graph.add_node(());
        let f = graph.add_node(());
        let g = graph.add_node(());
        let h = graph.add_node(());
        let i = graph.add_node(());
        let j = graph.add_node(());
        let k = graph.add_node(());
        let l = graph.add_node(());
        let m = graph.add_node(());
        let n = graph.add_node(());
        let o = graph.add_node(());
        let p = graph.add_node(());
        let q = graph.add_node(());
        let r = graph.add_node(());
        let s = graph.add_node(());
        let t = graph.add_node(());
        let u = graph.add_node(());
        let v = graph.add_node(());
        let w = graph.add_node(());
        let x = graph.add_node(());

        graph.extend_with_edges(&[
            (a, b),
            (a, d),
            (a, c),
            (a, l),
            (a, f),
            (b, c),
            (b, d),
            (b, s),
            (b, i),
            (c, d),
            (c, e),
            (c, t),
            (d, f),
            (d, e),
            (e, f),
            (f, r),
            (f, q),
            (f, m),
            (h, g),
            (h, i),
            (i, j),
            (s, t),
            (s, u),
            (s, v),
            (v, w),
            (t, x),
            (l, k),
            (l, n),
            (n, m),
            (n, o),
            (o, p)
        ]);

        let res = dijkstra(&graph, k, Some(a), |_| 1);

        println!("edges={}", graph.edge_count());
        println!("nodes={}", graph.node_count());
        println!("res={:?}", res.get(&a));

    }
}
