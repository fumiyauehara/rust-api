use rocket::get;
use crate::api::models::custom_header::CustomHeaders;

#[get("/")]
pub async fn index(_headers: CustomHeaders) -> &'static str {
    "Hello, world!"
}
