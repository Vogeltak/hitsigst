use std::{fs::File, io::Write, path::PathBuf};

use hitrelease_util::{Song, Songs};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
struct SongRecord {
    title: String,
    artist: String,
    year: i32,
    url: String,
}

pub(crate) fn start(input: &PathBuf, output: &String) -> anyhow::Result<()> {
    let mut reader = csv::Reader::from_path(input)?;
    let mut songs: Vec<SongRecord> = vec![];
    for song in reader.deserialize() {
        songs.push(song?)
    }

    // TODO: download audio and upload to object storage

    let songs: Vec<_> = songs
        .into_iter()
        .map(|s| Song {
            id: Uuid::new_v4(),
            title: s.title,
            artist: s.artist,
            year: s.year,
        })
        .collect();

    println!("processed {} songs", songs.len());

    let songs = Songs { songs };

    let mut file = File::create(output)?;
    file.write_all(serde_json::to_string_pretty(&songs)?.as_bytes())?;

    println!("written song data for Hitrelease to {output}");

    Ok(())
}
