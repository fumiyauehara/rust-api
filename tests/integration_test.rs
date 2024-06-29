use rocket::local::blocking::Client;
use rocket::http::{ContentType, Header, Status};
use rocket::routes;
use rocket::serde::json::{serde_json, Value};
use rust_web::products;

#[test]
fn test_products_endpoint() {
    // Rocketアプリケーションのインスタンスを作成
    let rocket = rocket::build().mount("/", routes![products]);
    let client = Client::tracked(rocket).expect("valid rocket instance");

    // すべてのパラメータがある場合のテスト
    let response = client.get("/products?product_name=Apple&category=Fruit&price=100").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));

    let body = response.into_string().unwrap();
    let json: Value = serde_json::from_str(&body).unwrap();

    assert_eq!(json["product_name"], "Apple");
    assert_eq!(json["category"], "Fruit");
    assert_eq!(json["price"], 100);
    assert_eq!(json["authorization"], "");
    assert_eq!(json["x_custom_header"], "");


    let response = client.get("/products?product_name=Banana&category=Fruit").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));

    let body = response.into_string().unwrap();
    let json: Value = serde_json::from_str(&body).unwrap();

    assert_eq!(json["product_name"], "Banana");
    assert_eq!(json["category"], "Fruit");
    assert_eq!(json["price"], 0);
    assert_eq!(json["authorization"], "");
    assert_eq!(json["x_custom_header"], "");

    let response = client.get("/products")
        .header(Header::new("content-type", "application/json"))
        .header(Header::new("authorization", "Bearer token"))
        .header(Header::new("x-custom-header", "fumiya-uehara"))
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));

    let body = response.into_string().unwrap();
    let json: Value = serde_json::from_str(&body).unwrap();

    assert_eq!(json["product_name"], "N/A");
    assert_eq!(json["category"], "N/A");
    assert_eq!(json["price"], 0);
    assert_eq!(json["authorization"], "Bearer token");
    assert_eq!(json["x_custom_header"], "fumiya-uehara");
}