#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;
extern crate iron;
extern crate env_logger;
extern crate regex;
extern crate serde;
extern crate serde_json;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate lazy_static;
mod models;
mod schema;
mod http;
mod database_connection;

use diesel::prelude::*;
use schema::posts;
use schema::posts::dsl::*;

use std::io::{stdin, Read};

use models::*;

fn insert(title_str: &str, body_str: &str) {
    let new_post = NewPost {
        title: title_str,
        body: body_str,
    };

    let x = database_connection::connection(|connection| {
        diesel::insert(&new_post).into(posts::table)
            .execute(connection)
    });

    println!("Displaying {:?} posts", x);
}

fn display() {
    let x = database_connection::connection(|connection| {
        posts.filter(published.eq(false))
            .limit(5)
            .load::<Post>(connection)
    });

    for post in x.unwrap() {
        println!("{}", post.title);

    }
}

fn main() {
    env_logger::init().unwrap();

    let mut title2 = String::new();
    let mut message = String::new();

    println!("What would you like your title to be?");
    stdin().read_line(&mut title2).unwrap();
    let title2 = title2.trim_right(); // Remove the trailing newline

    println!("\nOk! Let's write {} (Press {} when finished)\n", title2, database_connection::EOF);
    stdin().read_to_string(&mut message).unwrap();

    let _ = insert(title2, &message);
    display();
    //println!("\nSaved draft {} with id {}", title2, post.id);

    //let post = database_connection::create_post(title2, posts);

    //http::http_start();
}

#[test]
fn it_works() {
    use std::io::Write;
    let uri = db_url();
}
