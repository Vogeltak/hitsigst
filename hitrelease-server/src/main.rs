use askama::Template;
use aws_config::BehaviorVersion;
use aws_sdk_s3::{config::Region, presigning, Client as S3Client};
use axum::{
    extract::State,
    handler::Handler,
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::{get, post},
    Router,
};
use image;
use qrcode::{render::svg, QrCode};
use sqlx::SqlitePool;
use std::{fs::File, io::Write, str::FromStr, sync::Arc};
use uuid::Uuid;

mod song;
mod upload;

// Template for the player page
#[derive(askama::Template)]
#[template(path = "player.html")]
struct PlayerTemplate {
    song_url: String,
}

// Application state
struct AppState {
    db: SqlitePool,
    s3_client: S3Client,
    bucket_name: String,
    // TODO: add tx handle to S3 uploading actor
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Initialize SQLite database
    let db = SqlitePool::connect("sqlite:hitrelease.db?mode=rwc").await?;
    sqlx::migrate!("./migrations").run(&db).await?;

    let endpoint_url = std::env::var("S3_ENDPOINT").ok();

    // Initialize S3 client
    let mut config_loader =
        aws_config::defaults(BehaviorVersion::latest()).region(Region::new("auto"));

    if let Some(endpoint) = endpoint_url {
        config_loader = config_loader.endpoint_url(endpoint);
    }

    let config = config_loader.load().await;
    let s3_client = S3Client::new(&config);
    let bucket_name = std::env::var("S3_BUCKET").expect("S3_BUCKET must be set");

    // Create app state
    let state = Arc::new(AppState {
        db,
        s3_client,
        bucket_name,
    });

    // Build router
    let app = Router::new()
        .route("/", get(upload::show_upload_page))
        .route("/upload", post(upload::handle_upload))
        .route("/song/:id", get(show_player))
        .with_state(state);

    // Start server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    tracing::info!("listening on {}", listener.local_addr()?);
    axum::serve(listener, app).await?;

    Ok(())
}

// Handler for the player page
async fn show_player(
    State(_state): State<Arc<AppState>>,
    axum::extract::Path(song_id): axum::extract::Path<String>,
) -> Result<PlayerTemplate, StatusCode> {
    let Ok(song_uuid) = Uuid::from_str(&song_id) else {
        return Err(StatusCode::BAD_REQUEST);
    };

    Ok(PlayerTemplate {
        song_url: format!("http://cdn.hitrelease.nl/{song_uuid}.mp3"),
    })
}
