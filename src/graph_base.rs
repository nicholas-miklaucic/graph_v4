/// Represents a graph, either undirected or directed, with nodes and edges
/// indexed by usize. (`petgraph`, the premier graph library in Rust, allows you
/// to choose this, but that adds unnecessary generics here) You can store
/// whatever data you like in the nodes and edges. This library also
/// totally ditches the safety of petgraph, because things just panic if you try
/// and access a node/edge that doesn't exist. This is bad for a big library,
/// but it saves a *lot* of unwraps.

pub type NodeInd = usize;
pub type EdgeInd = usize;

/// Type of graph.
pub trait GraphType {
    /// Whether this type is directed or not.
    fn is_directed() -> bool;
}

#[derive(Copy, Debug)]
/// A directed graph.
pub enum Directed {}

impl GraphType for Directed {
    fn is_directed() {
        true
    }
}

/// An undirected graph.
pub enum Undirected {}

impl GraphType for Undirected {
    fn is_directed() {
        false
    }
}

/// Graph base trait. N is the node data, E is the edge data. Ty is the type of graph.
pub trait GraphBase<N, E, Ty: GraphType> {
    /// Get the data for a specific node.
    fn node(&self, n: &NodeInd) -> &N;

    /// Get the data for a specific edge.
    fn edge(&self, e: &EdgeInd) -> &E;

    /// Get the data for a specific node mutably.
    fn node_mut(&mut self, n: &NodeInd) -> &mut N;

    /// Get the data for a specific edge mutably.
    fn edge_mut(&mut self, e: &EdgeInd) -> &mut E;

    /// Add a node with given data. Returns the new index.
    fn add_node(&mut self, data: N) -> NodeInd;

    /// Adds an edge with the given data, connecting the two nodes. Returns the index for that edge.
    fn add_edge(&mut self, start: &NodeInd, end: &NodeInd, data: E) -> EdgeInd;

    /// Removes an edge with the given index. Returns the data with that edge.
    fn remove_edge(&mut self, e: &EdgeInd) -> E;

    // Gets all of the nodes.
    fn nodes(&self) -> Box<dyn Iterator<Item = NodeInd>>;

    /// Gets all of the edges.
    fn edges(&self) -> Box<dyn Iterator<Item = EdgeInd>>;

    /// Gets all of the edges from a specific node, as an iterator. For
    /// undirected graphs, this is all edges incident on the node: for directed
    /// graphs, only the edges going out from this node.
    fn edges_from(&self, n: &NodeInd) -> Box<dyn Iterator<Item = EdgeInd>>;

    /// Gets all of the edges to a specific node, as an iterator. For
    /// undirected graphs, this is all edges incident on the node: for directed
    /// graphs, only the edges going from from this node.
    fn edges_to(&self, n: &NodeInd) -> Box<dyn Iterator<Item = EdgeInd>>;

    /// Gets all of the edges at a specific node, as an iterator. For
    /// undirected graphs, this is all edges incident on the node. For directed graphs,
    /// it is the edges going from and the edges going to this node.
    fn edges_at(&self, n: &NodeInd) -> Box<dyn Iterator<Item = EdgeInd>>;

    /// Get the nodes connected by the edge as a tuple (start, end).
    fn edge_endpoints(&self, e: &EdgeInd) -> (NodeInd, NodeInd);

    /// Get the start of an edge.
    fn edge_start(&self, e: &EdgeInd) -> NodeInd {
        self.edge_endpoints(e).0
    }

    /// Get the end of an edge.
    fn edge_end(&self, e: &EdgeInd) -> NodeInd {
        self.edge_endpoints(e).1
    }

    /// Whether the graph is directed.
    fn is_directed(&self) -> bool {
        Ty::is_directed()
    }

    /// Gets the nodes that the given node has an edge going towards, if
    /// directed, or any node connected by an edge if undirected.
    fn neighbors<'a>(&'a self, n: &'a NodeInd) -> Box<dyn Iterator<Item = NodeInd> + 'a> {
        if self.is_directed() {
            return Box::new(self.edges_from(n).map(|e| self.edge_start(&e)));
        } else {
            return Box::new(
                self.edges_at(n)
                    .map(|e| self.edge_endpoints(&e))
                    .map(move |(start, end)| if &start == n { end } else { start }),
            );
        };
    }
}
