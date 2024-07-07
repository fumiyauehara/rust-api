#[macro_use]
extern crate rocket;

use rust_api::{bad_request, default_error, index, products, unauthorized};

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![products])
        .register("/", catchers![bad_request, unauthorized, default_error])
}
