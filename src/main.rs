
use std::thread;
use std::time::Duration;

fn process_task(task_id: u32) {
    println!("Task {} started", task_id);

    thread::sleep(Duration::from_millis(500));

    println!("Task {} completed", task_id);
}

fn main() {
    let mut handles = vec![];

    for id in 1..=3 {
        let handle = thread::spawn(move || {
            process_task(id);
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("All tasks finished");
}
