use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let request_count = Rc::new(RefCell::new(0));

    let service1 = request_count.clone();
    let service2 = request_count.clone();

    *service1.borrow_mut() += 3;
    *service2.borrow_mut() += 2;

    println!("Service 1 added requests");
    println!("Service 2 added requests");

    println!(
        "Total requests: {}",
        request_count.borrow()
    );

    println!(
        "Active references: {}",
        Rc::strong_count(&request_count)
    );
}
