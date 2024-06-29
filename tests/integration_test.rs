use rocket::local::blocking::Client;
use rocket::http::Status;
use rocket::routes;
use rust_web::products;

#[test]
fn test_products_endpoint() {
    // Rocketアプリケーションのインスタンスを作成
    let rocket = rocket::build().mount("/", routes![products]);
    let client = Client::tracked(rocket).expect("valid rocket instance");

    // すべてのパラメータがある場合のテスト
    let response = client.get("/products?product_name=Apple&category=Fruit&price=100").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.into_string().unwrap(), "Product name: Apple, Category: Fruit, Price: 100");

    // 一部のパラメータが欠けている場合のテスト
    let response = client.get("/products?product_name=Banana&category=Fruit").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.into_string().unwrap(), "Product name: Banana, Category: Fruit, Price: N/A");

    // すべてのパラメータが欠けている場合のテスト
    let response = client.get("/products").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.into_string().unwrap(), "Product name: N/A, Category: N/A, Price: N/A");
}