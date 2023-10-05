pub mod server {
    use std::net::TcpListener;

    use axum::{http::Request, routing::get, Router, Server};

    //I make this function to separate between route and other logic
    //Every route will be written here
    pub fn app() -> Router {
        Router::new().route(
            "/health-check",
            get(|req: Request<_>| async move {
                let req_method = req.method();
                println!("--> {:<8}{}", format!("{}", req_method), req.uri());
            }),
        )
    }

    //This function allows us to run the server anywhere we want
    //Whether in the main function or inside the test function
    //We only need to nest this function inside an async runtime to make the server running
    //The server will live as long as the runtime live.
    pub async fn run_app(router: Router, listener: TcpListener) {
        Server::from_tcp(listener)
            .unwrap()
            .serve(router.into_make_service())
            .await
            .expect("Can not run server");
    }
}
