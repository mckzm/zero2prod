use std::net::TcpListener;
use zero2prod::startup::run;

pub fn spawn_app() -> String {
    let socket_spec = "127.0.0.1:0";
    let listener = TcpListener::bind(socket_spec).expect("Failed to bind to socket");
    let port = listener.local_addr().unwrap().port();
    let server = run(listener).expect("Failed to bind to given socket");
    let _ = tokio::spawn(server);
    format!("http://{}:{}", socket_spec.trim_end_matches(":0"), port)
}
