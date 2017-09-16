extern crate console;
extern crate carboxyl;

use std::io;
use std::io::prelude::*;
use std::net::TcpListener;
use std::thread;
use console::Term;

fn clear(lines: usize) -> io::Result<()> {
    let term = Term::stdout();
    term.clear_last_lines(lines)?;
    Ok(())
}

fn display(stream: carboxyl::Stream<String>) {
    let mut events = stream.events();
    let next = events.next();
    let _ = clear(1);
    println!("{:?}", next);
}

fn main() {
    println!("send messate to localhost:3333 instead for button click");
    println!("hoge");
    let sink = carboxyl::Sink::new();
    let stream: carboxyl::Stream<String> = sink.stream().map(|_x| "".to_string());

    let th1 = thread::spawn(move || {
        let listener = TcpListener::bind("0.0.0.0:3333").unwrap();
        let recv_stream = listener.incoming().next();
        let s = recv_stream.unwrap().unwrap();
        let mut str = String::default();
        let mut buf_reader = io::BufReader::new(s);
        let _ = buf_reader.read_line(&mut str);
        sink.send(str.to_string())
    });

    let th2 = thread::spawn(move || {
        display(stream);
    });

    let _ = th1.join();
    let _ = th2.join();
}
