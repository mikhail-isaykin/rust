use std::cell::RefCell;
use std::rc::Rc;

type History = Rc<RefCell<Vec<String>>>;

fn add_action(history: &History, action: &str) {
    history.borrow_mut().push(action.to_string());
}

fn main() {
    let history = Rc::new(RefCell::new(Vec::new()));

    let editor = history.clone();
    let user = history.clone();

    add_action(&editor, "Created file");
    add_action(&user, "Added text");
    add_action(&editor, "Saved file");

    println!("Action history:");

    for action in history.borrow().iter() {
        println!("- {}", action);
    }

    println!("References: {}", Rc::strong_count(&history));
}
