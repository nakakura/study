use iron;
use iron::prelude::*;

use std::env;

fn hello_world(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((iron::status::Ok, "Hello World")))
}

fn bind_path() -> String {
    let port: u16 = env::var("PORT").ok().and_then(|p| p.parse().ok()).unwrap_or(8080);
    format!("0.0.0.0:{}", port)
}

pub fn http_start() {
    let http_url = bind_path();
    let chain = Chain::new(hello_world);
    Iron::new(chain).http(http_url).unwrap();
}

