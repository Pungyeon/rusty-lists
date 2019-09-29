
pub struct BinaryTree {
    head: Option<Box<BinaryNode>>,
}

impl BinaryTree {
    pub fn new(depth: usize) -> BinaryTree {
        BinaryTree {
            head: Some(Box::new(BinaryNode {
                left: BinaryNode::new(depth, 0),
                right: BinaryNode::new(depth, 0),
            })),
        }
    }

    pub fn count(self) -> usize {
        self.head.map(|node| {
            node.count()
        }).unwrap()
    }

    pub fn print(self) {
        self.head.map(|node| {
            node.print(0);
        });
    }
}

pub struct BinaryNode {
    left: Option<Box<BinaryNode>>,
    right: Option<Box<BinaryNode>>,
}

impl BinaryNode {
    pub fn new(depth: usize, current_step: usize) -> Option<Box<BinaryNode>> {
        if depth == current_step {
            Some(Box::new(BinaryNode{
                left: None,
                right: None,
            }))
        } else {
            Some(Box::new(BinaryNode{
                left: BinaryNode::new(depth, current_step+1),
                right: BinaryNode::new(depth, current_step+1),
            }))
        }
    }

    pub fn count(self) -> usize {
        BinaryNode::_count(self.left) + BinaryNode::_count(self.right)
    }

    fn _count(side: Option<Box<BinaryNode>>) -> usize {
        match side {
            None => 0,
            Some(left) => 1+left.count(),
        }
    }

    pub fn print(self, depth: usize) {
        self.left.map(|node| {
            println!("{}{}", BinaryNode::tabs(depth), depth);
            node.print(depth+1);
        });
        self.right.map(|node| {
            println!("{}{}", BinaryNode::tabs(depth), depth);
            node.print(depth+1);
        });
    }

    fn tabs(depth: usize) -> String {
        let mut tabs = String::from("\t");
        for _ in 0..depth {
            tabs += "\t";
        }
        tabs
    }
}

mod test {
    use super::BinaryTree;
    #[test]
    fn test_binary_tree_count() {
        let bin_t = BinaryTree::new(0);
        assert_eq!(bin_t.count(), 2);

        let bin_t = BinaryTree::new(1);
        assert_eq!(bin_t.count(), 6);

        let bin_t = BinaryTree::new(2);
        assert_eq!(bin_t.count(), 14);
    }
}