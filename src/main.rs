use zero2prod::startup::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let socket_spec = "127.0.0.1:8080";
    let listener = std::net::TcpListener::bind(socket_spec).expect("Failed to bind to socket");
    run(listener)?.await
}
