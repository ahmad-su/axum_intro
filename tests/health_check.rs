mod test_suite;
use std::net::TcpListener;

use crate::test_suite::{async_runner, new_client, send, Method, StatusCode};
use axum_intro::server::{app, run_app};

#[test]
fn health_check_ok() {
    //We bind to a random port so each test will never interupt each other
    //(each test will have their own dedicated server running)
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();

    //get the address
    let addr = listener.local_addr().unwrap();

    //init async runtime to run our server and run our http client
    let rt = async_runner();

    //init Http client to send package to our server
    let client = new_client();

    //The endpoint defined in our router that we want to test
    let endpoint = "/health-check";

    //final url
    let uri = format!("http://{}:{}{}", addr.ip(), addr.port(), endpoint);

    //Now run the server instance
    rt.spawn(run_app(app(), listener));

    //Hit the endpoint, and get the response
    let response = rt.block_on(send(&client, Method::Get(uri), None)).unwrap();

    //And finally assert the result
    //The health-check endpoint should reply with 200OK with empty body (content-length: 0)
    //
    //Curently, we depend on reqwest::Response here (using response.status() which is reqwest specific)
    //It is because we hasn't implement our own Response. But this is good so far.
    //We might only need to modify the assert section if we later use other lib.
    //
    //But to make it better:
    //TODO: make & use new function to map reqwest::Response (we do that in test_suite module)
    //so we can craft our fully independent test

    //Thus, at the end it should be similar to this:
    // let response = .....
    // let result = map_result(response);
    // assert_eq!(result.status, StatusCode::OK);
    // assert_eq!(result.content_length, 0);

    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(response.content_length(), Some(0));
}
