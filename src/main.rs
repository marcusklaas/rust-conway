#![feature(globs)]

extern crate getopts;
extern crate conway;
extern crate ncurses;
extern crate test;

use getopts::{optflag, getopts, optopt};
use std::os;
use conway::comm::DuplexChannel;
use conway::GameState;
use conway::pattern::{get_glider, get_acorn, Pattern};
use ncurses::*;
use std::io::Timer;
use std::time::Duration;
use test::Bencher;
use std::rand::Rng;
use std::rand;

fn main() {
    let args: Vec<String> = os::args();

    let opts = [
        optopt("t", "threads", "set number of concurrent threads", "THREAD_COUNT"),
        optflag("h", "help", "print this help menu")
    ];
    
    let thread_count = match getopts(args.tail(), &opts) {
        Ok(matches) => match matches.opt_str("t") {
            None    => 1u,
            Some(x) => match from_str::<uint>(x.as_slice()) {
                Some(number) => number,
                None         => 1u
            }
        },
        Err(f)      => 1u
    };
    
    loop {
        test_animation(acorn_test_state);
    }
}

fn glider_party_state(rows: uint, columns: uint) -> GameState {
    let mut state = GameState::new(rows, columns);
    let mut glider = get_glider();
    let mut rng = rand::task_rng();
    let glider_count = 30u;
    
    // produce number of gliders
    for _ in range(0, glider_count) {
        let rotations = rng.gen::<u8>() % 4;
        
        for _ in range(0, rotations) {
            glider.rotate_right();
        }
        
        let start_row = rng.gen::<uint>() % (rows - glider.get_height());
        let start_col = rng.gen::<uint>() % (columns - glider.get_width());
        
        state.add_pattern(&glider, start_row, start_col);
    }
    
    state
}

fn acorn_test_state(rows: uint, columns: uint) -> GameState {
    let mut state = GameState::new(rows, columns);
    let acorn = get_acorn();
    
    state.add_pattern(&acorn, rows/ 2, columns/ 2);
    
    state
}

fn test_animation(state_generator: |uint, uint| -> GameState) {
    let mut width = 120;
    let mut height = 20;

    initscr();
    curs_set(CURSOR_INVISIBLE);
    cbreak(); // enable <Ctrl+C> to kill program
    noecho(); // don't show input
    getmaxyx(stdscr, &mut height, &mut width);
    
    let mut state = state_generator(height as uint, width as uint);
    let mut timer = Timer::new().unwrap();
    let periodic = timer.periodic(Duration::milliseconds(40));
    
    for _ in range(0, 800u) {
        erase();
        
        state.print(|row, column| { 
            mvaddch(row as i32, column as i32, '#' as u32);
        });
        
        refresh();
        periodic.recv();
        
        state.progress(1);        
    }
    
    endwin();
}

fn progress_in_parallel(state: &GameState, steps: uint, thread_count: uint) -> GameState {
    let mut state_list = state.split(thread_count);
    let mut channels = DuplexChannel::<Vec<u8>>::get_chain(thread_count);
    let (result_sender, result_receiver) = channel::<(uint, GameState)>();
    
    for i in range(0, thread_count).rev() {
        let channel = channels.pop().unwrap();
        let mut state = state_list.pop().unwrap();
        let my_result_sender = result_sender.clone();
    
        spawn(proc() {        
            for _ in range(0, steps) {
                channel.send_top(state.read_top());
                channel.send_bottom(state.read_bottom());
            
                match channel.receive_top() {
                    Some(x) => state.set_top(x.as_slice()),
                    None    => ()
                };
                
                match channel.receive_bottom() {
                    Some(x) => state.set_bottom(x.as_slice()),
                    None    => ()
                };
                
                state.progress(1);
            }
            
            my_result_sender.send((i, state));
        });
    }
    
    let mut result_vec = Vec::from_fn(thread_count, |_| result_receiver.recv());
    
    result_vec.sort_by(|&(i,_), &(j,_)| i.cmp(&j));
    
    let state_vec = result_vec.into_iter().map(|(_, state)| state).collect();
    
    GameState::from_parts(&state_vec).unwrap()
}

#[bench]
fn bench_20_steps_serial(b: &mut Bencher) {
    let mut state = GameState::new(1000, 1000);
    let glider = get_glider();
    
    state.add_pattern(&glider, 1, 5);
    
    b.iter(|| {
        state.progress(20)
    });
}

#[bench]
fn bench_20_steps_concurrent(b: &mut Bencher) {
    let mut state = GameState::new(1000, 1000);
    let glider = get_glider();
    
    state.add_pattern(&glider, 1, 5);
    
    b.iter(|| {
        progress_in_parallel(&state, 20, 2)
    });
}

#[bench]
fn bench_20_steps_quad(b: &mut Bencher) {
    let mut state = GameState::new(1000, 1000);
    let glider = get_glider();
    
    state.add_pattern(&glider, 1, 5);
    
    b.iter(|| {
        progress_in_parallel(&state, 20, 4)
    });
}

