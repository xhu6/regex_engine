use std::fmt::Display;

use crate::graph::Graph;
use crate::set::Set as State;
use crate::value::Value;

fn update_value(graph: &Graph<Value>, input: &State, value: char, output: &mut State) {
    // Update state by consuming value.
    for &node in &input.usizes {
        for (next_value, next_node) in &graph.nodes[node].edges {
            if next_value.matches(value) {
                output.insert(*next_node);
            }
        }
    }
}

#[derive(Debug)]
pub struct Nfa {
    graph: Graph<Value>,
    start: usize,
    ends: Vec<usize>,
}

impl Display for Nfa {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.graph)?;
        writeln!(f, "start: {}", self.start)?;
        write!(f, "end: {:?}", self.ends)
    }
}

impl Nfa {
    pub fn new(graph: Graph<Value>, start: usize, ends: Vec<usize>) -> Self {
        Nfa { graph, start, ends }
    }

    fn create_state(&self) -> State {
        State::new(self.graph.len())
    }

    pub fn check(&self, inp: &str) -> bool {
        let mut _state = self.create_state();
        let mut _state2 = self.create_state();

        let mut state = &mut _state;
        let mut state2 = &mut _state2;

        state.insert(self.start);

        for c in inp.chars() {
            if state.usizes.is_empty() {
                return false;
            }

            update_value(&self.graph, state, c, state2);
            state.clear();
            (state, state2) = (state2, state);
        }

        self.ends.iter().any(|x| state.contains(*x))
    }

    pub fn has_match(&self, inp: &str) -> bool {
        let mut _state = self.create_state();
        let mut _state2 = self.create_state();

        let mut state = &mut _state;
        let mut state2 = &mut _state2;

        state.insert(self.start);

        for c in inp.chars() {
            if self.ends.iter().any(|x| state.contains(*x)) {
                return true;
            }

            update_value(&self.graph, state, c, state2);
            state.clear();
            (state, state2) = (state2, state);

            state.insert(self.start);
        }

        self.ends.iter().any(|x| state.contains(*x))
    }
}
