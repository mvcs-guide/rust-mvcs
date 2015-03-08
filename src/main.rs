extern crate iron;
use iron::prelude::*;

extern crate router;
use router::Router;

extern crate mvcs;
use mvcs::controllers;

fn main() {
    let mut router = Router::new();

    router.get("/", controllers::index::index);
    router.post("/users", controllers::user::post);

    Iron::new(router).http("localhost:3000").unwrap();
}
