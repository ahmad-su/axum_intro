This Project is created to try and get familiar with axum.

run server: ```cargo run``` then go to http://localhost:8080/health-check using your browser
or use curl ```curl -I -X GET localhost:8080/health-check```

run integration test (e2e testing): ```cargo test```

the integration test will not interupt or get interupted by the main thread 
running the server in localhost:8080
so you can use 2 terminals executing cargo run and cargo test at the same time
