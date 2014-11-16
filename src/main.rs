extern crate getopts;
extern crate conway;

use getopts::{optflag, getopts, reqopt};
use std::os;
use conway::DuplexChannel;

#[deriving(Clone)]
struct GameState {
    rows: uint,
    columns: uint,
    positions: Vec<u8>
}

impl GameState {
    fn new(rows: uint, columns: uint) -> GameState {
        GameState {
            rows: rows,
            columns: columns,
            positions: Vec::from_elem((rows + 2) * (columns + 2), 0u8)
        }
    }

    fn next(&self) -> GameState {
        let mut next = GameState::new(self.rows, self.columns);
    
        for row in range(0, self.rows) {
            for column in range(0, self.columns) {
                let index = self.get_index(row, column);
            
                next.positions[index] = self.next_value(row, column); 
            }
        }
    
        next
    }
    
    fn progress(&self, steps: uint) -> GameState {
        match steps {
            0 => self.clone(),
            1 => self.next(),
            _ => self.next().progress(steps - 1)
        }
    }
    
    fn read(&self, row: uint, column: uint) -> u8 {
        let index = self.get_index(row, column);
    
        self.positions[index]
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
    
    fn print(&self) {
        for row in range(0, self.rows) {
            for column in range(0, self.columns) {
                let index = self.get_index(row, column);
            
                let char = match self.positions[index] {
                    0u8 => ' ',
                    _   => '#'
                };
                
                print!("{}", char);
            }
            
            print!("\n");
        }
    }
    
    fn add_glider(&mut self, row: uint, column: uint) {
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
            
            self.positions[index] = 1u8;
        }
    }
}

fn main() {
    let args: Vec<String> = os::args();

    let opts = [
        reqopt("t", "threads", "set number of concurrent threads", "THREAD_COUNT"),
        optflag("h", "help", "print this help menu")
    ];
    
    let matches = match getopts(args.tail(), opts) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };
    
    let thread_count: uint = from_str::<uint>(matches.opt_str("t").unwrap().as_slice()).unwrap();
      
    println!("Thread count: {}", thread_count);
    
    let mut state = GameState::new(10, 50);
    
    state.add_glider(1, 5);
    
    state.print();
    
    state.progress(30).print();
    
    do_test(thread_count);
}

fn do_test(thread_count: uint) {
    let mut channels: Vec<DuplexChannel<uint>> = DuplexChannel::get_chain(thread_count);
    
    for i in range(0, thread_count).rev() {
        let channel = channels.pop().unwrap();
    
        spawn(proc() {
            println!("Started task {}!", i);
        
            channel.send_top(i);
            channel.send_bottom(i);
        
            match channel.receive_top() {
                Some(x) => println!("Task {} received top message from task {}!", i, x),
                None    => ()
            };
            
            match channel.receive_bottom() {
                Some(x) => println!("Task {} received bottom message from task {}!", i, x),
                None    => ()
            };
        });
    }
}




