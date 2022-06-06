//! An adjacency list representation of a graph.

use std::{collections::HashMap, marker::PhantomData};

use crate::graph_base::{Edge, EdgeInd, GraphBase, GraphType, NodeInd};

/// Adjacency list representation of a graph. N and E are edge types.
#[derive(Clone, Debug, PartialEq)]
pub struct ALGraph<N, E, Ty: GraphType> {
    /// The node data.
    nodes: HashMap<NodeInd, N>,

    /// The edge data.
    edges: HashMap<EdgeInd, Edge<E>>,

    /// The adjacency lists: a list of edges starting from each node. Edges keep
    /// track of start and end nodes as well as the index.
    adj: Vec<Vec<EdgeInd>>,

    /// The current node index.
    curr_node: NodeInd,

    /// The current edge index.
    curr_edge: EdgeInd,

    ty: PhantomData<Ty>,
}

impl<N, E: Clone, Ty: GraphType> GraphBase<N, E, Ty> for ALGraph<N, E, Ty> {
    fn node(&self, n: &NodeInd) -> &N {
        self.nodes.get(n).unwrap()
    }

    fn edge(&self, e: &EdgeInd) -> &Edge<E> {
        self.edges.get(e).unwrap()
    }

    fn node_mut(&mut self, n: &NodeInd) -> &mut N {
        self.nodes.get_mut(n).unwrap()
    }

    fn edge_mut(&mut self, e: &EdgeInd) -> &mut Edge<E> {
        self.edges.get_mut(e).unwrap()
    }

    fn add_node(&mut self, data: N) -> NodeInd {
        self.nodes.insert(self.curr_node, data);
        self.curr_node += 1;
        self.curr_node
    }

    fn add_edge(&mut self, start: &NodeInd, end: &NodeInd, data: E) -> EdgeInd {
        let edge = Edge {
            start: *start,
            end: *end,
            index: self.curr_edge,
            data,
        };

        // first, add edge information
        self.edges.insert(self.curr_edge, edge.clone());

        self.adj[*start].push(edge.index);
        if !self.is_directed() {
            // if undirected, add edge to tail as well
            self.adj[*end].push(edge.index);
        }

        self.curr_edge += 1;
        return edge.index;
    }

    fn remove_edge(&mut self, e: &EdgeInd) -> Edge<E> {
        let edge = self.edges.remove(e).unwrap();

        // remove from start and end lists
        let mut start_edges = &mut self.adj[edge.start];
        start_edges.remove(start_edges.iter().position(|&i| i == edge.index).unwrap());

        if !self.is_directed() {
            let mut end_edges = &mut self.adj[edge.end];
            end_edges.remove(end_edges.iter().position(|&i| i == edge.index).unwrap());
        }

        return edge;
    }

    fn nodes(&self) -> Box<dyn Iterator<Item = NodeInd>> {
        Box::new(
            self.nodes
                .keys()
                .map(|&n| n)
                .collect::<Vec<NodeInd>>()
                .into_iter(),
        )
    }

    fn edges(&self) -> Box<dyn Iterator<Item = EdgeInd>> {
        Box::new(
            self.edges
                .keys()
                .map(|&n| n)
                .collect::<Vec<EdgeInd>>()
                .into_iter(),
        )
    }

    fn edges_from(&self, n: &NodeInd) -> Box<dyn Iterator<Item = Edge<E>>> {
        todo!()
    }

    fn edges_to(&self, n: &NodeInd) -> Box<dyn Iterator<Item = Edge<E>>> {
        todo!()
    }

    fn edges_at(&self, n: &NodeInd) -> Box<dyn Iterator<Item = Edge<E>>> {
        todo!()
    }

    fn edge_endpoints(&self, e: &EdgeInd) -> (NodeInd, NodeInd) {
        todo!()
    }
}
