use rocket::get;
use rocket::serde::json::Json;
use crate::models::custom_header::CustomHeaders;
use crate::models::product::{Product, ProductParams};

#[get("/products?<params..>")]
pub async fn get_products(_headers: CustomHeaders, params: ProductParams) -> Json<Product> {
    Json(Product {
        product_name: params.product_name.unwrap_or_else(|| "N/A".to_string()),
        category: params.category.unwrap_or_else(|| "N/A".to_string()),
        price: params.price.unwrap_or(0),
    })
}
