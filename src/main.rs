use std::cell::RefCell;
use std::rc::Rc;

type Link = Option<Rc<RefCell<Node>>>;

struct Node {
    value: i32,
    next: Link,
}

fn main() {
    let first = Rc::new(RefCell::new(Node {
        value: 10,
        next: None,
    }));

    let second = Rc::new(RefCell::new(Node {
        value: 20,
        next: None,
    }));

    first.borrow_mut().next = Some(second.clone());

    second.borrow_mut().value += 5;

    println!("Первый: {}", first.borrow().value);
    println!("Второй: {}", second.borrow().value);

    if let Some(node) = &first.borrow().next {
        println!("Следующий: {}", node.borrow().value);
    }
}
