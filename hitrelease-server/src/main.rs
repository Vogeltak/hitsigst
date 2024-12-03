use axum::{extract::State, http::StatusCode, routing::get, Router};
use hitrelease_util::Store;
use std::{str::FromStr, sync::Arc};
use uuid::Uuid;

mod song;

// Template for the home/about page
#[derive(askama::Template)]
#[template(path = "index.html")]
struct AboutTemplate {}

// Template for the player page
#[derive(askama::Template)]
#[template(path = "player.html")]
struct PlayerTemplate {
    song_url: String,
}

// Application state
struct AppState {
    endpoint: String,
    song_cache: Store,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    let endpoint_url = std::env::var("S3_ENDPOINT").expect("should set the endpoint URL");

    // Create app state
    let state = Arc::new(AppState {
        endpoint: endpoint_url,
        song_cache: Store::default(),
    });

    // Build router
    let app = Router::new()
        .route("/", get(show_about))
        .route("/song/:id", get(show_player))
        .with_state(state);

    // Start server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    tracing::info!("listening on {}", listener.local_addr()?);
    axum::serve(listener, app).await?;

    Ok(())
}

// Handler for the home/about page
async fn show_about(State(_): State<Arc<AppState>>) -> AboutTemplate {
    AboutTemplate {}
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
