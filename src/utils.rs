use std::io::{stdout, Write};

pub fn print_grid(grid: &Vec<Vec<i32>>) {
    io::stdout().flush();
    for row in grid {
        print!(" ");
        for num in row {
            print!("{} |", num as char);
        }
    }
}
