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

struct TrackedState {
    state: State,
    indexes: Vec<usize>,
}

impl TrackedState {
    pub fn insert(&mut self, node: usize, index: usize) {
        self.state.insert(node);
        self.indexes[node] = self.indexes[node].min(index);
    }

    pub fn clear(&mut self) {
        self.state.clear();
        self.indexes.fill(usize::MAX);
    }
}

fn update_value_tracked(
    graph: &Graph<Value>,
    input: &TrackedState,
    value: char,
    output: &mut TrackedState,
) {
    // Update state by consuming value.
    for &node in &input.state.usizes {
        for (next_value, next_node) in &graph.nodes[node].edges {
            if next_value.matches(value) {
                output.insert(*next_node, input.indexes[node]);
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

    fn create_tracked_state(&self) -> TrackedState {
        TrackedState {
            state: self.create_state(),
            indexes: vec![usize::MAX; self.graph.len()],
        }
    }

    pub fn check(&self, inp: &str) -> bool {
        // Checks for full match.
        let mut state = &mut self.create_state();
        let mut state2 = &mut self.create_state();

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
        // Checks for partial match.
        let mut state = &mut self.create_state();
        let mut state2 = &mut self.create_state();

        state.insert(self.start);

        if self.ends.iter().any(|x| state.contains(*x)) {
            return true;
        }

        for c in inp.chars() {
            update_value(&self.graph, state, c, state2);
            state.clear();
            (state, state2) = (state2, state);

            state.insert(self.start);

            if self.ends.iter().any(|x| state.contains(*x)) {
                return true;
            }
        }

        false
    }

    pub fn search(&self, inp: &str) -> Option<(usize, usize)> {
        // Finds earliest longest match.
        let mut state = &mut self.create_tracked_state();
        let mut state2 = &mut self.create_tracked_state();

        state.insert(self.start, 0);

        let found = |state: &TrackedState| {
            self.ends
                .iter()
                .copied()
                .filter(|x| state.state.contains(*x))
                .map(|x| state.indexes[x])
                .min()
        };

        let mut best = if found(state).is_some() {
            Some((0, 0))
        } else {
            None
        };

        for (index, c) in inp.chars().enumerate() {
            update_value_tracked(&self.graph, state, c, state2);
            state.clear();
            (state, state2) = (state2, state);

            if best.is_none() {
                state.insert(self.start, index + 1);
            }

            if let Some(start) = found(state) {
                best = match best {
                    Some((x, _)) if x < start => best,
                    _ => Some((start, index + 1)),
                }
            }
        }

        best
    }
}
