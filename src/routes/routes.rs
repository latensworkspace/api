use crate::db::state::AppState;
use crate::routes::user;

use axum::Router;
use axum::routing::MethodRouter;

use axum::routing::{get, post};

type AppRoute = (&'static str, &'static str, MethodRouter<AppState>);

pub fn routes() -> Vec<AppRoute> {
    vec![
        ("GET", "/users", get(user::get::get_all_users)),
        ("POST", "/users", post(user::create::handler)),
    ]
}

pub fn apply_routes(
    mut router: Router<AppState>,
    routes: Vec<AppRoute>,
) -> Router<AppState> {
    for (method, path, handler) in routes {
        println!("{} {} loaded successfully", method, path);

        router = router.route(path, handler);
    }

    router
}

pub fn create_router(state: AppState) -> Router {
    let router= Router::new();

    let router = apply_routes(router, routes());

    router.with_state(state)
}
