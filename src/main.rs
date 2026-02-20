mod auth;
mod db;
mod errors;
mod models;
mod routes;
mod services;
mod middlewares;

use std::net::SocketAddr;
use tokio::net::TcpListener;

use crate::db::state::AppState;

#[tokio::main]
async fn main() {
    let oauth_client = auth::google::oauth_client().await.unwrap();
    let db = db::mongodb::establish_connection().await.unwrap();

    let state = AppState { db, oauth_client };

    let app = routes::routes::create_router(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("\n🚀 Running on http://{}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
