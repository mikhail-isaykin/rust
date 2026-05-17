use std::collections::HashSet;
use std::io;

fn write(s: &mut String) -> () {
    io::stdin()
        .read_line(s)
        .unwrap();
}

fn main() {
    let mut team: HashSet<String> = HashSet::new();
    let mut num_players = String::new();
    
    write(&mut num_players);
    
    let num_players: u32 = num_players.trim().parse().unwrap();

    for _ in 0..num_players {
        let mut player: String = String::new();
        write(&mut player);
        team.insert(player.trim().to_string());
    }

    println!("Количество: {:?}", team.len());

}