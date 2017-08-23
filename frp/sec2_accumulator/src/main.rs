extern crate console;
extern crate carboxyl;

use std::io;
use std::io::prelude::*;

use std::thread;

fn main() {
    println!("0");
    println!("input + to increment, input - to decrement");

    let plus_sink = carboxyl::Sink::new();
    let plus_stream = plus_sink.stream().map(|u: usize| 1);
    let minus_sink = carboxyl::Sink::new();
    let minus_stream = minus_sink.stream().map(|u: usize| -1);

    let delta_stream = plus_stream.merge(&minus_stream);
    let value_sync = carboxyl::Sink::new();
    let value_stream = value_sync.stream();
    let mut events = value_stream.hold(0).snapshot(&delta_stream, move |a, b| a + b).events();

    let th1 = thread::spawn(move || {
        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            match line.unwrap().as_str() {
                "+" => plus_sink.send(0),
                "-" => minus_sink.send(0),
                _ => println!("skip"),
            }
        }
    });

    let th2 = thread::spawn(move || {
        loop {
            let value = events.next().unwrap();
            value_sync.send(value);
            println!("thread2: {}", value);
        }
    });

    let _ = th1.join();
    let _ = th2.join();

    //  let _ = th3.join();
}
