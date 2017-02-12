use super::rocket;
use rocket::testing::MockRequest;
use rocket::http::{Status, Method};

#[test]
fn get_ping_returns_200() {

    let rocket = rocket::ignite().mount(
        "/",
        routes![super::ping]
    );

    let mut request = MockRequest::new(
        Method::Get,
        "/ping"
    );

    let response = request.dispatch_with(&rocket);

    assert_eq!(
        response.status(),
        Status::Ok
    );
}
