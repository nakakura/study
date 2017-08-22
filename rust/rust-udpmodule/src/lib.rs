use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex};
use std::net::{SocketAddrV4, UdpSocket, Ipv4Addr};
use std::collections::VecDeque;

#[no_mangle]
pub extern fn create_rust_object() -> Box<RustObject>{
    let socket: Arc<UdpSocket> = Arc::new(UdpSocket::bind("0.0.0.0:0").ok().expect("bind error"));
    let ip = Ipv4Addr::new(127, 0, 0, 1);
    let queue: Arc<Mutex<VecDeque<Vec<u8>>>> = Arc::new(Mutex::new(VecDeque::new()));
    let rust_object = Box::new(RustObject { socket: socket, ip: ip, port: 10000, packet_queue: queue});
    sender_thread(&rust_object);
    rust_object
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


