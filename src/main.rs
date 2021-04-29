use std::io::{stdout, stdin, Write};

mod board;
mod player;
mod utils;


fn main() {

    let size: i32 = 3;
    let empty_value: i32 = 32;
    let mut buff: String = String::new();

    let board: board::Board = board::Board::new(size, empty_value);
    let mut player_list: Vec<player::Player> = Vec::new();

    print!("Enter number of Players: ");
    stdout().flush().expect("Flush failure");
    buff.clear();
    stdin().read_line(&mut buff).expect("Error while taking input");

    let num_players: i32 = buff.trim().parse().unwrap();

    for i in 0..num_players {
        stdout().flush().expect("Flush failure");
        print!("Player {} enter symbol: ", i + 1);
        stdout().flush().expect("Flush failure");
        buff.clear();
        stdin().read_line(&mut buff).expect("Error while taking input");
        let symbol: char = buff.trim().chars().next().unwrap();
        let name: String = format!("Player {}", i + 1);
        player_list.push(player::Player::new(name, symbol));
    }

    println!("\n\n\n");
    for player in player_list {
        println!("{} with symbol {}", player.get_name(), player.get_symbol());
    }
}
