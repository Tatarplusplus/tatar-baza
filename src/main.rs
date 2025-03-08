use axum::{response::Html, routing::any, Router};
use tokio::{net::TcpListener, fs};   
use std::io::Error; 
// post locally, get only public

#[tokio::main]
async fn main() -> Result<(), Error> {
    let root = root().await?;
    let arsen = arsen().await?;
    let svetov = svetov().await?;
    let fallback = fallback().await?;

    let authors_routes = Router::new()
        .route("/", any(root))
        .route("/arsen_markaryan", any(arsen))
        .route("/mikhail_svetov", any(svetov));


    let app = Router::new()
        .nest("/authors", authors_routes)
        .fallback(fallback);

    let listener = TcpListener::bind("127.0.0.1:7979").await?;

    axum::serve(listener, app).await?;

    Ok(())
}

async fn root() -> Result<Html<String>, Error> {
    let path = "htmls/authors.html";
    let file = fs::read_to_string(path).await?;
    
    Ok(Html(file))
} 

async fn arsen() -> Result<Html<String>, Error> {
    let path = "htmls/arsen.html";
    let file= fs::read_to_string(path).await?; 

    Ok(Html(file))
}

async fn svetov() -> Result<Html<String>, Error> {
    let path = "htmls/svetov.html";
    let file= fs::read_to_string(path).await?;

    Ok(Html(file))
}

async fn fallback() -> Result<Html<String>, Error> {
    let path = "htmls/fallback.html";
    let file= fs::read_to_string(path).await?;

    Ok(Html(file))
}