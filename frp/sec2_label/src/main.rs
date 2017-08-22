extern crate console;
extern crate carboxyl;

use console::Term;
use std::io;
use std::io::prelude::*;

fn set_text_field(message: String) -> io::Result<()> {
    let term = Term::stdout();
    term.clear_last_lines(3)?;
    term.write_line(format!("box: {}", message).as_str())?;
    term.write_line("press enter to clear box")?;
    Ok(())
}

fn main() {
    println!("box: hello");
    println!("press enter to clear box");

    let sink = carboxyl::Sink::new();
    let stream = sink.stream().hold("hello".to_string());
    let _ = set_text_field(stream.sample());

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let message = line.unwrap();
        sink.send(message);
        let _ = set_text_field(stream.sample());
    }
}