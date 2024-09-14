#[macro_use]
extern crate rocket;

use rust_api::websocket::handlers::index::index;

#[launch]
fn rocket() -> _ {
    let config = rocket::Config::figment()
        .merge(("address", "127.0.0.1"))
        .merge(("port", 2501));

    rocket::custom(config).mount("/ws", routes![index])
}