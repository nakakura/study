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

use models::*;

use std::io::{stdin, Read};

pub fn create_post(title_str: &str, body_str: &str) {
    use schema::posts;
    use schema::posts::dsl::*;

    let new_post = NewPost {
        title: title_str,
        body: body_str,
    };

    database_connection::connection(|connection| {
        diesel::insert(&new_post).into(posts::table)
            .execute(connection)
            .expect("Error saving new post");
        let results = posts.filter(published.eq(false))
            .limit(5)
            .load::<Post>(connection)
            .expect("Error loading posts");

        println!("Displaying {} posts", results.len());
        for post in results {
            println!("{}", post.title);
            println!("----------\n");
            println!("{}", post.body);
        }
    });

    //posts::table.order(posts::id.desc()).first(database_connection::connection()).unwrap()
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

    let _ = create_post(title2, &message);
    //println!("\nSaved draft {} with id {}", title2, post.id);

    //let post = database_connection::create_post(title2, posts);

    //http::http_start();
}

#[test]
fn it_works() {
    use std::io::Write;
    let uri = db_url();
}
