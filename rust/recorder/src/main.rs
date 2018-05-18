#![feature(drain_filter)]
extern crate futures;
extern crate futures_fs;
extern crate tokio;
extern crate tokio_io;
extern crate bytes;
#[macro_use]
extern crate nom;
extern crate either;

use either::*;
use nom::*;
use futures::*;
use futures::sync::mpsc;
use futures_fs::FsPool;
use tokio::net::{UdpSocket, UdpFramed};
use tokio_io::codec::BytesCodec;
use tokio::executor::current_thread::CurrentThread;
use bytes::{ Bytes, BytesMut, BufMut };

use std::net::SocketAddr;

fn main() {
    let addr: SocketAddr = "0.0.0.0:10000".parse().unwrap();
    let (_send_udp_tx, send_udp_rx) = mpsc::channel::<(Bytes, SocketAddr)>(5000);
    let (recv_udp_tx, recv_udp_rx) = mpsc::channel::<(Bytes, SocketAddr)>(5000);

    let mut current_thread = CurrentThread::new();

    socket(&addr, send_udp_rx, recv_udp_tx, &mut current_thread);
    load(&mut current_thread);
    //save(recv_udp_rx, &mut current_thread);
    let _x = current_thread.run();
}

fn load(current_thread: &mut CurrentThread){
    let fs = FsPool::default();

    let read = fs.read("/home/nakakura/p.txt", Default::default());
    let mut buf = BytesMut::with_capacity(1600);
    let r = read.map_err(|_| ()).fold(buf, |mut sum: BytesMut, x| {
        use nom::IResult::Done;
        sum.put(x);
        let r = e(&sum, vec!());
        let mut buf = BytesMut::with_capacity(1600);
        buf.put(r.0);
        println!("{:?}", r.1);

        Ok(buf)
    }).map(|x| ());
    current_thread.spawn(r.map_err(|_| ()));
}

fn save(recv_udp_rx: mpsc::Receiver<(Bytes, SocketAddr)>, current_thread: &mut CurrentThread){
    let fs = FsPool::default();
    let write = fs.write("/home/nakakura/p.txt", Default::default());

    let r = recv_udp_rx.fold(write, |w, data| {
        let mut buf = BytesMut::with_capacity(1600);
        let len = data.0.len() as u16;
        buf.put_u16_be(len);
        buf.put(data.0);
        Ok(w.send(buf.freeze()).wait().unwrap())
    }).map(|_| ()).map_err(|e| println!("error = {:?}", e));
    current_thread.spawn(r);
}

fn socket(addr: &SocketAddr,
          send_data_stream: mpsc::Receiver<(Bytes, SocketAddr)>,
          recv_data_sink: mpsc::Sender<(Bytes, SocketAddr)>,
          current_thread: &mut CurrentThread) {
    let sock = UdpSocket::bind(&addr).unwrap();
    let (udp_sink, udp_stream) = UdpFramed::new(sock, BytesCodec::new()).split();
    let receiver = udp_stream.map_err(|_| ()).fold(recv_data_sink, |sink, x| {
        println!("recv {:?}", x);
        Ok(sink.send((x.0.freeze(), x.1)).wait().unwrap())
    });

    let sender = udp_sink.sink_map_err(|e| {
        eprintln!("err {:?}", e);
    }).send_all(send_data_stream);

    current_thread.spawn({
        sender.join(receiver)
            .map(|_| ())
            .map_err(|e| println!("error = {:?}", e))
    });
}

fn e<'a>(i: &'a [u8], mut v: Vec<&'a [u8]>) -> (&'a [u8], Vec<&'a[u8]>) {
    let x = ex(i);
    match x {
        Left(l) => {
            v.push(l.0);
            e(l.1, v)
        },
        Right(r) => {
            println!("right {:?}", r);
            (r, v)
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
