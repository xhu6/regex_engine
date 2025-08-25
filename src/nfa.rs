use std::fmt::Display;

use crate::ast::*;

#[derive(Default, Debug)]
struct Node {
    edges: Vec<(Option<u32>, usize)>,
}

#[derive(Default, Debug)]
struct Graph {
    nodes: Vec<Node>,
}

impl Graph {
    fn new_node(&mut self) -> usize {
        self.nodes.push(Node::default());

        return self.nodes.len() - 1;
    }

    fn add_edge(&mut self, start: usize, end: usize, value: u32) {
        self.nodes[start].edges.push((Some(value), end));
    }

    // Epsilon
    fn add_e(&mut self, start: usize, end: usize) {
        self.nodes[start].edges.push((None, end));
    }
}

impl Display for NFA {
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

#[derive(Debug)]
pub struct NFA {
    graph: Graph,
    start: usize,
    end: usize,
}

fn build(tree: Ast, graph: &mut Graph) -> (usize, usize) {
    use Ast::*;
    use BinOp::*;
    use UnOp::*;

    let start = graph.new_node();

    let end = match tree {
        Sym(x) => {
            let end = graph.new_node();
            graph.add_edge(start, end, x);
            end
        }
        Unary(op, t) => {
            let nfa = build(*t, graph);

            match op {
                Question => {
                    graph.add_e(start, nfa.0);
                    graph.add_e(start, nfa.1);

                    nfa.1
                }
                Plus => {
                    graph.add_e(start, nfa.0);
                    graph.add_e(nfa.1, start);

                    nfa.1
                }
                Star => {
                    graph.add_e(start, nfa.0);
                    graph.add_e(nfa.1, start);

                    nfa.0
                }
            }
        }
        Binary(op, t, u) => {
            let nfa = build(*t, graph);
            let nfa2 = build(*u, graph);

            match op {
                Union => {
                    let end = graph.new_node();

                    graph.add_e(start, nfa.0);
                    graph.add_e(start, nfa2.0);

                    graph.add_e(nfa.1, end);
                    graph.add_e(nfa2.1, end);

                    end
                }
                Concat => {
                    graph.add_e(start, nfa.0);
                    graph.add_e(nfa.1, nfa2.0);

                    nfa2.1
                }
            }
        }
    };

    (start, end)
}

struct Set {
    usizes: Vec<usize>,
    bools: Vec<bool>,
}

impl Set {
    fn new(n: usize) -> Self {
        Self {
            usizes: vec![],
            bools: vec![false; n],
        }
    }

    fn add(&mut self, value: usize) {
        if self.bools[value] {
            return;
        }

        self.bools[value] = true;
        self.usizes.push(value);
    }

    fn contains(&self, value: usize) -> bool {
        return self.bools[value];
    }

    fn len(&self) -> usize {
        return self.bools.len();
    }
}

impl NFA {
    pub fn new(tree: Ast) -> Self {
        let mut graph = Graph::default();
        let (start, end) = build(tree, &mut graph);
        NFA { graph, start, end }
    }

    fn traverse(&self, node: usize, seen: &mut Set) {
        if seen.contains(node) {
            return;
        }

        seen.add(node);

        for &(next_value, next_node) in &self.graph.nodes[node].edges {
            if next_value.is_none() {
                self.traverse(next_node, seen);
            }
        }
    }

    fn update_epsilon(&self, state: Set) -> Set {
        let mut new_state = Set::new(state.len());

        for &node in state.usizes.iter() {
            self.traverse(node, &mut new_state);
        }

        new_state
    }

    fn update_value(&self, state: Set, value: u32) -> Set {
        let mut new_state = Set::new(state.len());

        for &node in state.usizes.iter() {
            for &(next_value, next_node) in &self.graph.nodes[node].edges {
                if Some(value) == next_value {
                    new_state.add(next_node);
                }
            }
        }

        new_state
    }

    pub fn check(&self, inp: &str) -> bool {
        let mut state = Set::new(self.graph.nodes.len());
        state.add(self.start);
        state = self.update_epsilon(state);

        for c in inp.chars() {
            state = self.update_value(state, c as u32);
            state = self.update_epsilon(state);
        }

        state.contains(self.end)
    }
}
