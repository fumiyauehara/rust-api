use rocket::FromForm;
use rocket::serde::Serialize;

#[derive(Serialize)]
pub struct Product {
    pub product_name: String,
    pub category: String,
    pub price: u32,
}

#[derive(FromForm)]
pub struct ProductParams {
    pub product_name: Option<String>,
    pub category: Option<String>,
    pub price: Option<u32>,
}
