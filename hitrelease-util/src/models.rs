use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Song {
    pub id: uuid::Uuid,
    pub title: String,
    pub artist: String,
    pub year: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Songs {
    pub songs: Vec<Song>,
}
