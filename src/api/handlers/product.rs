use rocket::{get, State};
use rocket::serde::json::{Json};
use crate::api::models::app_state::AppState;
use crate::api::models::custom_header::CustomHeaders;
use crate::models::product::{Product, OptionalProduct, OptionalProductOmitPharmacyId};

#[get("/products?<params..>")]
pub async fn get_products(headers: CustomHeaders, params: OptionalProductOmitPharmacyId, state: &State<AppState>) -> Json<Vec<Product>> {
    let pharmacy_id = headers.pharmacy_id;
    let opt_product = OptionalProduct {
        product_id: params.product_id,
        product_name: params.product_name,
        category: params.category,
        price: params.price,
        stock: params.stock,
        pharmacy_id,
    };

    state.sse_sender.send("success".to_string()).unwrap();

    Json(crate::api::usecases::product::get_products(opt_product))
}
