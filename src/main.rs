use std::net::TcpListener;
use zero2prod::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error>{
    run(TcpListener::bind("127.0.0.1:8080")
        .expect("Failed to bind to port 8080"))?.await
}