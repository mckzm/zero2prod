use std::net::TcpListener;
use zero2prod::startup::run;

pub fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to socket");
    let port = listener.local_addr().unwrap().port();
    let server = run(listener).expect("Failed to bind to given socket");
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}
