use std::fmt::Display;

use crate::ast::*;
use crate::graph::Graph;

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

                // Doesn't do anything if upper < lower
                // Optimised to jump to end if fail
                for _ in *lower..*upper {
                    let nfa = build(t, graph);
                    graph.add_e(cur, nfa.0);
                    cur = nfa.1;
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

struct Set {
    // Set with efficient iter
    usizes: Vec<usize>,
    bools: Vec<bool>,
}

impl Set {
    fn new(n: usize) -> Self {
        Self {
            usizes: Vec::with_capacity(10),
            bools: vec![false; n],
        }
    }

    fn insert(&mut self, value: usize) {
        if self.bools[value] {
            return;
        }

        self.bools[value] = true;
        self.usizes.push(value);
    }

    fn contains(&self, value: usize) -> bool {
        self.bools[value]
    }

    fn clear(&mut self) {
        self.bools.fill(false);
        self.usizes.clear();
    }
}

type State = Set;

fn traverse<T>(graph: &Graph<Option<T>>, node: usize, seen: &mut State) {
    // Performs DFS on reachable nodes via epsilon
    if seen.contains(node) {
        return;
    }

    seen.insert(node);

    // Fine to iterate as graph is sparse
    for edge in &graph.nodes[node].edges {
        if edge.0.is_none() {
            traverse(graph, edge.1, seen);
        }
    }
}

#[derive(Debug)]
pub struct Nfa {
    graph: Graph<Option<char>>,
    start: usize,
    end: usize,
}

impl Display for Nfa {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (idx, node) in self.graph.nodes.iter().enumerate() {
            write!(f, "{idx}: ")?;

            for (value, next_node) in &node.edges {
                if let Some(x) = value {
                    write!(f, "{x}_")?;
                }
                write!(f, "{next_node} ")?;
            }

            writeln!(f)?;
        }

        writeln!(f, "start: {}", self.start)?;
        write!(f, "end: {}", self.end)
    }
}

impl Nfa {
    pub fn new(tree: &Ast) -> Self {
        let mut graph = Graph::default();
        let (start, end) = build(tree, &mut graph);

        Self { graph, start, end }
    }

    fn create_state(&self) -> State {
        State::new(self.graph.len())
    }

    fn update_epsilon(&self, state: &State, tmp_state: &mut State) {
        // Update state to account for epsilon transitions
        for &node in &state.usizes {
            traverse(&self.graph, node, tmp_state);
        }
    }

    fn update_value(&self, state: &State, value: char, tmp_state: &mut State) {
        // Update state by consuming char
        for &node in &state.usizes {
            for &(next_value, next_node) in &self.graph.nodes[node].edges {
                if Some(value) == next_value {
                    tmp_state.insert(next_node);
                }
            }
        }
    }

    pub fn check(&self, inp: &str) -> bool {
        let mut state = self.create_state();
        let mut tmp_state = self.create_state();

        state.insert(self.start);

        self.update_epsilon(&state, &mut tmp_state);
        state.clear();

        // For some reason, `swap` is very slow so do it manually
        for c in inp.chars() {
            self.update_value(&tmp_state, c, &mut state);
            tmp_state.clear();

            self.update_epsilon(&state, &mut tmp_state);
            state.clear();
        }

        tmp_state.contains(self.end)
    }
}
