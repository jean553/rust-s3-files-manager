use super::rocket;
use rocket::testing::MockRequest;
use rocket::http::{Status, Method};

fn test_returns_200(uri: &str) {

    let rocket = rocket::ignite().mount(
        "/",
        routes![
            super::ping,
            super::get_file
        ]
    );

    let mut request = MockRequest::new(
        Method::Get,
        uri
    );

    let response = request.dispatch_with(&rocket);

    assert_eq!(
        response.status(),
        Status::Ok
    );
}

#[test]
fn get_ping_returns_200() {

    test_returns_200("/ping");
}

#[test]
fn get_file_returns_200() {

    test_returns_200("/file");
}
