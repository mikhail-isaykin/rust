use std::cell::RefCell;
use std::rc::Rc;

type Chat = Rc<RefCell<Vec<String>>>;

fn send_message(chat: &Chat, user: &str, text: &str) {
    chat.borrow_mut()
        .push(format!("{}: {}", user, text));
}

fn main() {
    let room = Rc::new(RefCell::new(Vec::new()));

    let user1 = room.clone();
    let user2 = room.clone();

    send_message(&user1, "Alex", "Hello!");
    send_message(&user2, "John", "Hi!");
    send_message(&user1, "Alex", "How are you?");

    println!("Chat history:");

    for msg in room.borrow().iter() {
        println!("{}", msg);
    }
}
