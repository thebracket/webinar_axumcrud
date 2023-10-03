use axum::response::Html;
use axum::routing::get;
use axum::Router;

pub fn view_service() -> Router {
    Router::new().route("/", get(index_page))
}

const INDEX_PAGE: &str = include_str!("index.html");

async fn index_page() -> Html<&'static str> {
    Html(INDEX_PAGE)
}
