extern crate iron;
use self::iron::prelude::*;
use self::iron::status;

use super::super::services;
use super::super::entities::user::User;

pub fn post(_: &mut Request) -> IronResult<Response> {

    // create a user entity from request
    let mut user = User{ id: 0, name: "Grayson Koonce".to_string() };

    // delegate creation to the service layer
    services::user::create(&mut user);

    // return response
    let response = format!("Created {} ({})", user.name, user.id);

    //let response = "User Created";
    Ok(Response::with((status::Ok, response)))
}
