extern crate futures;
extern crate tokio_core;

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

fn main() {
    println!("Hello, world!");
}

#[test]
fn merge(){
    let (tx, rx): (mpsc::Sender<i32>, mpsc::Receiver<i32>) = mpsc::channel::<i32>(5000);
    let (tx2, rx2): (mpsc::Sender<i32>, mpsc::Receiver<i32>) = mpsc::channel::<i32>(5000);
    let mut rx = rx.select(rx2).wait();

    let th1 = thread::spawn(move|| {
        let x = tx.send(1).wait().unwrap();
        thread::sleep(Duration::from_millis(1000));
        let x = x.send(2).wait().unwrap();
    });

    thread::sleep(Duration::from_millis(10));

    let th2 = thread::spawn(move|| {
        let x = tx2.send(10).wait().unwrap();
        thread::sleep(Duration::from_millis(1000));
        let x = x.send(20).wait().unwrap();
    });

    assert_eq!(rx.next(), Some(Ok(1)));
    assert_eq!(rx.next(), Some(Ok(10)));
    assert_eq!(rx.next(), Some(Ok(2)));
    assert_eq!(rx.next(), Some(Ok(20)));
    assert_eq!(rx.next(), None);
    let _ = th1.join();
    let _ = th2.join();
}

#[test]
fn map() {
    let (tx, rx): (mpsc::Sender<usize>, mpsc::Receiver<usize>) = mpsc::channel::<usize>(5);
    let mut rx = rx.map(|x| x * 2);

    let tx = tx.send(10).wait().unwrap();

    let x = rx.for_each(|x| {
        assert_eq!(x, 20);
        Ok(())
    });
}

#[test]
fn accumulator() {
    let (tx, rx): (mpsc::Sender<usize>, mpsc::Receiver<usize>) = mpsc::channel::<usize>(5);
    let mut x = 0;
    let rx = rx.fold(0, move |sum, val| {
        assert_eq!(x, sum);
        x += val;
        if x == 30 {
            Err(())
        } else {
            Ok(sum + val)
        }
    });

    let tx = tx.send(10).wait().unwrap();
    let tx = tx.send(10).wait().unwrap();
    let mut tx = tx.send(10).wait().unwrap();

    let mut core = Core::new().unwrap();
    core.run(rx);
}

#[test]
fn zip() {
    let (tx, rx): (mpsc::Sender<usize>, mpsc::Receiver<usize>) = mpsc::channel::<usize>(5);
    let (tx2, rx2) = mpsc::channel::<()>(5);
    let stream = rx.zip(rx2).for_each(|x| {
        println!("{:?}", x);
        if x.0 == 0 {
            Err(())
        } else {
            Ok(())
        }
    });
    let tx = tx.send(10).wait().unwrap();
    let tx2 = tx2.send(()).wait().unwrap();
    let tx2 = tx2.send(()).wait().unwrap();
    let tx = tx.send(11).wait().unwrap();
    let tx = tx.send(0).wait().unwrap();
    let tx2 = tx2.send(()).wait().unwrap();

    let mut core = Core::new().unwrap();
    core.run(stream);
}
