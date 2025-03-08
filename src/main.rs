use axum::{response::Html, routing::any, Router};
use tokio::{net::TcpListener, sync::broadcast};
use std::{include_str, sync::Arc};   
// post locally, get only public

struct AppState {
    _tx: broadcast::Sender<String>,
}

#[tokio::main]
async fn main() {
    let authors_routes = Router::new()
        .route("/", any(authors))
        .route("/arsen_markaryan", any(arsen))
        .route("/mikhail_svetov", any(svetov));

    let (tx, _) = broadcast::channel(100);

    let app_state = Arc::new(AppState { _tx: tx });

    let app = Router::new()
        .route("/", any(root))
        .nest("/authors", authors_routes)
        .fallback(fallback)
        .with_state(app_state);

    let listener = TcpListener::bind("127.0.0.1:7979").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> Html<&'static str> {
    Html(include_str!("../htmls/root.html"))
}

async fn authors() -> Html<&'static str> {
    Html(include_str!("../htmls/authors.html"))
} 

async fn arsen() -> Html<&'static str> {
    Html(include_str!("../htmls/arsen.html"))
}

async fn svetov() -> Html<&'static str> {
    Html(include_str!("../htmls/svetov.html"))
}

async fn fallback() -> Html<&'static str> {
    Html(include_str!("../htmls/fallback.html"))
}