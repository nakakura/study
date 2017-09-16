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

fn display(cell: carboxyl::Signal<isize>) {
    let mut previous = 0;
    loop {
        let next = cell.sample();
        if previous == next {
            thread::sleep(Duration::from_millis(10));
            continue
        } else{
            let _ = clear(1);
            println!("{:?}", next);
            previous = next;
        }
    }
}

fn main() {
    println!("send messate to localhost:3333 instead for button click");

    let minus_sink: carboxyl::Sink<()> = carboxyl::Sink::new();
    let minus_stream: carboxyl:: Stream<isize> = minus_sink.stream().map(|x| -1);

    let plus_sink: carboxyl::Sink<()> = carboxyl::Sink::new();
    let plus_stream: carboxyl::Stream<isize> = plus_sink.stream().map(|y| 1);

    let calc_stream = plus_stream.merge(&minus_stream);

    let display_cell = carboxyl::Signal::cyclic(|a| {
        a.snapshot(&calc_stream, |message, trigger| {
            message + trigger
        }).hold(0)
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
            if str.eq("exit") {
                break;
            } else if str.eq("+") {
                plus_sink.send(());
            } else {
                minus_sink.send(());
            }
        }
    });

    let display_th = thread::spawn(move || {
        display(display_cell);
    });

    let _ = th1.join();
    let _ = display_th.join();
}
