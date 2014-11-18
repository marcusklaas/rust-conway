#![feature(globs)]

extern crate getopts;
extern crate conway;
extern crate ncurses;
extern crate test;

use getopts::{optflag, getopts, reqopt};
use std::os;
use conway::comm::DuplexChannel;
use conway::GameState;
use conway::pattern::{get_glider, Pattern};
use ncurses::*;
use std::io::Timer;
use std::time::Duration;
use test::Bencher;
use std::rand::Rng;
use std::rand;

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
    
    loop {
        test_animation();
    }
}

fn test_animation() {
    let mut width = 120;
    let mut height = 20;

    initscr();
    curs_set(CURSOR_INVISIBLE);
    cbreak(); // enable <Ctrl+C> to kill program
    noecho(); // don't show input
    getmaxyx(stdscr, &mut height, &mut width);
    
    let mut state = GameState::new(height as uint, width as uint);
    let mut glider = get_glider();
    let mut rng = rand::task_rng();
    let glider_count = 30u;
    
    // produce number of gliders
    for _ in range(0, glider_count) {
        let rotations = rng.gen::<u8>() % 4;
        
        for _ in range(0, rotations) {
            glider.rotate_right();
        }
        
        let start_row = rng.gen::<uint>() % (height as uint - glider.get_height());
        let start_col = rng.gen::<uint>() % (width as uint - glider.get_width());
        
        state.add_pattern(&glider, start_row, start_col);
    }
    
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

