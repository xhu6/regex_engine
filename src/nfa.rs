use std::fmt::Display;

use crate::graph::Graph;
use crate::set::Set as State;

fn update_epsilon<T>(graph: &Graph<Option<T>>, input: &State, output: &mut State) {
    // Update state to account for epsilon transitions.
    for &node in &input.usizes {
        graph.traverse(node, output);
    }
}

fn update_value<T: Eq>(graph: &Graph<T>, input: &State, value: T, output: &mut State) {
    // Update state by consuming value.
    for &node in &input.usizes {
        for (next_value, next_node) in &graph.nodes[node].edges {
            if &value == next_value {
                output.insert(*next_node);
            }
        }
    }
}

fn update_char(graph: &Graph<Option<char>>, input: &State, value: char, output: &mut State) {
    // Update state by consuming value.
    update_value(graph, input, Some(value), output);
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
    pub fn new(graph: Graph<Option<char>>, start: usize, end: usize) -> Self {
        Nfa { graph, start, end }
    }

    fn create_state(&self) -> State {
        State::new(self.graph.len())
    }

    pub fn check(&self, inp: &str) -> bool {
        let mut state = self.create_state();
        let mut tmp_state = self.create_state();

        state.insert(self.start);

        update_epsilon(&self.graph, &state, &mut tmp_state);
        state.clear();

        // For some reason, `swap` is very slow so do it manually
        for c in inp.chars() {
            update_char(&self.graph, &tmp_state, c, &mut state);
            tmp_state.clear();

            update_epsilon(&self.graph, &state, &mut tmp_state);
            state.clear();
        }

        tmp_state.contains(self.end)
    }
}
