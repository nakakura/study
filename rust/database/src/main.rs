#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;
extern crate iron;
extern crate chrono;
extern crate env_logger;
extern crate regex;
extern crate serde;
extern crate serde_json;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate lazy_static;
mod models;
mod schema;
//mod http;
mod database_connection;

use std::io::{stdin, Read};

use models::items_query;
use models::posts_query;


fn main() {
    env_logger::init().unwrap();

    let mut title = String::new();
    let mut message = String::new();

    println!("What would you like your title to be?");
    stdin().read_line(&mut title).unwrap();
    let title = title.trim_right(); // Remove the trailing newline

    println!("\nOk! Let's write {} (Press {} when finished)\n", title, database_connection::EOF);
    stdin().read_to_string(&mut message).unwrap();

    let _ = posts_query::insert(title, &message);
    posts_query::display();
    let _ = items_query::insert(title, &message);
    items_query::display();

    //http::http_start();
}

