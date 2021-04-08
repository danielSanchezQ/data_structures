use std::fmt::Debug;

#[derive(Debug)]
pub struct SequenceTree<T: Debug + Eq> {
    nodes: Vec<Node<T>>,
    root: usize,
}

#[derive(Debug)]
pub struct Node<T: Debug + Eq> {
    key: T,
    children: Vec<usize>,
}

impl<T> SequenceTree<T>
where
    T: Debug + Eq + Clone,
{
    pub fn new(nodes: Vec<Node<T>>, root: usize) -> Self {
        Self { nodes, root }
    }

    pub fn height(&self) -> usize {
        self.nodes[self.root].height(&self.nodes)
    }
}

impl<T> Node<T>
where
    T: Debug + Eq + Clone,
{
    pub fn new(key: T) -> Self {
        Self {
            key,
            children: Vec::new(),
        }
    }

    pub fn add_child(&mut self, child: usize) {
        self.children.push(child);
    }

    pub fn height(&self, nodes: &[Node<T>]) -> usize {
        1 + self
            .children
            .iter()
            .map(|c| nodes[*c].height(&nodes))
            .max()
            .unwrap_or(0)
    }
}

impl SequenceTree<usize> {
    pub fn from_input(parentship: &[isize]) -> Self {
        let mut nodes: Vec<Node<usize>> = (0..parentship.len()).map(Node::new).collect();
        let mut root = 0;
        for child_index in 0..parentship.len() {
            let parent_index = parentship[child_index];
            if parent_index == -1 {
                root = child_index;
            } else {
                let n = nodes.get_mut(parent_index as usize).unwrap();
                n.add_child(child_index);
            }
        }
        Self { nodes, root }
    }
}

#[cfg(test)]
mod test {
    use crate::tree::SequenceTree;

    #[test]
    fn test_tree_build_from_input() {
        let parents = [4, -1, 4, 1, 1];
        let tree = SequenceTree::from_input(&parents);
        println!("{:?}", tree);
        assert_eq!(tree.height(), 3);
    }

    #[test]
    fn test_tree_build_from_input2() {
        let parents = [9, 7, 5, 5, 2, 9, 9, 9, 2, -1];
        let tree = SequenceTree::from_input(&parents);
        println!("{:?}", tree);
        assert_eq!(tree.height(), 4);
    }
}

