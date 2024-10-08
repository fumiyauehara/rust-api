use rocket::http::{ContentType, Header, Status};
use rocket::local::blocking::{Client, LocalRequest};
use rocket::{catchers, routes};
use rust_api::{bad_request, default_error, unauthorized};
use rust_api::handlers::product::get_products;

#[test]
fn test_products_endpoint_ok() {
    // Rocketアプリケーションのインスタンスを作成
    let rocket = rocket::build().mount("/", routes![get_products]);
    let client = Client::tracked(rocket).expect("valid rocket instance");

    let test_cases = vec![
        (
            "/products",
            200,
            r#"{"product_name":"N/A","category":"N/A","price":0}"#,
        ),
        (
            "/products?product_name=Apple&category=Fruit&price=100",
            200,
            r#"{"product_name":"Apple","category":"Fruit","price":100}"#,
        ),
        (
            "/products?product_name=Banana&category=Fruit",
            200,
            r#"{"product_name":"Banana","category":"Fruit","price":0}"#,
        ),
        (
            "/products",
            200,
            r#"{"product_name":"N/A","category":"N/A","price":0}"#,
        ),
    ];

    for (url, status, expected) in test_cases {
        let response = client
            .get(url)
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
    let rocket = rocket::build()
        .mount("/", routes![get_products])
        .register("/", catchers![bad_request, unauthorized, default_error]);

    let client = Client::tracked(rocket).expect("valid rocket instance");

    let test_cases: Vec<(&str, Box<dyn Fn(LocalRequest) -> LocalRequest>, u16, &str)> = vec![
        (
            "/products",
            Box::new(|c: LocalRequest| c.header(Header::new("pharmacy-id", "112"))),
            401,
            r#"{"error":"Unauthorized"}"#,
        ),
        (
            "/products",
            Box::new(|c: LocalRequest| c.header(Header::new("authorization", "Bearer: Token"))),
            400,
            r#"{"error":"Bad Request"}"#,
        ),
    ];

    for (url, add_header, status, expected) in test_cases {
        let response = add_header(client.get(url)).dispatch();

        assert_eq!(response.status(), Status::from_code(status).unwrap());
        // assert_eq!(response.content_type(), Some(ContentType::JSON));

        let actual = response.into_string().unwrap();
        assert_eq!(actual, expected)
    }
}
