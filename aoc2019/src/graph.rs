/// Based on http://smallcultfollowing.com/babysteps/blog/2015/04/06/modeling-graphs-in-rust-using-vector-indices/
pub struct Graph {
    nodes: Vec<NodeData>,
    edges: Vec<EdgeData>,
}
pub type NodeIndex = usize;

#[derive(Debug, PartialEq)]
pub struct NodeData {
    first_outgoing_edge: Option<EdgeIndex>,
}

pub type EdgeIndex = usize;

#[derive(Debug, PartialEq)]
pub struct EdgeData {
    target: NodeIndex,
    next_outgoing_edge: Option<EdgeIndex>
}

impl Graph {
    pub fn new() -> Self {
        Self { 
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }
    pub fn add_node(&mut self) -> NodeIndex {
        let index = self.nodes.len();
        self.nodes.push(NodeData { first_outgoing_edge: None });
        index
    }
    pub fn add_edge(&mut self, source: NodeIndex, target: NodeIndex) {
        let edge_index = self.edges.len();
        let node_data = &mut self.nodes[source];
        self.edges.push(EdgeData {
            target: target,
            next_outgoing_edge: node_data.first_outgoing_edge
        });
        node_data.first_outgoing_edge = Some(edge_index);
    }
    pub fn successors(&self, source: NodeIndex) -> Successors {
        let first_outgoing_edge = self.nodes[source].first_outgoing_edge;
        Successors { graph: self, current_edge_index: first_outgoing_edge }
    }
    pub fn djikstra(&self, start_node: NodeIndex, goal_node: NodeIndex) -> Vec<NodeIndex>
    {
        use std::collections::HashMap;
        let mut vertices = Vec::new();

        let mut dist: HashMap<NodeIndex, usize> = HashMap::new();
        let mut prev: HashMap<NodeIndex, NodeIndex> = HashMap::new();
        
        for i in 0..self.nodes.len()
        {
            dist.insert(i, std::usize::MAX);
            vertices.push(i);
        }

        
        dist.insert(start_node, 0);

        while vertices.len() > 0
        {
            let (current_node_idx, current_node) = vertices.iter().enumerate()
                                                .min_by_key(|(_, n)|dist.get(n)).unwrap();
            let current_node = *current_node;

            vertices.remove(current_node_idx);
            if current_node == goal_node
            {
                break;
            }
            self.successors(current_node).for_each(|neighbor| {
                let alt = (dist[&current_node] as i32 + 1) as usize;
                if alt < dist[&neighbor]
                {
                    dist.insert(neighbor, alt);
                    prev.insert(neighbor, current_node);
                }
            });
        }
        
        // Reconstruct path
        let mut u = goal_node;
        let mut path = Vec::new();
        path.push(u);
        while let Some(next_node) = prev.get(&u)
        {
            path.push(*next_node);
            u = *next_node;
        }
        path.reverse();
        path
    }
}

pub struct Successors<'graph> {
    graph: &'graph Graph,
    current_edge_index: Option<EdgeIndex>,
}

impl<'graph> Iterator for Successors<'graph> {
    type Item = NodeIndex;
    
    fn next(&mut self) -> Option<NodeIndex> {
        match self.current_edge_index {
            None => None,
            Some(edge_num) => {
                let edge = &self.graph.edges[edge_num];
                self.current_edge_index = edge.next_outgoing_edge;
                Some(edge.target)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_graph_add()
    {
        let mut g = Graph::new();
        let a = g.add_node();
        let b = g.add_node();
        let c = g.add_node();

        assert!(g.nodes == vec![NodeData{first_outgoing_edge: None}, 
                                NodeData{first_outgoing_edge: None},
                                NodeData{first_outgoing_edge: None}]);
        
        
        g.add_edge(a, b);
        assert!(g.nodes == vec![NodeData{first_outgoing_edge: Some(0)},
                                NodeData{first_outgoing_edge: None},
                                NodeData{first_outgoing_edge: None}]);
        assert!(g.edges == vec![EdgeData{target: b, next_outgoing_edge: None}]);

        g.add_edge(a, c);
        assert!(g.nodes == vec![NodeData{first_outgoing_edge: Some(1)},
                                NodeData{first_outgoing_edge: None},
                                NodeData{first_outgoing_edge: None}]);
        assert!(g.edges == vec![EdgeData{target: b, next_outgoing_edge: None}, // a -> b
                                EdgeData{target: c, next_outgoing_edge: Some(0)}]);
        
    }
}