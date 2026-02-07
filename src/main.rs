mod db;
mod models;
mod routes;

use std::net::SocketAddr;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let state = db::mongodb::establish_connection().await.unwrap();

    let app = routes::routes::create_router(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("\n🚀 Running on http://{}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();
    
    axum::serve(listener, app).await.unwrap();
}
