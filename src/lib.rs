use std::mem::swap;

pub mod comm;

pub struct GameState {
    pub rows: uint,
    pub columns: uint,
    front: Vec<u8>,
    back: Vec<u8>
}

impl GameState {
    pub fn new(rows: uint, columns: uint) -> GameState {
        let element_count = (rows + 2) * (columns + 2);
    
        GameState {
            rows: rows,
            columns: columns,
            front: Vec::from_elem(element_count, 0u8),
            back: Vec::from_elem(element_count, 0u8)
        }
    }
    
    pub fn from_parts(parts: &Vec<GameState>) -> Option<GameState> {
        if parts.len() == 0 {
            return None;
        }
        
        let columns = parts[0].columns;
        let mut front = Vec::from_elem(columns + 2, 0u8);
        
        for state in parts.iter() {        
            for elem in state.front.slice_or_fail(&(columns + 2), &((state.rows + 1) * (columns + 2)))
              .iter().map(|&x| x) {
                front.push(elem);
            }
        }
        
        for _ in range(0, columns + 2) {
            front.push(0u8);
        }
        
        let elem_count = front.len();
        let rows = elem_count/ (columns + 2) - 2;
        
        Some(GameState {
            rows: rows,
            columns: columns,
            front: front,
            back: Vec::from_elem(elem_count, 0u8)
        })
    }

    fn next(&mut self) {
        for row in range(0, self.rows) {
            for column in range(0, self.columns) {
                let index = self.get_index(row, column);
            
                self.back[index] = self.next_value(row, column); 
            }
        }
        
        swap(&mut self.front, &mut self.back);
    }
    
    pub fn progress(&mut self, steps: uint) {
        for _ in range(0, steps) {
            self.next();
        }
    }
    
    fn read(&self, row: uint, column: uint) -> u8 {
        let index = self.get_index(row, column);
    
        self.front[index]
    }
    
    fn get_index(&self, row: uint, column: uint) -> uint {
        (row + 1) * (self.columns + 2) + column + 1
    }
    
    fn next_value(&self, row: uint, column: uint) -> u8 {
        let neighbour_sum = self.read(row - 1, column - 1) 
                          + self.read(row - 1, column) 
                          + self.read(row - 1, column + 1) 
                          + self.read(row, column - 1) 
                          + self.read(row, column + 1) 
                          + self.read(row + 1, column - 1) 
                          + self.read(row + 1, column) 
                          + self.read(row + 1, column + 1);
                          
        let cell = self.read(row, column);
                          
        match (cell, neighbour_sum) {
            (_, 3) => 1u8,
            (1, 2) => 1u8,
            _      => 0u8
        }
    }
    
    pub fn print(&self, print_funk: |uint, uint| -> ()) {
        for row in range(0, self.rows) {
            for column in range(0, self.columns) {
                let index = self.get_index(row, column);
            
                if 1u8 == self.front[index] {
                    print_funk(row, column);
                }
            }
        }
    }
    
    pub fn add_glider(&mut self, row: uint, column: uint) {
        let pattern = [(0u, 1u), (1u, 2u), (2u, 0u), (2u, 1u), (2u, 2u)];
        let translated_pattern: Vec<(uint, uint)> = pattern
                                    .iter()
                                    .map(|&(x, y)| (x + row, y + column))
                                    .collect();
        
        self.add_pattern(& translated_pattern);
    }
    
    fn add_pattern(&mut self, pattern: & Vec<(uint, uint)>) {
        for &(row, column) in pattern.iter() {
            let index = self.get_index(row, column);
            
            self.front[index] = 1u8;
        }
    }
    
    pub fn split(&self, pieces: uint) -> Result<Vec<GameState>, &'static str> {        
        if pieces > self.rows {
            return Err("Not enough rows to split into this many pieces!");
        }
        
        Ok(Vec::from_fn(pieces, |i| {
            let row_low = self.rows * i / pieces; // inclusive!
            let row_high = self.rows * (i + 1) / pieces; // exclusive!
            let offset = (self.columns + 2) * (row_low + 1);
            
            let mut part = GameState::new(row_high - row_low, self.columns);
            
            for j in range(0, (row_high - row_low) * (self.columns + 2)) {
                let index = self.columns + j;
            
                part.front[index] = self.front[offset + j];
            }
            
            part
        }))
    }
    
    pub fn read_top(&self) -> Vec<u8> {
        self.read_line(1)
    }
    
    pub fn read_bottom(&self) -> Vec<u8> {
        self.read_line(self.rows)
    }
    
    // line number starts at 1. returns self.columns + 2 elements
    fn read_line(&self, line: uint) -> Vec<u8> {
        self.front.slice_or_fail(&(line * (self.columns + 2)), &((line + 1) * (self.columns + 2))).iter().map(|&x| x).collect()
    }
    
    pub fn set_top(&mut self, line: &Vec<u8>) {
        for (i, value) in line.iter().enumerate() {
            self.front[i] = *value;
        }
    }
    
    pub fn set_bottom(&mut self, line: &Vec<u8>) {
        for (i, value) in line.iter().enumerate() {
            self.front[(1 + self.rows) * (self.columns + 2) + i] = *value;
        }
    }
}
