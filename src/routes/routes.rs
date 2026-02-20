use crate::db::state::AppState;
use crate::middlewares::{admin::admin, jwt::jwt};
use crate::routes::user;

use axum::routing::{MethodRouter, Route};
use axum::{Router, middleware};

use axum::routing::{get, post};

pub enum RouteGuard {
    Public,
    Jwt,
    Admin,
}

type AppRoute = (&'static str, &'static str, MethodRouter<AppState>);

pub fn public_routes() -> Vec<AppRoute> {
    vec![("POST", "/users", post(user::create::handler))]
}

pub fn protected_routes() -> Vec<AppRoute> {
    vec![("GET", "/users/@me", get(user::get::handler))]
}

pub fn admin_routes() -> Vec<AppRoute> {
    vec![("GET", "/users/:id", get(user::get::handler))]
}

pub fn apply_routes(
    mut router: Router<AppState>,
    routes: Vec<AppRoute>,
    guard: RouteGuard,
) -> Router<AppState> {
    for (method, path, handler) in routes {
        println!("Route ({}): {}", method, path);

        let route = Router::new().route(path, handler);

        let route = match guard {
            RouteGuard::Public => route,

            RouteGuard::Jwt => route.route_layer(middleware::from_fn(jwt)),

            RouteGuard::Admin => route
                .route_layer(middleware::from_fn(admin))
                .route_layer(middleware::from_fn(jwt)),
        };

        router = router.merge(route)
    }

    router
}

pub fn create_router(state: AppState) -> Router {
    let router = Router::new();

    let router = apply_routes(router, public_routes(), RouteGuard::Public);
    let router = apply_routes(router, protected_routes(), RouteGuard::Jwt);
    let router = apply_routes(router, admin_routes(), RouteGuard::Admin);

    router.with_state(state)
}
