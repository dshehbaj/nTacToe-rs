struct Player {
    name: String,
    symbol: char
}

impl Player {
    pub fn new(name: String, symbol: char) -> Self {
        return Player {
            name,
            symbol
        }
    }

    pub fn get_name(&self) -> String {
        return self.name.to_string();
    }

    pub fn get_symbol(&self) -> char {
        return self.symbol;
    }
}

