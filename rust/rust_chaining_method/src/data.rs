use std::sync::{ Arc, RwLock };
use std::thread;
use std::time::Duration;

use pipe::*;

pub struct DataA {
    items: Arc<RwLock<Vec<i32>>>,
    next_chain: Option<Box<Closure>>
}

impl DataA {
    pub fn new() -> Self{
        DataA {
            items: Arc::new(RwLock::new(vec!())),
            next_chain: None
        }
    }
}

impl ChainItemTrait for DataA {
    fn array(&self) -> Arc<RwLock<Vec<i32>>> {
        self.items.clone()
    }

    fn connect_chain(&mut self, next_chain: &ChainEnum){
        self.next_chain = Some(next_chain.chain());
    }

    fn run(&self) {
        loop {
            if let Some(ref x) = self.next_chain {
                x(0)
            }
            thread::sleep(Duration::from_millis(100));
        }
    }
}

pub struct DataB {
    items: Arc<RwLock<Vec<i32>>>,
    next_chain: Option<Box<Closure>>
}

impl DataB {
    pub fn new() -> Self{
        DataB {
            items: Arc::new(RwLock::new(vec!())),
            next_chain: None
        }
    }
}

impl ChainItemTrait for DataB {
    fn array(&self) -> Arc<RwLock<Vec<i32>>> {
        self.items.clone()
    }

    fn connect_chain(&mut self, next_chain: &ChainEnum){
        self.next_chain = Some(next_chain.chain());
    }

    fn run(&self) {
        if let Some(ref x) = self.next_chain {
            x(0)
        }
    }
}