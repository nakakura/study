extern crate futures;
extern crate futures_fs;
extern crate bytes;
extern crate tokio;

use futures::*;
use futures::sync::mpsc;
use futures_fs::FsPool;
use futures_fs::ReadOptions;
use bytes::{Bytes, BytesMut, BufMut};
use tokio::executor::current_thread::CurrentThread;

fn main() {
    let mut current_thread = CurrentThread::new();

    let fs = FsPool::default();

// our source file
    let read = fs.read("~/pingcopy.txt", Default::default());

// default writes options to create a new file
    let mut write = fs.write("~/pingcopy.txt", Default::default());

// block this thread!
// the reading and writing however will happen off-thread

    let (tx, rx) = mpsc::channel::<Bytes>(1500);
    for _x in 0..100000 {
        let mut buf = BytesMut::with_capacity(1024);
        buf.put_u16_be(10u16);
        buf.put(vec!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9));
        write = write.send(buf.freeze()).wait().unwrap();
    }
    let mut buf = BytesMut::with_capacity(999999);
    let r = read.map_err(|_| ()).fold((tx, buf), |mut sum: (mpsc::Sender<Bytes>, BytesMut), x| {
        let tx = sum.0.send(x).wait().unwrap();
        Ok((tx, sum.1))
    }).map(|x| ());

    use std::thread;


    let th = thread::spawn(|| {
        let rx = rx.for_each(|x| {
            println!("{:?}", x);
            Ok(())
        });
        tokio::run(rx);
    });


    current_thread.spawn(r.map_err(|_| ()));
    current_thread.run();
    /*
    read.forward(write).wait()
        .expect("IO error piping foo.txt to out.txt");
        */

    let _ = th.join();
}
