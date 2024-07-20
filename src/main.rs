#[macro_use]
extern crate rocket;

use rust_api::{bad_request, default_error, unauthorized};
use rust_api::handlers::index::index;
use rust_api::handlers::product::get_products;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![get_products])
        .register("/", catchers![bad_request, unauthorized, default_error])
}
