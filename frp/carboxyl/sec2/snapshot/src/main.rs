extern crate console;
extern crate carboxyl;

use console::Term;

use std::io;
use std::io::prelude::*;
use std::net::TcpListener;
use std::thread;

fn clear(lines: usize) -> io::Result<()> {
    let term = Term::stdout();
    term.clear_last_lines(lines)?;
    Ok(())
}

fn display(stream: carboxyl::Stream<String>) {
    loop {
        let mut events = stream.events();
        let next = events.next();
        let _ = clear(1);
        if Some(true) == next.map(|message| {
            println!("{:?}", message);
            return message.eq(&"exit".to_string());
        }) {
            return;
        }
    }
}

fn main() {
    println!("send messate to localhost:3333 instead for button click");
    let sink = carboxyl::Sink::new();
    let stream: carboxyl::Stream<String> = sink.stream();
    let cell = stream.hold("message should be displayed after receiving a trigger".to_string());

    let trigger_sink = carboxyl::Sink::new();
    let trigger_stream: carboxyl::Stream<()> = trigger_sink.stream();

    let display_stream: carboxyl::Stream<String> = cell.snapshot(&trigger_stream, |message, _trigger| {
        message
    });

    let listener = TcpListener::bind("0.0.0.0:3333").unwrap();
    let th1 = thread::spawn(move || {
        for stream in listener.incoming() {
            let s = stream.unwrap();
            let mut str = String::default();
            let mut buf_reader = io::BufReader::new(s);
            let _ = buf_reader.read_line(&mut str);
            let len = str.len();
            str.truncate(len - 1);
            trigger_sink.send(());
            if str.eq("exit") {
                break;
            }
        }
    });

    let display_th = thread::spawn(move || {
        display(display_stream);
    });

    let _ = th1.join();
    let _ = display_th.join();
}
