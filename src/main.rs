use std::io::{stdout, stdin, Write};

mod board;
mod player;
mod utils;


fn main() {

    let size: i32 = 3;
    let empty_value: i32 = 32;
    let mut buff: String = String::new();

    let mut board: board::Board = board::Board::new(size, empty_value);
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

    let mut idx: usize = 0;
    let mut spots: i32 = size * size;
    while spots != 0 {
        println!("\n");
        let curr_player: &player::Player = player_list.get(idx).unwrap();
        println!("{}'s turn!", curr_player.get_name());
        utils::print_grid(&board.get_grid(), size);
        buff.clear();
        stdin().read_line(&mut buff).expect("Error while taking input");
        let coord: i32 = utils::translate(buff.trim().parse().unwrap());
        if board.place_mark(coord, curr_player.get_symbol()) {
            idx += 1;
            idx %= num_players as usize;
            spots -= 1;
            if board.check_board() {
                println!();
                utils::print_grid(&board.get_grid(), size);
                println!("{} is the winner!", curr_player.get_name());
                break;
            }
        }
        else {
            println!("Error placing mark! Try again!");
        }
    }

    if spots == 0 {
        println!("\nDraw!");
        utils::print_grid(&board.get_grid(), size);
    }

    return;
}
