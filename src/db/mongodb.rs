use mongodb::{Client, Database, options::ClientOptions};
use dotenvy::dotenv;

use crate::db::state::AppState;

pub async fn establish_connection() -> mongodb::error::Result<Database> {
    dotenv().ok();

    let uri = std::env::var("MONGODB_URI")
        .expect("MONGODB_URI must be set");

    let options = ClientOptions::parse(&uri).await?;
    let client = Client::with_options(options)?;

    let db = client.database("latens");

    println!("✅ Database connected successfully");

    Ok(db)
}
