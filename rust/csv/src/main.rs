use std::io;
use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;
use std::io::Error;

fn read() -> Result<(), Error> {
    let mut f = File::open("item.csv")?;
    let f = BufReader::new(f);

    for line_result in f.lines() {
        line_result.map(|line|{
         let x: Vec<&str> = line.split(',').filter(|x|{
             *x != ""
         }).collect();
            println!("{:?}", x);
        });
   }
    Ok(())
}

fn main() {
    read();
}
