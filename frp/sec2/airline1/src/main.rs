extern crate console;
extern crate carboxyl;

use std::io;
use std::io::prelude::*;
use std::time::Duration;
use std::thread;

use console::{Term, style};


fn set_text_field(message: Option<&str>) -> io::Result<()> {
    let term = Term::stdout();
    term.clear_last_lines(3)?;
    message.map(|m|{
        term.write_line(format!("box: {}", m).as_str());
        term.write_line("press enter to clear box");
    });
    Ok(())
}

fn main() {
    println!("box: hello");
    println!("press enter to clear box");

    let sink = carboxyl::Sink::new();
    let stream = sink.stream();

    let mut events = stream.map(|x: String| "").events();
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let message = line.unwrap();
        sink.send(message);
        set_text_field(events.next()).unwrap();
    }
}