pub struct Board {
    empty_value: i32,
    size: i32,
    rows: Vec<Vec<i32>>
}

impl Board {
    pub fn new(size: i32, empty_value: i32) -> Self {
        let mut rows: Vec<Vec<i32>> = Vec::new();
        for _i in 0..size {
            let mut row: Vec<i32> = Vec::with_capacity(size as usize);
            row.resize(size as usize, empty_value);
            rows.push(row);
        }
        return Board {
            empty_value,
            size,
            rows
        };
    }

    pub fn check_rows(&self) -> bool {
        for row in &self.rows {
            if self.check_row(&row) {
                return true;
            }
        }
        return false;
    }

    pub fn check_columns(&self) -> bool {
        for i in 0..self.size {
            let mut row: Vec<i32> = Vec::new();
            for j in 0..self.size {
                row.push(self.rows[j as usize][i as usize]);
            }
            if self.check_row(&row) {
                return true;
            }
        }
        return false;
    }

    pub fn check_diagonals(&self) -> bool {
        let mut diag1: Vec<i32> = Vec::new();
        let mut diag2: Vec<i32> = Vec::new();
        for i in 0..self.size {
            diag1.push(self.rows[i as usize][i as usize]);
            diag2.push(self.rows[(self.size - 1 - i) as usize][i as usize]);

        }
        return self.check_row(&diag1) || self.check_row(&diag2);
    }

    pub fn check_row(&self, row: &Vec<i32>) -> bool {
        for i in 0..(row.len() - 1) {
            if row[i] == self.empty_value || row[i] != row[i + 1] {
                return false;
            }
        }
        return true;
    }

    pub fn place_mark(&mut self, idx: i32, mark: char) -> bool {
        if !self.is_occupied(idx) {
            let row: usize = (idx / self.size) as usize;
            let col: usize = (idx % self.size) as usize;
            self.rows[row][col] = mark as i32;
            return true;
        }
        return false;
    }

    pub fn is_occupied(&self, idx: i32) -> bool {
        let row: usize = (idx / self.size) as usize;
        let col: usize = (idx % self.size) as usize;
        return self.rows[row][col] != self.empty_value;
    }

    pub fn check_board(&self) -> bool {
        return self.check_diagonals()
            || self.check_columns()
            || self.check_rows();
    }

    pub fn get_grid(&self) -> Vec<Vec<i32>> {
        return self.rows.to_vec();
    }

    pub fn get_size(&self) -> i32 {
        return self.size;
    }
}

