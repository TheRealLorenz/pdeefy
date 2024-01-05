use axum::{extract::State, routing::post, Json, Router};
use axum_extra::extract::WithRejection;
use headless_chrome::types::PrintToPdfOptions;
use headless_chrome::LaunchOptions;
use serde::Deserialize;
use std::sync::Arc;
use tower_http::trace::TraceLayer;

mod error;
mod renderer;

const DEFAULT_PORT: u16 = 3000;
const HOST: &str = "0.0.0.0";

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let port = std::env::var("PORT")
        .ok()
        .and_then(|port| port.parse::<u16>().ok())
        .unwrap_or_else(|| {
            tracing::warn!("Invalid PORT env variable, using {DEFAULT_PORT}");

            DEFAULT_PORT
        });

    let renderer =
        renderer::SingleInstance::new(LaunchOptions::default_builder()).expect("renderer");

    let app = Router::new()
        .route("/api/generate", post(generate))
        .with_state(renderer.into())
        .layer(TraceLayer::new_for_http());

    tracing::info!("listening on {HOST}:{port}");

    axum::Server::bind(&format!("{HOST}:{port}").parse().unwrap())
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
    State(renderer): State<Arc<renderer::SingleInstance>>,
    WithRejection(Json(payload), _): WithRejection<Json<GeneratePdfRequest>, error::Api>,
) -> Result<Vec<u8>, error::Api> {
    let bytes = match payload {
        GeneratePdfRequest::RawHtml(payload) => {
            renderer.html_to_bytes(&payload.html, payload.options)?
        }
        GeneratePdfRequest::Url(payload) => renderer.url_to_bytes(&payload.url, payload.options)?,
    };

    Ok(bytes)
}
