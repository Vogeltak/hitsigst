use std::{collections::HashMap, sync::Arc};

use anyhow::{anyhow, Result};
use askama::Template;
use askama_axum::IntoResponse;
use axum::{
    extract::{Multipart, State},
    response::Html,
};
use bytes::Bytes;
use uuid::Uuid;

use crate::AppState;

// Template for the upload page
#[derive(askama::Template)]
#[template(path = "upload.html")]
struct UploadTemplate {}

// Handler for the upload page
pub(crate) async fn show_upload_page() -> impl IntoResponse {
    let template = UploadTemplate {};

    Html(template.render().unwrap())
}

#[derive(Debug)]
struct SongUpload {
    title: String,
    artist: String,
    year: i32,
    song: Bytes,
}

impl SongUpload {
    async fn from_multipart(mut multipart: Multipart) -> Result<Self> {
        let mut fields = HashMap::new();
        let mut song_data = None;

        // Collect all fields
        while let Some(field) = multipart.next_field().await? {
            let name = field.name().ok_or(anyhow!("unnamed field"))?.to_string();

            // Handle file field separately
            if name == "song" {
                song_data = Some(field.bytes().await?);
            } else {
                // For text fields, collect as strings
                let value = String::from_utf8_lossy(&field.bytes().await?).to_string();
                fields.insert(name, value);
            }
        }

        // Extract and validate required fields
        let title = fields.remove("title").ok_or(anyhow!("missing title"))?;
        let artist = fields.remove("artist").ok_or(anyhow!("missing artist"))?;
        let year_str = fields.remove("year").ok_or(anyhow!("missing year"))?;
        let year = year_str.parse::<i32>()?;
        let song = song_data.ok_or(anyhow!("missing song file"))?;

        Ok(SongUpload {
            title,
            artist,
            year,
            song,
        })
    }
}

// Handler for adding new songs via the web UI.
pub(crate) async fn handle_upload(
    State(state): State<Arc<AppState>>,
    multipart: Multipart,
) -> impl IntoResponse {
    // Generate unique ID for the song
    let song_id = Uuid::new_v4().to_string();

    // Process the uploaded file
    let song = SongUpload::from_multipart(multipart).await.unwrap();

    // Upload to S3
    // state
    //     .s3_client
    //     .put_object()
    //     .bucket(&state.bucket_name)
    //     .key(&song_id)
    //     .body(data.into())
    //     .content_type(content_type)
    //     .send()
    //     .await
    //     .unwrap();

    // Generate QR code
    // let code = QrCode::new(format!("/song/{}", song_id)).unwrap();
    // let image = code.render::<image::Luma<u8>>().build();
    // let qr_path = format!("static/qr/{}.png", song_id);
    // image.save(&qr_path).unwrap();

    // Save to database
    sqlx::query!(
        "INSERT INTO songs (id, title, artist, year) VALUES (?, ?, ?, ?)",
        song_id,
        song.title,
        song.artist,
        song.year,
    )
    .execute(&state.db)
    .await
    .unwrap();

    // Return the song ID
    Html(format!(
        "Upload successful! Your song ID is: {}. <br>QR Code: <img src='/static/qr/{}.png'>",
        song_id, song_id
    ))
}
