use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Song {
    pub id: u32,
    pub title: String,
    pub artist: String,
    pub year: i32,
    pub deck: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Songs {
    pub songs: Vec<Song>,
}
