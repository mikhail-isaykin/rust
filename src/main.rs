use std::collections::HashSet;
use std::io;

fn main() {
    let allowed_employees: HashSet<i32> = HashSet::from([123, 456, 789, 101, 202]);


    let mut employee_id = String::new();

    io::stdin()
        .read_line(&mut employee_id)
        .unwrap();

    let employee_id: i32 = employee_id.trim().parse().unwrap();

    println!("{}", allowed_employees.contains(&employee_id));

}