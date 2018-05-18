extern crate futures;
extern crate futures_fs;
extern crate bytes;
extern crate tokio;
#[macro_use]
extern crate nom;
extern crate either;

use either::*;
use nom::*;
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
    let read = fs.read("/home/nakakura/pingcopy.txt", Default::default());

// default writes options to create a new file
    let mut write = fs.write("/home/nakakura/pingcopy.txt", Default::default());

// block this thread!
// the reading and writing however will happen off-thread

    let (tx, rx) = mpsc::channel::<Bytes>(1500);
    /*
    let mut buf = BytesMut::with_capacity(1024);
    buf.put_u16_be(10u16);
    buf.put(vec!(0, 1, 2, 3, 4, 5, 6, 7, 8));
    write = write.send(buf.freeze()).wait().unwrap();
    */
    /*
    for _x in 0..10000 {
        let mut buf = BytesMut::with_capacity(1024);
        buf.put_u16_be(10u16);
        buf.put(vec!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9));
        write = write.send(buf.freeze()).wait().unwrap();
    }
    */
    let mut buf = BytesMut::with_capacity(999999);
    let r = read.map_err(|_| ()).fold((tx, buf), |mut sum: (mpsc::Sender<Bytes>, BytesMut), x| {
        use nom::IResult::Done;
        sum.1.put(x);
        let r = e(&sum.1);
        let mut buf = BytesMut::with_capacity(999999);
        buf.put(r);

        Ok((sum.0, buf))
    }).map(|x| ());

    use std::thread;


    let th = thread::spawn(|| {
        let rx = rx.for_each(|x| {
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

fn e(i: &[u8]) -> &[u8] {
    let x = ex(i);
    match x {
        Left(l) => {
            e(l.1)
        },
        Right(r) => {
            println!("right {:?}", r);
            r
        }
    }
}

fn ex(i: &[u8]) -> Either<(&[u8], &[u8]), &[u8]>{
    let x = extract(i);
    if x.is_done() {
        let x = x.unwrap().1;
        Left((x.1, x.2))
    } else if x.is_incomplete() {
        Right(i)
    } else {
        println!("err");
        unreachable!();
    }
}

fn extract(i: &[u8]) -> IResult<&[u8], (u16, &[u8], &[u8])> {
    let total_len = i.len();
    do_parse!(i,
                data_len: be_u16
            >>  payload: take!(data_len)
            >>  rest: take!(total_len - data_len as usize - 2)
            >> (
                data_len, payload, rest
            )
        )
}

