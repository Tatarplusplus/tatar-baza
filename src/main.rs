use axum::{Router, routing::any, response::Html};
use tokio::{net::TcpListener, fs};    
// post locally, get only public

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", any(root)).route("/arsen_markaryan", any(arsen)).route("/mikhail_svetov", any(svetov));

    let listener = TcpListener::bind("127.0.0.1:7979").await.unwrap();

    axum::serve(listener,app).await.unwrap();
}

async fn root() -> Html<&'static str> {
    let path = "htmls/main.html";
    let file: &'static str = fs::read_to_string(path).await.unwrap().leak();
    Html(file)
} 

async fn arsen() -> Html<&'static str> {
    let path = "htmls/arsen.html";
    let file: &'static str = fs::read_to_string(path).await.unwrap().leak(); 
    Html(file)
}

async fn svetov() -> Html<&'static str> {
    let path = "htmls/svetov.html";
    let file: &'static str = fs::read_to_string(path).await.unwrap().leak();
    Html(file)
}