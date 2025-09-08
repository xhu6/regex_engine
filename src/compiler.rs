use crate::ast::{Ast, BinOp, UnOp};
use crate::graph::Graph;
use crate::nfa::Nfa;
use crate::set::Set as State;

fn compactify_forward<T>(graph: &mut Graph<Option<T>>, end: usize) {
    // Skip destination along epsilon chains.
    //
    // Avoids managing more states and reduce epsilon depth.
    let mut cache: Vec<Option<usize>> = vec![None; graph.len()];

    // Don't skip past end state
    cache[end] = Some(end);

    fn only_epsilon<T>(e: &Vec<(Option<T>, usize)>) -> bool {
        e.len() == 1 && e[0].0.is_none()
    }

    fn skip<T>(cache: &mut Vec<Option<usize>>, graph: &Graph<Option<T>>, node: usize) -> usize {
        if let Some(x) = cache[node] {
            x
        } else {
            let e = &graph.nodes[node].edges;

            let res = if only_epsilon(e) {
                skip(cache, graph, e[0].1)
            } else {
                node
            };

            cache[node] = Some(res);

            res
        }
    }

    // Can't use map because graph needs to be accessible for `skip`
    // Maybe there is a way to simplify this?
    for node in 0..graph.len() {
        for i in 0..graph.nodes[node].edges.len() {
            graph.nodes[node].edges[i].1 = skip(&mut cache, graph, graph.nodes[node].edges[i].1);
        }
    }
}

fn compactify_backward<T: Clone>(graph: &mut Graph<Option<T>>, end: usize) {
    // Copies non-epsilon transitions in epsilon chains.
    //
    // NOTE: It may not be beneficial to remove epsilons entirely
    // due to edges becoming more duplicated across states.
    // However, this is more performant usually (more benchmarks needed)
    let mut state = State::new(graph.len());

    for node in 0..graph.len() {
        graph.traverse(node, &mut state);

        let edges: Vec<(Option<T>, usize)> = state
            .usizes
            .iter()
            .flat_map(|x| graph.nodes[*x].edges.iter())
            .filter(|x| x.0.is_some() || x.1 == end)
            .map(|x| x.clone())
            .collect();

        graph.nodes[node].edges = edges;
        state.clear();
    }
}

fn compactify<T: Clone>(graph: &mut Graph<Option<T>>, end: usize) {
    // Skips epsilons.
    compactify_forward(graph, end);
    compactify_backward(graph, end);
}

fn remove_duplicate_edges<T: Ord>(graph: &mut Graph<T>) {
    for node in &mut graph.nodes {
        node.edges.sort_unstable();
        node.edges.dedup();
    }
}

fn remove_nodes<T: Clone>(
    graph: &Graph<T>,
    start: usize,
    end: usize,
) -> (Graph<T>, Option<usize>, Option<usize>) {
    // Reachable from start
    let mut reachable = vec![false; graph.len()];

    fn from_start<T>(graph: &Graph<T>, node: usize, reachable: &mut Vec<bool>) {
        if reachable[node] {
            return;
        }

        reachable[node] = true;

        for e in &graph.nodes[node].edges {
            from_start(graph, e.1, reachable);
        }
    }

    from_start(graph, start, &mut reachable);

    // Finishable to end
    let mut finishable = vec![false; graph.len()];

    // Reverse transition directions, then do dfs
    let mut adj_graph: Vec<Vec<usize>> = vec![vec![]; graph.len()];

    for node in 0..graph.len() {
        for edge in &graph.nodes[node].edges {
            adj_graph[edge.1].push(node);
        }
    }

    fn from_end(adj_graph: &Vec<Vec<usize>>, node: usize, relevant: &mut Vec<bool>) {
        if relevant[node] {
            return;
        }

        relevant[node] = true;

        for e in &adj_graph[node] {
            from_end(adj_graph, *e, relevant);
        }
    }

    from_end(&adj_graph, end, &mut finishable);

    // Filter relevant
    let mut map = vec![None; graph.len()];
    let mut new_graph = Graph::default();

    for node in 0..graph.len() {
        if finishable[node] & reachable[node] {
            map[node] = Some(new_graph.new_node());
        }
    }

    for node in 0..graph.len() {
        if let Some(x) = map[node] {
            for edge in &graph.nodes[node].edges {
                if let Some(y) = map[edge.1] {
                    new_graph.add_edge(x, y, edge.0.clone());
                }
            }
        }
    }

    (new_graph, map[start], map[end])
}

fn build(tree: &Ast, graph: &mut Graph<Option<char>>) -> (usize, usize) {
    use Ast::*;
    use BinOp::*;
    use UnOp::*;

    match tree {
        Sym(x) => {
            let start = graph.new_node();
            let end = graph.new_node();

            graph.add_edge(start, end, Some(*x));

            (start, end)
        }

        Unary(op, t) => match op {
            Range(lower, Some(upper)) => {
                let start = graph.new_node();
                let end = graph.new_node();
                let mut cur = start;

                // Build chain of NFA `lower` times
                for _ in 0..*lower {
                    let nfa = build(t, graph);
                    graph.add_e(cur, nfa.0);
                    cur = nfa.1;
                }

                graph.add_e(cur, end);

                // Doesn't do anything extra if upper < lower
                // Should be validated in parser anyway
                for _ in *lower..*upper {
                    let nfa = build(t, graph);
                    graph.add_e(cur, nfa.0);
                    cur = nfa.1;

                    // Optimised to jump to end if fail
                    graph.add_e(cur, end);
                }

                (start, end)
            }

            Range(lower, None) => {
                let nfa = build(t, graph);

                let start = graph.new_node();
                graph.add_e(start, nfa.0);

                // Can get away with reusing start node for {0,}
                let end = if *lower == 0 {
                    start
                } else {
                    let end = graph.new_node();
                    graph.add_e(end, start);

                    end
                };

                graph.add_e(nfa.1, end);

                (start, end)
            }
        },

        Binary(op, t, u) => {
            let nfa = build(t, graph);
            let nfa2 = build(u, graph);

            match op {
                Union => {
                    let start = graph.new_node();
                    let end = graph.new_node();

                    graph.add_e(start, nfa.0);
                    graph.add_e(start, nfa2.0);

                    graph.add_e(nfa.1, end);
                    graph.add_e(nfa2.1, end);

                    (start, end)
                }

                Concat => {
                    graph.add_e(nfa.1, nfa2.0);

                    (nfa.0, nfa2.1)
                }
            }
        }
    }
}

pub fn compile(tree: &Ast) -> Nfa {
    let mut graph = Graph::default();
    let (start, end) = build(tree, &mut graph);
    compactify(&mut graph, end);
    remove_duplicate_edges(&mut graph);

    // Reduces memory but not much performance
    // let (new_graph, s, e) = remove_nodes(&graph, start, end);
    // Nfa::new(new_graph, s.unwrap(), e.unwrap())

    Nfa::new(graph, start, end)
}
