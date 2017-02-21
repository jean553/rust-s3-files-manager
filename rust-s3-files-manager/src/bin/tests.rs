extern crate hyper;

use hyper::Client;
use hyper::client::response::Response;

/// Returns the response of a GET request on ping URL
fn get_ping(client: &hyper::Client) -> Response {
    return client.get("http://localhost:8000/api/1/ping").send().unwrap();
}

/// Tests that a GET ping request returns 200 status code
fn test_get_ping(client: &hyper::Client) {

    let response = get_ping(&client);
    assert_eq!(
        response.status, 
        hyper::Ok
    );
}

/// Executes every test
fn main() {

    let client = Client::new();
    test_get_ping(&client);
}
