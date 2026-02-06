mod routes;

use axum::Router;
use axum::routing::get;
use std::net::SocketAddr;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(routes::status::get_status));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Running on http://{}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}