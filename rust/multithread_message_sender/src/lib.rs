extern crate regex;
use regex::Regex;
use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex};
use std::net::{SocketAddrV4, SocketAddr, UdpSocket, Ipv4Addr};
use std::collections::VecDeque;
use std::str;
use std::ffi::CStr;
use std::os::raw::c_char;
use std::os::raw::c_uint;
use std::io::Write;
#[macro_use]
extern crate lazy_static;

#[repr(C)]
pub struct RelayDataTuple{
    pub length: c_uint,
    pub contents: *const u8,
}

lazy_static! {
    static ref RECVDATA: Mutex<Vec<(usize, Vec<u8>)>> = Mutex::new(vec!());
}

#[no_mangle]
pub extern fn create_rust_object(message: *const c_char, port: u16) -> Box<RustObject>{
    let address_src = unsafe { CStr::from_ptr(message).to_str() };
    let address = match address_src{
        Ok(x) => x,
        _ => "127.0.0.1",
    };

    let socket: Arc<UdpSocket> = Arc::new(UdpSocket::bind("0.0.0.0:0").ok().expect("bind error"));
    let re = Regex::new(r"([1-9]?[0-9]*)\.([1-9]?[0-9]*)\.([1-9]?[0-9]*)\.([1-9]?[0-9]*)").unwrap();
    let cap = re.captures(address);
    let addr_tuple = match cap{
        Some(c) => {
            (c.at(1).unwrap().parse::<u8>().unwrap(),
             c.at(2).unwrap().parse::<u8>().unwrap(),
             c.at(3).unwrap().parse::<u8>().unwrap(),
             c.at(4).unwrap().parse::<u8>().unwrap())
        },
        _ => (127, 0, 0, 1),
    };

    let ip = Ipv4Addr::new(addr_tuple.0, addr_tuple.1, addr_tuple.2, addr_tuple.3);
    let queue: Arc<Mutex<VecDeque<Vec<u8>>>> = Arc::new(Mutex::new(VecDeque::new()));
    let rust_object = Box::new(RustObject { socket: socket, ip: ip, port: port, packet_queue: queue});
    sender_thread(&rust_object);
    rust_object
}


#[no_mangle]
pub fn start_listening(port: u16){
    let ip = Ipv4Addr::new(0, 0, 0, 0);
    let listen_addr = SocketAddrV4::new(ip, port);

    let socket = match UdpSocket::bind(listen_addr) {
        Ok(s) => s,
        Err(e) => panic!("couldn't bind socket: {}", e)
    };

    thread::spawn(move || {
        loop {
            let mut buf = [0u8; 2048];
            socket.recv_from(&mut buf).map(|(amt, src)| {
                RECVDATA.lock().unwrap().push((amt, (&buf[0..amt]).to_vec()));
            });
        }
    });
}

#[no_mangle]
pub extern "C" fn get_data(mut to: &mut [u8]) -> usize {
    let ref mut vec = RECVDATA.lock().unwrap();
    if vec.len() > 0 {
        let ref item = vec[0].clone();
        vec.remove(0);
        match to.write(item.1.as_slice()) {
            Ok(x) => x,
            _ => 0,
        }
    } else {
        0
    }
}

fn socket(listen_on: SocketAddr) -> UdpSocket {
    let attempt = UdpSocket::bind(listen_on);
    let mut socket;
    match attempt {
        Ok(sock) => {
            println!("Bound socket to {}", listen_on);
            socket = sock;
        },
        Err(err) => panic!("Could not bind: {}", err)
    }
    socket
}

fn sender_thread(obj: &RustObject){
    let socket = obj.socket.clone();
    let to = SocketAddrV4::new(obj.ip, obj.port);
    let item = obj.packet_queue.clone();//.lock().unwrap();//.push_back(buffer);
    thread::spawn(move || {
        loop{
            let queue: Option<Vec<u8>> = {
                let mut item = item.lock().unwrap();
                item.pop_front()
            };
            match queue{
                Some(x) => {
                    let x = x.as_slice();
                    socket.send_to(x, &to).ok().expect("send error");
                },
                None => {
                    thread::sleep(Duration::from_millis(15));
                },
            }
        }
    });
}

#[no_mangle]
pub extern fn send_message(obj: &RustObject, buffer: *const[u8], length: i32){
    let buffer: &[u8] = unsafe { &*buffer };
    let mut x: Vec<u8> = vec![0; buffer.len()];
    x.clone_from_slice(buffer);
    let mut item = obj.packet_queue.lock().unwrap();
    item.push_back(x);
}

#[repr(C)]
pub struct RustObject {
    socket: Arc<UdpSocket>,
    ip: Ipv4Addr,
    port: u16,
    packet_queue: Arc<Mutex<VecDeque<Vec<u8>>>>,
}


