extern crate iron;
extern crate router;
extern crate rustc_serialize;

use std::io::Read;
use iron::prelude::*;
use iron::{headers, status};
use iron::modifiers::Header;
use iron::Handler;
use router::Router;
use rustc_serialize::json;

pub struct PostHandler {
    message: String
}

#[derive(RustcEncodable, RustcDecodable)]
struct Greeting {
    msg: String
}

// Ok(Response::with((status::Ok, Header(headers::ContentType::json()), self.message.clone())))

impl Handler for PostHandler {
    fn handle(&self, request: &mut Request) -> IronResult<Response> {
        let mut payload = String::new();
        request.body.read_to_string(&mut payload).unwrap();
        let request: Greeting = json::decode(&payload).unwrap();
        let greeting = Greeting { msg: request.msg };
        let payload = json::encode(&greeting).unwrap();
        Ok(Response::with((status::Ok, payload)))
    }
}

fn main() {
    let post_handler = PostHandler {
        message: "{ \"result\": \"match\" }".to_string()
    };

    let mut router = Router::new();

    router.post("/rust/matching-service", post_handler, "handle_post");

    Iron::new(router).http("localhost:3003").unwrap();
}
