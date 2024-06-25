use rocket::{FromForm, get, launch, routes};

#[get("/")]
async fn index() -> &'static str {
    "Hello, world!"
}

#[derive(FromForm)]
struct ProductParams {
    product_name: Option<String>,
    category: Option<String>,
    price: Option<u32>,
}

#[get("/test?<params..>")]
async fn test(params: ProductParams) -> String {
    format!(
        "Product name: {}, Category: {}, Price: {}",
        params.product_name.unwrap_or_else(|| "N/A".to_string()),
        params.category.unwrap_or_else(|| "N/A".to_string()),
        params.price.map_or("N/A".to_string(), |p| p.to_string())
    )
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index]).mount("/", routes![test])
}