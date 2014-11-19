use std::mem::swap;
use pattern::Pattern;

pub mod comm;
pub mod pattern;

pub struct GameState {
    pub rows: uint,
    pub columns: uint,
    front: Vec<u8>,
    back: Vec<u8>
}

impl Index<(uint, uint), u8> for GameState {
    fn index(&self, _index: &(uint, uint)) -> &u8 {
        let (row, column) = *_index;
        
        &self.front[self.get_index(row, column)]
    }
}

impl IndexMut<(uint, uint), u8> for GameState {
    fn index_mut(&mut self, _index: &(uint, uint)) -> &mut u8 {
        let (row, column) = *_index;
        let real_index = self.get_index(row, column);
        
        &mut self.front[real_index]
    }
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
        
        let mut result = GameState::new(
            parts.iter().fold(0u, |acc, val| acc + val.rows),
            parts[0].columns
        );
        
        let mut result_row = 0;
        
        for state in parts.iter() {
            for line_number in range(0, state.rows) {
                let row = state.read_line(line_number);
                
                result.set_line(result_row, row);
                result_row += 1;
            }
        }
        
        Some(result)
    }
    
    fn get_index(&self, row: uint, column: uint) -> uint {    
        (row + 1) * (self.columns + 2) + column + 1
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
        range(0, steps).map(|_| self.next()).last();
    }
    
    fn next_value(&self, row: uint, column: uint) -> u8 {
        let neighbour_sum = self[(row - 1, column - 1)]
                          + self[(row - 1, column)]
                          + self[(row - 1, column + 1)]
                          + self[(row, column - 1)]
                          + self[(row, column + 1)]
                          + self[(row + 1, column - 1)]
                          + self[(row + 1, column)]
                          + self[(row + 1, column + 1)];
                          
        let cell = self[(row, column)];
                          
        match (cell, neighbour_sum) {
            (_, 3) => 1u8,
            (1, 2) => 1u8,
            _      => 0u8
        }
    }
    
    pub fn print(&self, print_funk: |uint, uint| -> ()) {
        for row in range(0, self.rows) {
            for column in range(0, self.columns) {            
                if 1u8 == self[(row, column)] {
                    print_funk(row, column);
                }
            }
        }
    }
    
    pub fn add_pattern(&mut self, pattern: &Pattern, start_row: uint, start_column: uint) {
        for &(row, column) in pattern.iter() {
            self[(start_row + row, start_column + column)] = 1u8;
        }
    }
    
    pub fn split(&self, pieces: uint) -> Vec<GameState> {                
        Vec::from_fn(pieces, |i| {
            let row_from = self.rows * i / pieces;
            let row_to   = self.rows * (i + 1) / pieces;
            let mut part = GameState::new(row_to - row_from, self.columns);
            
            for (part_row_number, row_number) in range(row_from, row_to).enumerate() {
                part.set_line(part_row_number, self.read_line(row_number));
            }
            
            part
        })
    }
    
    pub fn read_top(&self) -> Vec<u8> {
        self.read_line(0).iter().map(|&x| x).collect()
    }
    
    pub fn read_bottom(&self) -> Vec<u8> {
        self.read_line(self.rows - 1).iter().map(|&x| x).collect()
    }
    
    fn read_line<'a>(&'a self, line: uint) -> &'a [u8] {
        let index_from = self.get_index(line, 0);
        let index_to   = self.get_index(line, self.columns);
    
        self.front.slice_or_fail(&index_from, &index_to)
    }
    
    #[allow(unsigned_negation)]
    pub fn set_top(&mut self, line: &[u8]) {
        self.set_line(-1u, line);
    }
    
    pub fn set_bottom(&mut self, line: &[u8]) {
        let line_number = self.rows;
        
        self.set_line(line_number, line);
    }
    
    fn set_line(&mut self, line_number: uint, line: &[u8]) {    
        for (i, &value) in line.iter().enumerate() {
            self[(line_number, i)] = value;
        }
    }    
}
