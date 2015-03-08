extern crate iron;

use self::iron::prelude::*;
use self::iron::status;

pub fn index(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "MVCS in Rust.")))
}
