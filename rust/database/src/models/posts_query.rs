use diesel;
use diesel::prelude::*;
use schema::posts;
use schema::posts::dsl::*;

use database_connection;
use models::posts::*;

pub fn insert(title_str: &str, body_str: &str) {
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

pub fn display() {
    let x = database_connection::connection(|connection| {
        posts.filter(published.eq(false))
            .limit(5)
            .load::<Post>(connection)
    });

    for post in x.unwrap() {
        println!("{}", post.title);

    }
}
