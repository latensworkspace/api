use mongodb::Database;
use oauth2::basic::BasicClient;

#[derive(Clone)]
pub struct AppState {
    pub db: Database,
    pub oauth_client: BasicClient,
}
