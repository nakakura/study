use diesel;
use diesel::prelude::*;
use schema::items;
use schema::items::dsl::*;
use chrono::prelude::*;

use database_connection;
use models::items::*;

pub fn insert(title_str: &str, owner_str: &str) {
    let native: NaiveDateTime = Local::now().naive_utc();

    let new_post = NewItem {
        title: title_str,
        owner: owner_str,
        due_date: Some(native),
        borrower: None
    };

    let _ = database_connection::connection(|connection| {
        diesel::insert(&new_post).into(items::table)
            .execute(connection)
    });

}

pub fn display() {
    let x = database_connection::connection(|connection| {
        items.limit(5).load::<Item>(connection)
    });

    println!("{:?}", x.unwrap());
}
