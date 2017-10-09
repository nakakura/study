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
        } else if next.eq(&"tixe".to_string()) {
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
    println!("send messate to localhost:3333 instead for button click");
    let sink = carboxyl::Sink::new();
    let stream: carboxyl::Stream<String> = sink.stream().map(|x: String| x.chars().rev().collect::<String>());
    let cell = stream.hold("hoge".to_string());

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

    let th2 = thread::spawn(move || {
        display(cell);
    });

    let _ = th1.join();
    let _ = th2.join();
}
