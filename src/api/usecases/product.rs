use diesel::prelude::*;
use crate::establish_connection;
use crate::models::product::{OptionalProduct, Product};
use crate::schema::products::dsl::*;

pub fn get_products(opt_product: OptionalProduct) -> Vec<Product> {

    let mut query = products.into_boxed();

    if let Some(req_product_id) = opt_product.product_id {
        query = query.filter(product_id.eq(req_product_id));
    }

    // if let Some(product_name) = &opt_product.product_name {
    //     query = query.filter(product_name.eq(product_name));
    // }
    //
    // if let Some(category) = &opt_product.category {
    //     query = query.filter(category.eq(category));
    // }
    //
    // if let Some(price) = opt_product.price {
    //     query = query.filter(products::price.eq(price));
    // }
    //
    // if let Some(stock) = opt_product.stock {
    //     query = query.filter(products::stock.eq(stock));
    // }

    let mut connection = establish_connection();
    let results = query
        .filter(pharmacy_id.eq(opt_product.pharmacy_id))
        .load::<Product>(&mut connection)
        .expect("Error loading products");

    results
}