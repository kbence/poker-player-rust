extern crate iron;
extern crate player;

use iron::prelude::*;
use iron::status;
use iron::method::Method;

use player::Player;

fn handler(req: &mut Request) -> IronResult<Response> {
    if req.method == Method::Post {
        Ok(Response::with((status::Ok, Player::new().version())))
    } else {
        Ok(Response::with((status::MethodNotAllowed, "Method not allowed!\n")))
    }
}

fn main() {
    Iron::new(Chain::new(handler)).http("localhost:1234").unwrap();
}
