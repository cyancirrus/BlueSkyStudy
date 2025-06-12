mod handlers;
mod routes;
mod state;
mod types;

use axum::serve;
use routes::create_router;
use state::AppState;
use std::sync::Arc;

const PORT: &'static str = "0.0.0.0:3000";

#[tokio::main]
async fn main() {
    let logic = Arc::new(AppState::new());
    let router = create_router(logic);
    let listener = tokio::net::TcpListener::bind(PORT).await.unwrap();
    {
        serve(listener, router).await.unwrap();
    }
}
