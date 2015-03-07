extern crate mvcs;

use mvcs::services;
use mvcs::entities::user::User;

fn main() {

    // create a user entity from request
    let mut user = User{ id: 0, name: "Grayson Koonce".to_string() };

    // delegate creation to the service layer
    services::user::create(&mut user);

    println!("Created {} ({})", user.name, user.id);
}
