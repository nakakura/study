use diesel;
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;

use std::env;
use std::sync::Mutex;

#[cfg(not(windows))]
pub const EOF: &'static str = "CTRL+D";

#[cfg(windows)]
pub const EOF: &'static str = "CTRL+Z";

lazy_static! {
  static ref CONNECTION: Mutex<MysqlConnection> = {
    Mutex::new(establish_connection())
  };
}

fn db_url() -> String {
    env::var("DATABASE_URL").ok().unwrap_or("mysql://root:mysql@127.0.0.1:3306/mono".to_string())
}

fn establish_connection() -> MysqlConnection {
    MysqlConnection::establish(&db_url())
        .expect(&format!("Error connecting to {}", db_url()))
}

pub fn connection<F, T>(closure: F) -> QueryResult<T>
    where F: Fn(&MysqlConnection) -> QueryResult<T> {
    closure(&*CONNECTION.lock().unwrap())
}

