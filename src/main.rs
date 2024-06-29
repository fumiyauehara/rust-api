#[macro_use] extern crate rocket;

use rust_web::{index, products};

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index]).mount("/", routes![products])
}