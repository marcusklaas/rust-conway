#![feature(globs)]

extern crate ncurses;

use std::comm::{Sender, Receiver};
use std::cmp::max;
use std::mem::swap;
use ncurses::*;

pub struct DuplexChannel<T: Send> {
    top_sender: Option<Sender<T>>,
    top_receiver: Option<Receiver<T>>,
    bottom_sender: Option<Sender<T>>,
    bottom_receiver: Option<Receiver<T>>
}

impl<T: Send> DuplexChannel<T> {
    fn new() -> DuplexChannel<T> {
        DuplexChannel {
            top_sender: None,
            top_receiver: None,
            bottom_sender: None,
            bottom_receiver: None
        }
    }
    
    pub fn get_chain(number: uint) -> Vec<DuplexChannel<T>> {    
        let mut result_vector = Vec::from_fn(number, |_| DuplexChannel::new());
        
        for i in range(0, max(1, number) - 1) {
            let (bottom_sender, top_receiver): (Sender<T>, Receiver<T>) = channel();
            let (top_sender, bottom_receiver): (Sender<T>, Receiver<T>) = channel();
            
            result_vector[i].bottom_sender = Some(bottom_sender);
            result_vector[i].bottom_receiver = Some(bottom_receiver);
            result_vector[i+1].top_sender = Some(top_sender);
            result_vector[i+1].top_receiver = Some(top_receiver);
        }
        
        result_vector
    }
    
    pub fn send_top(&self, value: T) {
        match self.top_sender {
            Some(ref x) => x.send(value),
            None    => ()
        }
    }
    
    pub fn receive_top(&self) -> Option<T> {
        match self.top_receiver {
            Some(ref x) => Some(x.recv()),
            None    => None
        }
    }
    
    pub fn send_bottom(&self, value: T) {
        match self.bottom_sender {
            Some(ref x) => x.send(value),
            None    => ()
        }
    }
    
    pub fn receive_bottom(&self) -> Option<T> {
        match self.bottom_receiver {
            Some(ref x) => Some(x.recv()),
            None    => None
        }
    }
}

pub struct GameState {
    rows: uint,
    columns: uint,
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
    
    pub fn print(&self) {
        erase();
    
        for row in range(0, self.rows) {
            for column in range(0, self.columns) {
                let index = self.get_index(row, column);
            
                if 1u8 == self.front[index] {
                    mvaddch(row as i32, column as i32, '#' as u32);
                }
            }
        }
        
        refresh();
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
    
    // hier klopt nog niet zoveel van
    pub fn split(&self, pieces: uint) -> Result<Vec<GameState>, &'static str> {
        let total_rows = self.rows + 2;
        
        if pieces > total_rows {
            return Err("Not enough rows to split into this many pieces!");
        }
        
        let mut result = Vec::new();
        
        for i in range(0, pieces) {
            let row_low = total_rows * i / pieces; // inclusive!
            let row_high = total_rows * (i + 1) / pieces; // exclusive!
            let mut part = GameState::new(row_high - row_low, self.columns);
            
            part.front.clear();
            
            for j in range(row_low * self.columns, row_high * self.columns) {
                part.front.push(self.front[j]);
            }
            
            result.push(part);
        }
        
        Ok(result)
    }
}
