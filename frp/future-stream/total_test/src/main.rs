extern crate futures;
extern crate tokio_core;
extern crate rand;

use futures::*;
use futures::sync::mpsc;
use tokio_core::reactor::Core;

use std::io::stdin;
use std::thread;

#[derive(Debug, PartialOrd, PartialEq, Clone)]
pub struct SlackPost {
    pub user: String,
    pub channel: String,
    pub command: String,
    pub args: Vec<String>,
}

fn search_db(post: SlackPost) -> Vec<usize> {
    use rand::Rng;
    use std;

    println!("search db");

    let mut rng = rand::thread_rng();
    let x = rng.gen::<usize>();
    if x < std::usize::MAX /2 {
        vec!(x)
    } else {
        vec!(x, rng.gen::<usize>())
    }
}

fn make_post(command: String) -> SlackPost {
    SlackPost {
        user: "user".to_string(),
        channel: "channel".to_string(),
        command: command,
        args: vec!("hoge".to_string()),
    }
}

use std::sync::RwLock;

#[derive(Debug)]
struct Latest {
    pub output: RwLock<Option<Vec<usize>>>,
}

impl Latest {
    pub fn set_latest(&self, message: Vec<usize>) {
        *(self.output.write().unwrap()) = Some(message);
    }

    pub fn erase_latest(&self) {
        *(self.output.write().unwrap()) = None;
    }

    pub fn message(&self, pos: usize) -> usize {
        if let Some(ref x) = *(self.output.read().unwrap()) {
            println!("from {:?}", x);
            if x.len() > pos {
                x[pos]
            } else {
                println!("index out of range");
                0
            }
        } else {
            0
        }
    }

    pub fn flag(&self) -> bool {
        self.output.read().unwrap().is_some()
    }
}

fn set_receiver(receiver: mpsc::Receiver<String>) {
    let _ = thread::spawn(move || {
        let (mut tx_post, rx_post) = mpsc::channel::<SlackPost>(5000);
        let (mut tx_num, rx_num) = mpsc::channel::<usize>(5000);

        let th = thread::spawn(move || {
            let mut core = Core::new().unwrap();

            let x = receiver.map(make_post).fold((tx_post, tx_num), |(s_p, s_n), x| {
                let num = x.command.parse::<usize>();
                if let Ok(val) = num {
                    Ok((s_p, s_n.send(val).wait().unwrap()))
                } else {
                    Ok((s_p.send(x).wait().unwrap(), s_n))
                }
            });
            let _ = core.run(x);
        });

        use std::sync::Arc;
        let latest = Arc::new(Latest{output: RwLock::new(None)});

        let l1 = latest.clone();
        let th_post = thread::spawn(move || {
            let mut core = Core::new().unwrap();
            let x = rx_post.map(search_db).for_each(|x|{
                if x.len() == 1 {
                    l1.erase_latest();
                    println!("#########{}############", x[0]);
                } else {
                    println!("reset & wait");
                    l1.set_latest(x);
                }
                Ok(())
            });
            let _ = core.run(x);
        });

        let l = latest.clone();
        let th_num = thread::spawn(move || {
            let mut core = Core::new().unwrap();
            let x = rx_num.filter(|_x| {
                let flag = l.flag();
                flag
            }).for_each(|x|{
                println!("#############{:?}###########", l.message(x));
                l.erase_latest();
                Ok(())
            });
            let _ = core.run(x);
        });

        let _ = th.join();
        let _ = th_post.join();
        let _ = th_num.join();
    });
}

fn main() {
    let (mut tx, rx) = mpsc::channel::<String>(5000);
    set_receiver(rx);

    loop {
        println!(">mono SUBCOMMAND");
        let mut line = String::new();
        stdin().read_line(&mut line).unwrap();
        let line = line.trim_right(); // Remove the trailing newline
        if line == "exit" {
            return;
        } else {
            tx = tx.send(line.to_string()).wait().unwrap();
        }
    }
}
