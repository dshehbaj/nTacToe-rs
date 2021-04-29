use std::io::{stdout, Write};


pub fn print_grid(grid: &Vec<Vec<i32>>, size: i32) {
    stdout().flush().expect("Error while flushing stdout");
    for row in grid {
        print!(" ");
        stdout().flush().expect("Error while flushing stdout");
        for num in row {
            let temp = *num as u8;
            print!("{} | ", temp as char);
        }
        stdout().flush().expect("Error while flushing stdout");
        println!();
        for _i in 0..size {
            print!("----");
        }
        stdout().flush().expect("Error while flushing stdout");
        println!();
    }
}


pub fn translate(num: i32) -> i32 {
    if num == 7 || num == 8 || num == 9 {
        return num - 7;
    }
    else if num == 4 || num == 5 || num == 6 {
        return num - 1;
    }
    else if num == 1 || num == 2 || num == 3 {
        return num + 5;
    }
    else {
        return -1;
    }
}

