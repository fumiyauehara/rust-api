use rocket::{catch, Request};
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::serde::Serialize;

pub mod handlers;
pub mod models;


#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    error: String,
}

#[catch(401)]
pub fn unauthorized(status: Status, _request: &Request) -> Json<ErrorResponse> {
    Json(ErrorResponse {
        error: status.reason().unwrap_or("Unauthorized").to_string(),
    })
}

#[catch(400)]
pub fn bad_request(status: Status, _request: &Request) -> Json<ErrorResponse> {
    Json(ErrorResponse {
        error: status.reason().unwrap_or("Bad Request").to_string(),
    })
}

#[catch(default)]
pub fn default_error(status: Status, _request: &Request) -> Json<ErrorResponse> {
    Json(ErrorResponse {
        error: status.reason().unwrap_or("Unknown Error").to_string(),
    })
}
