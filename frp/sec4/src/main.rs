extern crate console;

use console::Term;

use std::io;
use std::io::prelude::*;
use std::net::TcpListener;
use std::thread;

fn clear() -> io::Result<()> {
    let term = Term::stdout();
    term.clear_last_lines(3)?;
    Ok(())
}

fn display(preset: f64, dollars: f64, liters: f64, fuel1: f64, fuel2: f64, fuel3: f64) {
    println!("preset {}", preset);
    println!("dollars {}", dollars);
    println!("fuel1 {}, fuel2 {}, fuel3 {}", fuel1, fuel2, fuel3);
}

fn main() {
    display(0.0, 0.0, 0.0, 0.0, 0.0, 0.0);

    let listener = TcpListener::bind("0.0.0.0:3333").unwrap();
    for stream in listener.incoming() {
        let mut s = stream.unwrap();
        let mut str = String::default();
        let mut buf_reader = io::BufReader::new(s);
        buf_reader.read_line(&mut str);
        //println!("{}hoge", str.lines().next().unwrap());
        clear();
        display(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
    }

    /*
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

*/
    //  let _ = th3.join();
}
