pub struct Tree {
    nodes: Vec<NodeData>,
}
pub type NodeIndex = usize;

#[derive(Debug, PartialEq)]
pub struct NodeData {
    parent_node: Option<NodeIndex>
}

impl Tree {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new()
        }
    }
    pub fn add_node(&mut self) -> NodeIndex
    {
        let node_index = self.nodes.len();
        let node = NodeData{
            parent_node: None
        };
        self.nodes.push(node);
        node_index
        
    }
    pub fn set_parent(&mut self, node: NodeIndex, parent: NodeIndex)
    {
        self.nodes[node].parent_node = Some(parent)
    }
    pub fn ancestors(&self, source: NodeIndex) -> Ancestors {
        Ancestors { tree: self, current_node_index: Some(source) }
    }
}

pub struct Ancestors<'tree> {
    tree: &'tree Tree,
    current_node_index: Option<NodeIndex>
}

impl<'tree> Iterator for Ancestors<'tree> {
    type Item = NodeIndex;
    
    fn next(&mut self) -> Option<NodeIndex> 
    {
        match self.current_node_index 
        {
            None => None,
            Some(current_node) => {
                let parent = self.tree.nodes[current_node].parent_node;
                self.current_node_index = parent;
                Some(current_node)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_tree_create()
    {
        let mut t = Tree::new();
        let a = t.add_node();
        let b = t.add_node();
        t.set_parent(b, a);
        let c = t.add_node();
        t.set_parent(c, a);

        assert!(
            t.nodes == [NodeData { parent_node: None }, 
                            NodeData { parent_node: Some(0) }, 
                            NodeData { parent_node: Some(0) }]
        );
    }
    #[test]
    fn test_tree_ancestors()
    {
        let mut t = Tree::new();
        let a = t.add_node();
        let b = t.add_node();
        t.set_parent(b, a);
        let c = t.add_node();
        t.set_parent(c, a);

        let d = t.add_node();
        t.set_parent(d, c);

        assert!(t.nodes == [NodeData { parent_node: None }, 
                            NodeData { parent_node: Some(0) }, 
                            NodeData { parent_node: Some(0) },
                            NodeData { parent_node: Some(2) }]
                );

        let ancestors: Vec<NodeIndex> = t.ancestors(d).collect();
        assert_eq!(ancestors, vec![3,2,0]);
    }
}