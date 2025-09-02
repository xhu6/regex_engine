type Edge<T> = (T, usize);

#[derive(Debug)]
pub struct Node<T> {
    pub edges: Vec<Edge<T>>,
}

impl<T> Default for Node<T> {
    // Apparently derive default requires T to have default?
    fn default() -> Self {
        Self { edges: Vec::new() }
    }
}

#[derive(Default, Debug)]
pub struct Graph<T> {
    pub nodes: Vec<Node<T>>,
}

impl<T> Graph<T> {
    pub fn new_node(&mut self) -> usize {
        // Create a new node and returns its index
        self.nodes.push(Node::default());
        self.nodes.len() - 1
    }

    pub fn add_edge(&mut self, start: usize, end: usize, value: T) {
        // Add an edge with a value
        self.nodes[start].edges.push((value, end));
    }
}

impl<T> Graph<Option<T>> {
    pub fn add_e(&mut self, start: usize, end: usize) {
        // Add an epsilon edge
        self.nodes[start].edges.push((None, end));
    }
}
