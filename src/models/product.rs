use diesel::prelude::*;
use rocket::FromForm;
use serde::Serialize;
use crate::models::custom_decimal::CustomDecimal;

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = crate::schema::products)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct Product {
    pub product_id: u32,
    pub product_name: String,
    pub category: String,
    pub price: CustomDecimal,
    pub stock: u32,
    pub pharmacy_id: u32,
}

pub struct OptionalProduct {
    pub product_id: Option<u32>,
    pub product_name: Option<String>,
    pub category: Option<String>,
    pub price: Option<u32>,
    pub stock: Option<u32>,
    pub pharmacy_id: u32,
}

#[derive(FromForm)]
pub struct OptionalProductOmitPharmacyId {
    pub product_id: Option<u32>,
    pub product_name: Option<String>,
    pub category: Option<String>,
    pub price: Option<u32>,
    pub stock: Option<u32>,
}
