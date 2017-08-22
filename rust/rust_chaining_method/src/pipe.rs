use crossbeam;

use std::thread;
use std::time::Duration;
use std::sync::{ Arc, RwLock };

use data::*;

pub type Closure = Fn(i32) + Sync;

pub trait ChainItemTrait {
    fn array(&self) -> Arc<RwLock<Vec<i32>>>;

    fn create_chain_if(&self) -> Box<Closure> {
        let array = self.array();
        Box::new(move |x| {
            array.write().unwrap().push(x)
        })
    }

    fn connect_chain(&mut self, next_chain: &ChainEnum);

    fn run(&self);

    fn display(&self) {
        let array = self.array();
        println!("display inner item");
        for x in &(*array.read().unwrap()) {
            println!("{}", x);
        }
    }
}

pub enum ChainEnum {
    DataA(DataA),
    DataB(DataB)
}

impl ChainEnum {
    fn inner(&self) -> &ChainItemTrait {
        match *self {
            ChainEnum::DataA(ref i) => i,
            ChainEnum::DataB(ref i) => i
        }
    }

    pub fn chain(&self) -> Box<Closure> {
        self.inner().create_chain_if()
    }

    pub fn connect_chain(&mut self, outer: &ChainEnum) {
        match *self {
            ChainEnum::DataA(ref mut i) => i.connect_chain(outer),
            ChainEnum::DataB(ref mut i) => i.connect_chain(outer)
        }
    }

    pub fn run(&self) {
        self.inner().run();
    }

    pub fn display(&self) {
        self.inner().display();
    }
}

pub struct Chain {
    items: Vec<ChainEnum>
}

impl Chain {
    pub fn new(items: Vec<ChainEnum>) -> Self {
        Chain {
            items: items
        }
    }

    pub fn run(&self) {
        crossbeam::scope(|scope| {
            let mut guards = vec!();
            for i in &self.items {
                let handle = scope.spawn(move || {
                    i.run();
                });
                guards.push(handle);
            }

            let handle = scope.spawn(||{
                loop {
                    self.display();
                    thread::sleep(Duration::from_millis(1000));
                }
            });
            guards.push(handle);

            for h in guards {
                h.join();
            }
        });
    }

    pub fn display(&self) {
        for i in &self.items {
            i.display();
        }
    }
}


pub struct ChainBuilder {
    items: Vec<ChainEnum>
}

impl ChainBuilder {
    pub fn new() -> Self {
        ChainBuilder{
            items: vec!()
        }
    }

    pub fn add_chain(mut self, next: ChainEnum) -> ChainBuilder {
        let _ = self.items.last_mut().map(|mut item| item.connect_chain(&next));
        self.items.push(next);
        self
    }

    pub fn build(self) -> Chain {
        Chain::new(self.items)
    }
}

