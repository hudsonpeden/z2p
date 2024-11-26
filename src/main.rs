use std::net::TcpListener;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error>{
    let configuration = get_configuration()
        .expect("Could not read configuration");
    
    run(TcpListener::bind(format!("127.0.0.1:{}", configuration.port))
        .expect("Failed to bind to port 8080"))?.await
}