pub struct Stack<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack{
            head: None,
        }
    }

    pub fn push(&mut self, value: T) {
        let link = Some(Box::new(Node {
            value,
            next: self.head.take(),
        }));
        self.head = link;
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node|{
            self.head = node.next;
            node.value
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| {
            &node.value
        })
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    pub fn iter(&self) -> Iter<T> {
        Iter { next: self.head.as_ref().map(|node| &**node) }
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut { next: self.head.as_mut().map(|node| &mut **node) }
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
pub struct IntoIter<T>(Stack<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
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

pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
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

pub struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

mod test {
    use super::Stack;

    #[test]
    fn push_pop() {
        let mut stack = Stack::new();
        stack.push(1); stack.push(2); stack.push(3);

        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn peek() {
        let mut stack = Stack::new();
        stack.push(1);

        assert_eq!(stack.peek(), Some(&1));
    }

    #[test]
    fn into_iter() {
        let mut stack = Stack::new();
        stack.push(8);
        stack.push(10);
        stack.push(12);

        let mut iter = stack.into_iter();
        assert_eq!(iter.next(), Some(12));
        assert_eq!(iter.next(), Some(10));
        assert_eq!(iter.next(), Some(8));
    }

    #[test]
    fn iter() {
        let mut stack = Stack::new();
        stack.push(8);
        stack.push(10);
        stack.push(12);

        let mut iter = stack.iter();
        assert_eq!(iter.next(), Some(&12));
        assert_eq!(iter.next(), Some(&10));
        assert_eq!(iter.next(), Some(&8));
    }

    #[test]
    fn mut_iter() {
        let mut stack = Stack::new();
        stack.push(8);
        stack.push(10);
        stack.push(12);

        let mut iter = stack.iter_mut();
        assert_eq!(iter.next(), Some(&mut 12));
        assert_eq!(iter.next(), Some(&mut 10));
        assert_eq!(iter.next(), Some(&mut 8));
    }
}
