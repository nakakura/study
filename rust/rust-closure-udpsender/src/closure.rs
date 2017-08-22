use std::thread;
use std::net::{SocketAddrV4, UdpSocket, Ipv4Addr};
use std::sync::Arc;

pub fn closure(port: u16) -> Box<Fn(&'static str) -> ()>{
    let socket: Arc<UdpSocket> = Arc::new(UdpSocket::bind("0.0.0.0:0").ok().expect("bind error"));
    let ip = Ipv4Addr::new(127, 0, 0, 1);

    Box::new(move |message: &'static str|{
        let socket = socket.clone();
        thread::spawn(move || {
            let message = message.as_bytes();
            let to = SocketAddrV4::new(ip, port);
            socket.send_to(message, &to).ok().expect("send error");
        });
    })
}



