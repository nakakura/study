extern crate console;
extern crate carboxyl;

use std::io;
use std::io::prelude::*;

use std::thread;

fn main() {
    println!("box: hello");
    println!("press enter to clear box");

    let sink = carboxyl::Sink::new();
    let stream = sink.stream();

    let th1 = thread::spawn(move || {
        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            sink.send(line.unwrap());
        }
    });

    let mut event1 = stream.events();
    let th2 = thread::spawn(move || {
        loop {
            println!("thread2: {}", event1.next().unwrap());
        }
    });

    let mut event2 = stream.events();
    let th3 = thread::spawn(move || {
        loop {
            println!("thread3: {}", event2.next().unwrap());
        }
    });

    let _ = th1.join();
    let _ = th2.join();
    let _ = th3.join();
}