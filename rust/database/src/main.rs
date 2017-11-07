#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;
extern crate iron;
extern crate env_logger;
extern crate regex;
extern crate serde;
extern crate serde_json;
#[macro_use] extern crate serde_derive;
mod models;
mod schema;
mod http;

use self::models::*;

use regex::Regex;
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;

use std::io::{stdin, Read};
use std::env;

#[cfg(not(windows))]
const EOF: &'static str = "CTRL+D";

#[cfg(windows)]
const EOF: &'static str = "CTRL+Z";

fn db_url() -> String {
    env::var("DATABASE_URL").ok().unwrap_or("mysql://root:mysql@127.0.0.1:3306/mono".to_string())
}

pub fn establish_connection() -> MysqlConnection {
    MysqlConnection::establish(&db_url())
        .expect(&format!("Error connecting to {}", db_url()))
}

pub fn create_post(conn: &MysqlConnection, title: &str, body: &str) -> Post {
    use schema::posts;

    let new_post = NewPost {
        title: title,
        body: body,
    };

    diesel::insert(&new_post).into(posts::table)
        .execute(conn)
        .expect("Error saving new post");

    posts::table.order(posts::id.desc()).first(conn).unwrap()
}

fn main() {
    use self::schema::posts::dsl::*;


    env_logger::init().unwrap();
    println!("{}", db_url());

    let connection = establish_connection();

    let mut title2 = String::new();
    let mut message = String::new();

    println!("What would you like your title to be?");
    stdin().read_line(&mut title2).unwrap();
    let title2 = title2.trim_right(); // Remove the trailing newline

    println!("\nOk! Let's write {} (Press {} when finished)\n", title2, EOF);
    stdin().read_to_string(&mut message).unwrap();

    let post = create_post(&connection, title2, &message);
    println!("\nSaved draft {} with id {}", title2, post.id);

    let results = posts.filter(published.eq(false))
        .limit(5)
        .load::<Post>(&connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{}", post.title);
        println!("----------\n");
        println!("{}", post.body);
    }
    http::http_start();
}

#[test]
fn it_works() {
    use std::io::Write;
    let uri = db_url();
}
