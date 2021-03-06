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
/*
fn display<F>(mut cell: futures::stream::Map<futures::sync::mpsc::Receiver<String>, F>) {
    let mut previous = "".to_string();
    while let Some(x) = cell.poll() {
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
*/
fn main() {
    println!("send messate to localhost:3333 instead for button click");
    println!("default");
    let (tx, rx): (mpsc::Sender<String>, mpsc::Receiver<String>) = mpsc::channel::<String>(5);
    let mut rx = rx.map(|x| format!("{}-xxx", x));

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
            x.send(str.to_string()).wait().unwrap()
        });
    });

    let th2 = thread::spawn(move || {
        let mut core = Core::new().unwrap();
        let x = rx.for_each(|x| {
            println!("{}", x);
            Ok(())
        });
        core.run(x);
    });


    let _ = th2.join();
    println!("close th2");
    let _ = th1.join();
    println!("close th1");
}
