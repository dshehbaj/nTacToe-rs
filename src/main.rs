mod board;
mod player;
mod utils;

fn main() {
    let board: board::Board = board::Board::new(3, 32);
    utils::print_grid(&board.get_grid(), board.get_size());
    println!("Yo");
}
