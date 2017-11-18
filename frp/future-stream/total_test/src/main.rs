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

use std::io;

#[derive(Debug, PartialOrd, PartialEq, Clone)]
struct OutputData {
    pub message_array_opt: Option<Vec<usize>>,
    pub position_opt: Option<usize>,
}

impl OutputData {
    pub fn can_output(&self) -> bool {
        self.message_array_opt.is_some() && self.position_opt.is_some()
    }

    pub fn output(&self) {
        if let (&Some(ref array), &Some(ref position)) = (&self.message_array_opt, &self.position_opt) {
            let x = array.get(*position);
            x.map(|data| {
                println!("###########\n{:?}\n#############", data);
            });
        }
    }

    pub fn is_data(&self) -> bool {
        self.message_array_opt.is_some()
    }

    pub fn feed(&self, data: OutputData) {
        OutputData {
            message_array_opt: self.message_array_opt.clone(),
            position_opt: data.position_opt.clone()
        }.output();
    }
}

impl Future for OutputData {
    type Item = OutputData;
    type Error = io::Error;

    fn poll(&mut self) -> Poll<OutputData, io::Error> {
        let outputdata = OutputData {
            message_array_opt: None,
            position_opt: None,
        };

        Ok(Async::Ready(outputdata))
    }
}

fn db2outputdata(db: Vec<usize>) -> OutputData {
    if db.len() == 1 {
        OutputData {
            message_array_opt: Some(db),
            position_opt: Some(0),
        }
    } else {
        OutputData {
            message_array_opt: Some(db),
            position_opt: None,
        }
    }
}

fn set_receiver(receiver: mpsc::Receiver<String>) {
    let (mut tx_output, rx_output) = mpsc::channel::<OutputData>(5000);
    let (mut tx_slackpost, rx_slackpost) = mpsc::channel::<SlackPost>(5000);

    let th_parse_input_message = thread::spawn(move || {
        let mut core = Core::new().unwrap();

        let x = receiver.map(make_post).fold((tx_output, tx_slackpost), |tx, post| {
            let num = post.command.parse::<usize>();
            if let Ok(val) = num {
                let output = tx.0.send(OutputData {
                    message_array_opt: None,
                    position_opt: Some(val),
                }).wait().unwrap();
                Ok((output, tx.1))
            } else {
                let slackpost = tx.1.send(post).wait().unwrap();
                Ok((tx.0, slackpost))
            }
        });
        let _ = core.run(x);
    });

    let rx_message_array = rx_slackpost.map(search_db).map(db2outputdata);

    let rx_merge = rx_output.select(rx_message_array);
    let th_output = thread::spawn(move || {
        let mut core = Core::new().unwrap();
        let data = OutputData{ message_array_opt: None, position_opt: None };
        let x = rx_merge.fold(data.clone(), |prev, x| {
            if x.can_output() {
                x.output();
                Ok(data.clone())
            } else {
                if x.is_data() {
                    println!("there are multiple target. please input position to specifiy");
                    Ok(x)
                } else if prev.is_data() {
                    prev.feed(x);
                    Ok(data.clone())
                } else {
                    Ok(data.clone())
                }
            }
        });

        let _ = core.run(x);
    });

    let _ = th_parse_input_message.join();
    let _ = th_output.join();
}

fn main() {
    let (mut tx, rx) = mpsc::channel::<String>(5000);
    let th = thread::spawn(move || {
        set_receiver(rx);
    });

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

    let _ = th.join();
}
