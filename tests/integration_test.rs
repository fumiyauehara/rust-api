use rocket::local::blocking::Client;
use rocket::http::{ContentType, Header, Status};
use rocket::routes;
use rust_web::products;

#[test]
fn test_products_endpoint_ok() {
    // Rocketアプリケーションのインスタンスを作成
    let rocket = rocket::build().mount("/", routes![products]);
    let client = Client::tracked(rocket).expect("valid rocket instance");

    let test_cases = vec![
        ("/products", 200, r#"{"product_name":"N/A","category":"N/A","price":0}"#),
        ("/products?product_name=Apple&category=Fruit&price=100", 200, r#"{"product_name":"Apple","category":"Fruit","price":100}"#),
        ("/products?product_name=Banana&category=Fruit", 200, r#"{"product_name":"Banana","category":"Fruit","price":0}"#),
        ("/products", 200, r#"{"product_name":"N/A","category":"N/A","price":0}"#),
    ];

    for (url, status, expected) in test_cases {
        let response = client.get(url)
            .header(Header::new("authorization", "Bear Token"))
            .header(Header::new("pharmacy-id", "112"))
            .dispatch();

        assert_eq!(response.status(), Status::from_code(status).unwrap());
        assert_eq!(response.content_type(), Some(ContentType::JSON));

        let actual = response.into_string().unwrap();
        assert_eq!(actual, expected)
    }
}

#[test]
fn test_products_endpoint_error() {
    let rocket = rocket::build().mount("/", routes![products]);
    let client = Client::tracked(rocket).expect("valid rocket instance");

    let test_cases = vec![
        ("/products", 400, r#"{"error":"MissingAuthorization"}"#),
        ("/products", 400, r#"{"error":"MissingPharmacyId"}"#),
    ];

    for (url, status, expected) in test_cases {
        let response = client.get(url).dispatch();

        assert_eq!(response.status(), Status::from_code(status).unwrap());
        assert_eq!(response.content_type(), Some(ContentType::JSON));

        let actual = response.into_string().unwrap();
        assert_eq!(actual, expected)
    }
}