use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex};

pub struct Container{
	pub packet: Arc<Mutex<Vec<u8>>>,
}

fn main() {
	println!("Hello, world!");
	let container: Container = Container{ packet: Arc::new(Mutex::new(vec!())) };
	generate(&container, 0);
	generate(&container, 100);
	display(&container);
	thread::sleep(Duration::from_millis(40000));
}

pub fn generate(container: &Container, prefix: u8) {
	let client = container.packet.clone();
	thread::spawn(move || {
		let mut x = 1;
		loop {
			thread::sleep(Duration::from_millis(400));
			client.lock().unwrap().push(prefix + x);
			x += 1;
		}
	});
}

pub fn display(container: &Container) {
	let client = container.packet.clone();
	thread::spawn(move || {
		loop {
			let len = client.lock().unwrap().len();
			if len > 0 {
				let item = client.lock().unwrap().pop();
				println!("item: {}", item.unwrap());
			} else{
				thread::sleep(Duration::from_millis(10));
			}
		}
	});
}
