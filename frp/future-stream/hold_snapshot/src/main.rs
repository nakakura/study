extern crate console;
extern crate futures;
extern crate tokio_core;

use console::Term;
use futures::*;
use futures::prelude::*;
use futures::executor;
use futures::future::{err, ok};
use futures::stream::{empty, iter_ok, poll_fn, Peekable};
use futures::sync::oneshot;
use futures::sync::mpsc;
use tokio_core::reactor::Core;

use std::sync::Arc;
use std::thread;
use std::time::Duration;
use std::io;
use std::io::prelude::*;
use std::net::TcpListener;

fn clear(lines: usize) -> io::Result<()> {
    let term = Term::stdout();
    term.clear_last_lines(lines)?;
    Ok(())
}

fn display(cell: mpsc::Receiver<String>) {
    let mut previous = "".to_string();
    let mut cell = cell.wait();
    while let Some(x) = cell.next() {
        let flag = x.and_then(|next| {
            if previous.eq(&next) {
                thread::sleep(Duration::from_millis(10));
                Ok(())
            } else if next.eq(&"exit".to_string()) {
                Err(())
            } else {
                let _ = clear(1);
                println!("{}", next);
                previous = next;
                Ok(())
            }
        });
        if flag.is_err() {
            return;
        }
    }
}

fn main() {
    println!("send messate to localhost:3333 instead for button click");
    let (tx, rx): (mpsc::Sender<isize>, mpsc::Receiver<isize>) = mpsc::channel::<isize>(5);

    let (minus_input, minus_output): (mpsc::Sender<isize>, mpsc::Receiver<isize>) = mpsc::channel::<isize>(5);
    minus_output.map(|x| -1);

    let rx = rx.fold(0, |sum, val|{
        println!("{}", sum);
        Ok(sum + val)
    });

    let listener = TcpListener::bind("0.0.0.0:3333").unwrap();
    let th1 = thread::spawn(move || {
        listener.incoming().fold(tx, |x, stream| {
            let s = stream.unwrap();
            let mut str = String::default();
            let mut buf_reader = io::BufReader::new(s);
            let _ = buf_reader.read_line(&mut str);
            let len = str.len();
            str.truncate(len - 1);

            if str.eq(&"exit".to_string()) {
                std::process::exit(1);
            }
            println!("send");
            x.send(100).wait().unwrap()
        });
    });

    let display_th = thread::spawn(move || {
        /*
        let mut core = Core::new().unwrap();
        let rx = rx.into_stream().for_each(|x| {
            println!("{}", x);
            Ok(())
        });
        core.run(rx);
        */
        let x = rx.wait();
    });

    let _ = th1.join();
    let _ = display_th.join();
}
