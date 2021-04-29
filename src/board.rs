pub struct Board {
    size: i32,
    rows: Vec<Vec<i32>>
}

impl Board {
    fn new(size: i32) -> Self {
        let mut rows: Vec<Vec<i32>> = Vec::with_capacity(size as usize);
        for i in 0..size {
            let mut row: Vec<i32> = Vec::with_capacity(size as usize);
            row.resize(size as usize, 10);
        }
        return Board {
            size,
            rows
        };
    }

    fn check_rows(&self) -> bool {
        let mut flag: bool = true;
        for row in &self.rows {
            flag = self.check_row(row);
        }
        return flag;
    }

    fn check_columns(&self) -> bool {
        return true;
    }

    fn check_diagonals(&self) -> bool {
        return true;
    }

    fn check_row(&self, row: &Vec<i32>) -> bool {
        for i in 0..(row.len() - 1) {
            if row[i] != row[i + 1] {
                return false;
            }
        }
        return true;
    }

    fn is_occupied(&self, idx: i32) -> bool {
        let row: usize = (idx / self.size) as usize;
        let col: usize = (idx % self.size) as usize;
        return self.rows[row][col] != 10;
    }

    fn check_board(&self) -> bool {
        return self.check_diagonals()
            && self.check_columns()
            && self.check_rows();
    }

    fn get_grid(&self) -> Vec<Vec<i32>> {
        return self.rows.to_vec();
    }
}
