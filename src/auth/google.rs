use dotenvy::dotenv;
use oauth2::{ClientId, ClientSecret, basic::BasicClient};

use crate::errors::error::ApiError;

pub async fn oauth_client() -> Result<BasicClient, ApiError> {
    dotenv().ok();

    let id = ClientId::new(std::env::var("GOOGLE_CLIENT_ID").expect("Missing GOOGLE_CLIENT_ID"));
    let secret = ClientSecret::new(std::env::var("GOOGLE_CLIENT_SECRET").expect("Missing GOOGLE_CLIENT_SECRET"));

    let oauth_client = BasicClient::new(id)
        .set_client_secret(secret);

    Ok(oauth_client)
}
    