#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]
extern crate crossbeam;

mod pipe;
mod data;

use pipe::*;
use data::*;

fn main() {
    println!("Hello, world!");
    let outer_a = ChainEnum::DataA(DataA::new());
    let outer_b = ChainEnum::DataB(DataB::new());

    let chain = ChainBuilder::new().add_chain(outer_a).add_chain(outer_b).build();
    chain.run();
    chain.display();
}

