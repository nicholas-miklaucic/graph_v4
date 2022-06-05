//! An adjacency list representation of a graph.

use std::marker::PhantomData;

use crate::graph_base::{EdgeInd, GraphBase, GraphType, NodeInd};

/// Adjacency list representation of a graph. N and E are edge types.
#[derive(Clone, Debug, PartialEq, Hash)]
pub struct ALGraph<N, E, Ty: GraphType> {
    /// The nodes, as a vector of node data.
    nodes: Vec<N>,

    /// The edge data as a vector.
    edges: Vec<E>,

    /// The adjacency lists: a list of edges starting from each node.
    adj: Vec<Vec<EdgeInd>>,

    ty: PhantomData<Ty>,
}

// impl<N, E, Ty: GraphType> GraphBase<N, E, Ty> for ALGraph<N, E, Ty> {}
