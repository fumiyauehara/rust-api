// src/lib.rs
#[macro_use] extern crate rocket;

#[get("/")]
pub async fn index() -> &'static str {
    "Hello, world!"
}

#[derive(FromForm)]
pub struct ProductParams {
    pub product_name: Option<String>,
    pub category: Option<String>,
    pub price: Option<u32>,
}

#[get("/products?<params..>")]
pub async fn products(params: ProductParams) -> String {
    format!(
        "Product name: {}, Category: {}, Price: {}",
        params.product_name.unwrap_or_else(|| "N/A".to_string()),
        params.category.unwrap_or_else(|| "N/A".to_string()),
        params.price.map_or("N/A".to_string(), |p| p.to_string())
    )
}
