use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::post, Json, Router};
use headless_chrome::{Browser, LaunchOptions};
use renderer::Renderer;
use serde::Deserialize;
use std::sync::Arc;

mod renderer;

#[tokio::main]
async fn main() {
    let browser = Browser::new(LaunchOptions::default_builder().build().unwrap()).unwrap();
    let renderer = Renderer::new(browser);

    let app = Router::new()
        .route("/api/generate", post(generate))
        .with_state(renderer.into());

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Deserialize)]
struct RawHtmlRequest {
    html: String,
}

#[derive(Deserialize)]
struct UrlRequest {
    url: String,
}

#[derive(Deserialize)]
#[serde(untagged)]
enum GeneratePdfRequest {
    RawHtml(RawHtmlRequest),
    Url(UrlRequest),
}

async fn generate(
    State(renderer): State<Arc<Renderer>>,
    Json(payload): Json<GeneratePdfRequest>,
) -> Result<Vec<u8>, impl IntoResponse> {
    match payload {
        GeneratePdfRequest::RawHtml(payload) => renderer.html_to_bytes(&payload.html),
        GeneratePdfRequest::Url(payload) => renderer.url_to_bytes(&payload.url),
    }
    .map_err(internal_server_error)
}

fn internal_server_error<E: AsRef<dyn std::error::Error>>(error: E) -> impl IntoResponse {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        error.as_ref().to_string(),
    )
}
