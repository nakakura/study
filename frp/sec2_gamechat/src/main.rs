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

    let sink1 = carboxyl::Sink::new();
    let stream1 = sink1.stream();
    let sink2 = carboxyl::Sink::new();
    let stream2 = sink2.stream();

    let merged_stream = stream1.merge(&stream2).hold("hoge".to_string());

    let stdin = io::stdin();
    let mut x = false;
    for line in stdin.lock().lines() {
        let message = line.unwrap();
        if x {
            sink1.send(message);
        } else {
            sink2.send(message);
        }
        x = !x;
        let _ = set_text_field(merged_stream.sample());
    }
}