mod lists;
use lists::stack::Stack;
use lists::binary::BinaryTree;

fn main() {
    let mut stack : Stack<i32> = Stack::new();
    stack.push(32);
    stack.push(14);
    stack.push(8);

    for value in stack.into_iter() {
        println!("{}", value);
    }

    let bin_t = BinaryTree::new(3);
    bin_t.print();
}
