#[macro_use]
extern crate rocket;

use tokio::sync::broadcast::channel;
use rust_api::api::handlers::index::index;
use rust_api::api::handlers::product::get_products;
use rust_api::api::handlers::error::{bad_request, unauthorized, default_error};
use rust_api::api::handlers::sse::sse;
use rust_api::api::models::app_state::AppState;

#[launch]
fn rocket() -> _ {
    let config = rocket::Config::figment()
        .merge(("address", "127.0.0.1"))
        .merge(("port", 2500));

    let (tx, _) = channel::<String>(1024);

    rocket::custom(config)
        .manage(AppState { sse_sender: tx })
        .mount("/api", routes![index, get_products, sse])
        .register("/", catchers![bad_request, unauthorized, default_error])
}
