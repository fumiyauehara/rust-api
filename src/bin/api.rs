#[macro_use]
extern crate rocket;

use rust_api::api::handlers::index::index;
use rust_api::api::handlers::product::get_products;
use rust_api::api::handlers::error::{bad_request, unauthorized, default_error};

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![get_products])
        .register("/", catchers![bad_request, unauthorized, default_error])
}
