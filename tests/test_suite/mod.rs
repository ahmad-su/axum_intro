#![allow(unused)]
use tokio::runtime::Runtime;

pub use reqwest::StatusCode;

pub enum Method {
    Get(String),
    Post(String),
    Delete(String),
    Put(String),
}

//I intent to make the test functions to not rely to any dependency as much as possible
//including the test runtime and the http client used for testing.
//The reason is that so the test functions will not likely to change if the server
//need to change the framework
//
//Only this section that needs to adapt to any change for testing

//Here we use tokio, and if we decide to use other async runner,
//we only need to modify this function.
pub fn async_runner() -> Runtime {
    Runtime::new().unwrap()
}

//I use reqwest here even if there is httpc_test that has api specialized for testing.
//It's simply because httpc_test has poor documentation at the time this is written
pub fn new_client() -> reqwest::Client {
    reqwest::Client::new()
}

//Well this is Incomplete. Only Get method is working here
//We should implement and adapt requests that contain request body.
//Let's update it later when we encounter the problem
pub async fn send(
    client: &reqwest::Client,
    method: Method,
    _req_body: Option<String>,
) -> Result<reqwest::Response, reqwest::Error> {
    match method {
        Method::Get(uri) => client.get(uri).send().await,
        Method::Post(uri) => client.post(uri).send().await,
        Method::Put(uri) => client.put(uri).send().await,
        Method::Delete(uri) => client.delete(uri).send().await,
    }
}
