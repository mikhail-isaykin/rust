use std::cell::RefCell;
use std::rc::Rc;

type NodeRef = Rc<RefCell<Node>>;

struct Node {
    value: i32,
    children: Vec<NodeRef>,
}

fn main() {
    let root = Rc::new(RefCell::new(Node {
        value: 1,
        children: vec![],
    }));

    let child1 = Rc::new(RefCell::new(Node {
        value: 2,
        children: vec![],
    }));

    let child2 = Rc::new(RefCell::new(Node {
        value: 3,
        children: vec![],
    }));

    root.borrow_mut().children.push(child1.clone());
    root.borrow_mut().children.push(child2.clone());

    child1.borrow_mut().value += 10;

    println!("Root: {}", root.borrow().value);

    for child in &root.borrow().children {
        println!("Child: {}", child.borrow().value);
    }
}
