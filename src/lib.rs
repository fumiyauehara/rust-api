// src/lib.rs
#[macro_use] extern crate rocket;

use rocket::http::Status;
use rocket::Request;
use rocket::request::FromRequest;
use rocket::serde::json::Json;
use rocket::serde::Serialize;


#[get("/")]
pub async fn index() -> &'static str {
    "Hello, world!"
}

// クエリパラメータ関連
#[derive(Serialize)]
pub struct Product {
    product_name: String,
    category: String,
    price: u32,
}

#[derive(FromForm)]
pub struct ProductParams {
    pub product_name: Option<String>,
    pub category: Option<String>,
    pub price: Option<u32>,
}


// ヘッダ関連
pub struct CustomHeaders {
    authorization: String,
    pharmacy_id: i32,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for CustomHeaders {
    type Error = CustomHeaderError;

    async fn from_request(request: &'r rocket::Request<'_>) -> rocket::request::Outcome<Self, Self::Error> {
        let authorization = match request.headers().get_one("Authorization") {
            Some(authorization) => authorization.to_string(),
            None => return rocket::request::Outcome::Error((Status::Unauthorized, CustomHeaderError::MissingAuthorization))
        };

        let pharmacy_id= match request.headers().get_one("pharmacy-id") {
            Some(pharmacy_id) => pharmacy_id.parse().expect("pharmacy-id must be an integer"),
            None => return rocket::request::Outcome::Error((Status::BadRequest, CustomHeaderError::MissingPharmacyId))
        };

        rocket::request::Outcome::Success(CustomHeaders {
            authorization,
            pharmacy_id
        })
    }
}

// エラー関連
#[derive(Debug, Serialize)]
pub enum CustomHeaderError {
    MissingAuthorization,
    MissingPharmacyId,
}

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

#[get("/products?<params..>")]
pub async fn products(headers: CustomHeaders, params: ProductParams) -> Json<Product>{
    Json(Product {
        product_name: params.product_name.unwrap_or_else(|| "N/A".to_string()),
        category: params.category.unwrap_or_else(|| "N/A".to_string()),
        price: params.price.unwrap_or(0)
    })
}
