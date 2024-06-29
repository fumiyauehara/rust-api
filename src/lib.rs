// src/lib.rs
#[macro_use] extern crate rocket;

use rocket::request::FromRequest;
use rocket::serde::json::Json;
use rocket::serde::Serialize;

#[get("/")]
pub async fn index() -> &'static str {
    "Hello, world!"
}

pub struct CustomHeaders {
    authorization: String,
    x_custom_header: String,
}

// enum CustomHeaderError {
//     MissingContentType,
//     MissingAuthorization,
//     MissingXCustomHeader,
// }

#[derive(Serialize)]
pub struct ProductsResponse {
    product_name: String,
    category: String,
    price: u32,
    authorization: String,
    x_custom_header: String,
}

#[rocket::async_trait]
impl <'r>FromRequest<'r> for CustomHeaders {
    type Error = ();

    async fn from_request(request: &'r rocket::Request<'_>) -> rocket::request::Outcome<Self, Self::Error> {
        let authorization = request.headers().get_one("Authorization").unwrap_or_default();
        let x_custom_header = request.headers().get_one("X-Custom-Header").unwrap_or_default();

        rocket::request::Outcome::Success(CustomHeaders {
            authorization: authorization.to_string(),
            x_custom_header: x_custom_header.to_string(),
        })
    }
}

#[derive(FromForm)]
pub struct ProductParams {
    pub product_name: Option<String>,
    pub category: Option<String>,
    pub price: Option<u32>,
}

#[get("/products?<params..>")]
pub async fn products(headers: CustomHeaders, params: ProductParams) -> Json<ProductsResponse>{
    Json(ProductsResponse {
        product_name: params.product_name.unwrap_or_else(|| "N/A".to_string()),
        category: params.category.unwrap_or_else(|| "N/A".to_string()),
        price: params.price.unwrap_or(0),
        authorization: headers.authorization.to_string(),
        x_custom_header: headers.x_custom_header.to_string(),
    })
}
