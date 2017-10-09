extern crate futures;

use futures::*;
use futures::prelude::*;
use futures::executor;
use futures::future::{err, ok};
use futures::stream::{empty, iter_ok, poll_fn, Peekable};
use futures::sync::oneshot;
use futures::sync::mpsc;

use std::sync::Arc;
use std::thread;
use std::time::Duration;

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
fn (){
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
