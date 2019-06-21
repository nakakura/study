use std::io::BufReader;

use futures::future::*;
use futures::sync::mpsc;
use futures::sink::Sink;
use futures::stream::Stream;

mod error;

fn main() {
    let (tx, rx) = mpsc::channel::<String>(10);

    let pf = lazy(|| {
        let lines = tokio::io::lines(BufReader::new(tokio::io::stdin()));
        let lines = lines.then(|line|{
            match line {
                Ok(ref line) if line == "exit" => Ok(None),
                Ok(line) => Ok(Some(line)),
                Err(e) => Err(e),
            }
        }).take_while(|x| ok(x.is_some()))
            .map(Option::unwrap)
            .map_err(|e| panic!(e))
            .fold(tx, |tx, e| {
                println!("{}", e);
                let tx = tx.send(e).wait().unwrap();
                Ok(tx)
            });
        tokio::spawn(lines.map(|_| ()).map_err(|_| ()))
    });

    let rx = rx.for_each(|r|{
        println!("recv {}", r);
        Ok(())
    });

    let lines = rx.join(pf);
    tokio::run(lines.map(|_| ()).map_err(|_| ()));
}
