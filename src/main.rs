struct IntoIter<T>(Stack<T>);

impl<T> Stack<T> {
    fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<T> Stack<T> {
    fn iter(&self) -> Iter<T> {
        Iter { next: self.head.as_ref().map(|node| &**node) }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_ref().map(|node| &**node);
            &node.value
        })
    }
}

struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<T> Stack<T> {
    fn iter(&mut self) -> IterMut<'_, T> {
        IterMut { next: self.head.as_mut().map(|node| &mut **node) }
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut  T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_mut().map(|node| &mut **node);
            &mut node.value
        })
    }
}

struct Stack<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Stack{
           head: None,
        }
    }

    fn push(&mut self, value: T) {
        let link = Some(Box::new(Node {
            value,
            next: self.head.take(),
        }));
        self.head = link;
    }

    fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node|{
            self.head = node.next;
            node.value
        })
    }

    fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| {
            &node.value
        })
    }
}

impl<T> Drop for Stack<T> {
    fn drop(&mut self) {
        let mut current_link = self.head.take();
        while let Some(mut node) = current_link {
            current_link = node.next.take();
        }
    }
}

struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

struct BinaryTree {
    head: Option<Box<BinaryNode>>,
}

impl BinaryTree {
    fn new(depth: usize) -> BinaryTree {
        BinaryTree {
            head: Some(Box::new(BinaryNode {
                left: BinaryNode::new(depth, 0),
                right: BinaryNode::new(depth, 0),
            })),
        }
    }

    fn count(self) -> usize {
        self.head.map(|node| {
            node.count()
        }).unwrap()
    }

    fn print(self) {
        self.head.map(|node| {
            node.print(0);
        });
    }
}

struct BinaryNode {
    left: Option<Box<BinaryNode>>,
    right: Option<Box<BinaryNode>>,
}

impl BinaryNode {
    fn new(depth: usize, current_step: usize) -> Option<Box<BinaryNode>> {
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

    fn count(self) -> usize {
        BinaryNode::_count(self.left) + BinaryNode::_count(self.right)
    }

    fn _count(side: Option<Box<BinaryNode>>) -> usize {
        match side {
            None => 0,
            Some(left) => 1+left.count(),
        }
    }

    fn print(self, depth: usize) {
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
        let mut tabs = "".to_string();
        for _ in 0..depth {
            tabs += "\t";
        }
        tabs
    }
}

fn main() {
    let mut stack : Stack<i32> = Stack::new();
    stack.push(32);
    stack.push(14);
    stack.push(8);

    for value in stack.into_iter() {
        println!("{}", value);
    }

    let bin_t = BinaryTree::new(3);
    println!("Elements: {}", bin_t.count());
}

#[test]
fn test_binary_tree_count() {
    let bin_t = BinaryTree::new(0);
    assert_eq!(bin_t.count(), 2);

    let bin_t = BinaryTree::new(1);
    assert_eq!(bin_t.count(), 6);

    let bin_t = BinaryTree::new(2);
    assert_eq!(bin_t.count(), 14);
}

#[test]
fn into_iter() {
    let mut stack = Stack::new();
    stack.push(8); stack.push(10); stack.push(12);

    assert_eq!(stack.into_iter().next(), Some(12));
}
