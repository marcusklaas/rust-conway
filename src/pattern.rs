extern crate collections;
extern crate core;

use std::cmp::{min, max};
use std::mem::{replace, swap};
use self::core::slice::Items;

// these values are actually extremes. so 2x2 figure would have rows = 1 and columns = 1
struct Bounds {
    rows:    uint,
    columns: uint
}

pub struct Pattern {
    coordinate_list: Box<[(uint, uint)]>,
    bounds: Bounds
}

impl Pattern {
    pub fn from_pairs(pair_list: &[(uint, uint)]) -> Pattern {        
        let top  = pair_list.iter().fold(-1u, |acc, &(row, _)| min(acc, row));
        let left = pair_list.iter().fold(-1u, |acc, &(_, col)| min(acc, col));
        let mut vec: Vec<(uint, uint)> = pair_list.iter()
          .map(|&(row, col)| (row - top, col - left))
          .collect();
    
        Pattern {
            coordinate_list: vec.into_boxed_slice(),
            bounds: Bounds {
                rows:    pair_list.iter().fold(0, |acc, &(row, _)| max(acc, row)) - top,
                columns: pair_list.iter().fold(0, |acc, &(_, col)| max(acc, col)) - left,
            }
        }
    }

    // fix this! we should be able to work it out without allocation
    pub fn rotate_right(&mut self) {
        let mut vec: Vec<(uint, uint)> = self.coordinate_list.iter().map(|&(row, col)| (col, self.bounds.rows - row)).collect();
        let boxed = vec.into_boxed_slice();
        
        replace(&mut self.coordinate_list, boxed);
        swap(&mut self.bounds.rows, &mut self.bounds.columns);
    }
    
    pub fn iter<'r>(&'r self) -> Items<(uint, uint)> {
        self.coordinate_list.iter()
    }
    
    pub fn get_width(&self) -> uint {
        self.bounds.columns + 1
    }
    
    pub fn get_height(&self) -> uint {
        self.bounds.rows + 1
    }
}

pub fn get_glider() -> Pattern {
    Pattern::from_pairs(&[(0, 1), (1, 2), (2, 0), (2, 1), (2, 2)])
}

pub fn get_acorn() -> Pattern {
    Pattern::from_pairs(&[(0, 1), (1, 3), (2, 0), (2, 1), (2, 4),
      (2, 5), (2, 6)])
}
