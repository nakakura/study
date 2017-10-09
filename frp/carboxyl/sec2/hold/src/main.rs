extern crate console;
extern crate carboxyl;

use console::Term;

use std::io;
use std::io::prelude::*;
use std::net::TcpListener;
use std::thread;
use std::time::Duration;

fn clear(lines: usize) -> io::Result<()> {
    let term = Term::stdout();
    term.clear_last_lines(lines)?;
    Ok(())
}

fn display(cell: carboxyl::Signal<String>) {
    let mut previous = "".to_string();
    loop {
        let next = cell.sample();
        if previous.eq(&next) {
            thread::sleep(Duration::from_millis(10));
            continue
        } else if next.eq(&"exit".to_string()) {
            return
        } else{
            let _ = clear(1);
            println!("{:?}", next);
            previous = next;
        }
    }
}

fn main() {
    println!("send messate to localhost:3333 instead for button click");
    println!("or localhost:4444");
    let sink = carboxyl::Sink::new();
    let stream: carboxyl::Stream<String> = sink.stream();
    let sink2 = carboxyl::Sink::new();
    let stream2: carboxyl::Stream<String> = sink2.stream();

    let merged_stream = stream.merge(&stream2);

    let listener = TcpListener::bind("0.0.0.0:3333").unwrap();
    let th1 = thread::spawn(move || {
        for stream in listener.incoming() {
            let s = stream.unwrap();
            let mut str = String::default();
            let mut buf_reader = io::BufReader::new(s);
            let _ = buf_reader.read_line(&mut str);
            let len = str.len();
            str.truncate(len - 1);
            sink.send(str.to_string());
            if str.eq("exit") {
                break;
            }
        }
    });

    let listener2 = TcpListener::bind("0.0.0.0:4444").unwrap();
    let th2 = thread::spawn(move || {
        for stream in listener2.incoming() {
            let s = stream.unwrap();
            let mut str = String::default();
            let mut buf_reader = io::BufReader::new(s);
            let _ = buf_reader.read_line(&mut str);
            let len = str.len();
            str.truncate(len - 1);
            sink2.send(str.to_string());
            if str.eq("exit") {
                break;
            }
        }
    });

    let display_th = thread::spawn(move || {
        display(merged_stream.hold("".to_string()));
    });

    let _ = th1.join();
    let _ = th2.join();
    let _ = display_th.join();
}
