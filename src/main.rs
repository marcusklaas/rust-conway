#![feature(globs)]

extern crate getopts;
extern crate conway;
extern crate ncurses;
extern crate test;

use getopts::{optflag, getopts, reqopt};
use std::os;
use conway::{GameState, DuplexChannel};
use ncurses::*;
use std::io::Timer;
use std::time::Duration;

use self::test::Bencher;

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
    
    test_animation();
    
    test_concurrency(thread_count);
}

fn test_animation() {
    let mut width = 120;
    let mut height = 20;

    initscr();
    curs_set(CURSOR_INVISIBLE);
    cbreak(); // enable <Ctrl+C> to kill game
    noecho(); // don't show input
    getmaxyx(stdscr, &mut height, &mut width);
    timeout(2000);

    let mut state = GameState::new(width as uint, height as uint);
    
    state.add_glider(1, 5);
    
    let mut timer = Timer::new().unwrap();
    let periodic = timer.periodic(Duration::milliseconds(40));
    
    for _ in range(0, 100u) {
        state.print();
        periodic.recv();
        
        state.progress(1);        
    }
    
    endwin();
}

fn test_concurrency(thread_count: uint) {
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

static BENCH_SIZE: uint = 2;

#[bench]
fn naive_implementation(b: &mut Bencher) {
    let mut state = GameState::new(300, 150);
    
    state.add_glider(1, 5);
    
    b.iter(|| state.progress(BENCH_SIZE))
}

