use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::post, Json, Router};
use headless_chrome::types::PrintToPdfOptions;
use headless_chrome::{Browser, LaunchOptions};
use renderer::Renderer;
use serde::Deserialize;
use std::sync::Arc;
use tower_http::trace::TraceLayer;

mod renderer;

const PORT: u16 = 3000;
const HOST: &str = "0.0.0.0";

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let browser = Browser::new(LaunchOptions::default_builder().build().unwrap())
        .expect("cannot launch a browser instance");

    let renderer = Renderer::new(browser);

    let app = Router::new()
        .route("/api/generate", post(generate))
        .with_state(renderer.into())
        .layer(TraceLayer::new_for_http());

    tracing::info!("listening on {HOST}:{PORT}");

    axum::Server::bind(&format!("{HOST}:{PORT}").parse().unwrap())
        .serve(app.into_make_service())
        .await
        .expect("server error");
}

#[derive(Deserialize)]
struct RawHtmlRequest {
    html: String,
    options: Option<PrintToPdfOptions>,
}

#[derive(Deserialize)]
struct UrlRequest {
    url: String,
    options: Option<PrintToPdfOptions>,
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
        GeneratePdfRequest::RawHtml(payload) => {
            renderer.html_to_bytes(&payload.html, payload.options)
        }
        GeneratePdfRequest::Url(payload) => renderer.url_to_bytes(&payload.url, payload.options),
    }
    .map_err(internal_server_error)
}

fn internal_server_error<E: AsRef<dyn std::error::Error>>(error: E) -> impl IntoResponse {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        error.as_ref().to_string(),
    )
}
