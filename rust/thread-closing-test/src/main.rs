extern crate crossbeam;

#[macro_use]
extern crate chan;
#[macro_use]
extern crate lazy_static;
extern crate chan_signal;
extern crate mio;

use chan_signal::Signal;
use mio::{Events, Poll, Ready, PollOpt, Token};
use mio::tcp::TcpStream;

use std::thread;
use std::sync::RwLock;
use std::net::{TcpListener, SocketAddr};

lazy_static! {
    static ref STOPPED: RwLock<bool> = RwLock::new(false);
}

fn main() {
    let signal = chan_signal::notify(&[Signal::INT, Signal::TERM]);
    let (sdone, rdone): (chan::Sender<Signal>, chan::Receiver<Signal>) = chan::sync(0);

    crossbeam::scope(|scope| {
        let h = scope.spawn(|| {
            thread::sleep(std::time::Duration::from_millis(40));
        });

        let j = scope.spawn(|| {
            let poller = mio::Poll::new().unwrap();
            let token_id = 0;
            let addr: SocketAddr = "127.0.0.1:7100".parse().unwrap();
            let mut conn = mio::udp::UdpSocket::bind(&addr).unwrap();
            poller.register(&conn, mio::Token(token_id), mio::Ready::readable(), mio::PollOpt::edge()).unwrap();

            let mut buf = [0u8; 1024];
            let mut events = mio::Events::with_capacity(1024);
            while !(*STOPPED.read().unwrap()) {
                poller.poll(&mut events, Some(std::time::Duration::from_millis(100))).unwrap();
                for ev in events.iter() {
                    match ev.token() {
                        mio::Token(id) if id == token_id => {
                            match conn.recv_from(&mut buf[0..]) {
                                Ok(Some((0, _))) => {
                                    println!("read: closed");
                                    return Ok(());
                                },
                                Ok(Some((n, _))) => {
                                    println!("recv len {}", n);
                                },
                                Ok(None) => {
                                    println!("none");
                                },
                                Err(e) => {
                                    return Err(format!("read: {}", e));
                                }
                            }
                        }
                        mio::Token(_) => (),
                    }
                }
            }
            Ok(())
        });
        let k: crossbeam::ScopedJoinHandle<std::result::Result<(), std::io::Error>> = scope.spawn(|| {
            while !(*STOPPED.read().unwrap()) {
                thread::sleep(std::time::Duration::from_millis(300));
                println!("300ms");
            }
            Ok(())
        });
        let z: crossbeam::ScopedJoinHandle<std::result::Result<(), std::io::Error>> = scope.spawn(|| {
            chan_select! {
                signal.recv() -> signal => {
                    println!("received signal: {:?}", signal);
                    (*STOPPED.write().unwrap()) = true;
                },
                rdone.recv() => {
                    println!("Program completed normally.");
                    (*STOPPED.write().unwrap()) = true;
                }
            }
            Ok(())
        });
        j.join();
        h.join();
        k.join();
        z.join();
    });

    println!("finished");
}

