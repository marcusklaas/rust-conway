extern crate collections;
extern crate core;

use std::cmp::{min, max};
use std::mem::{replace, swap};
use self::core::slice::Items;

pub struct Pattern {
    coordinate_list: Box<[(uint, uint)]>,
    farthest_right: uint,
    farthest_down:  uint
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
            farthest_right:  pair_list.iter().fold(0, |acc, &(_, col)| max(acc, col)) - left,
            farthest_down:   pair_list.iter().fold(0, |acc, &(row, _)| max(acc, row)) - top
        }
    }

    pub fn rotate_right(&mut self) {
        for pair in self.coordinate_list.iter_mut() {
            let (row, col) = *pair;
            *pair = (col, self.farthest_down - row);
        }
        
        swap(&mut self.farthest_down, &mut self.farthest_right);
    }
    
    pub fn iter<'r>(&'r self) -> Items<(uint, uint)> {
        self.coordinate_list.iter()
    }
    
    pub fn get_width(&self) -> uint {
        self.farthest_right + 1
    }
    
    pub fn get_height(&self) -> uint {
        self.farthest_down + 1
    }
}

pub fn get_glider() -> Pattern {
    Pattern::from_pairs(&[(0, 1), (1, 2), (2, 0), (2, 1), (2, 2)])
}

pub fn get_acorn() -> Pattern {
    Pattern::from_pairs(&[(0, 1), (1, 3), (2, 0), (2, 1), (2, 4), (2, 5), (2, 6)])
}
