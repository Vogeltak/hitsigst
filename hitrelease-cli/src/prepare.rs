use std::{fs::File, io::Write, path::PathBuf, process::Command};

use hitrelease_util::{Song, Songs};
use indicatif::ParallelProgressIterator;
use rayon::prelude::*;
use serde::Deserialize;
use twox_hash::XxHash32;

#[derive(Debug, Deserialize)]
struct SongRecord {
    title: String,
    artist: String,
    year: i32,
    url: String,
}

pub(crate) fn start(input: &PathBuf, output: &String, download_dir: &String) -> anyhow::Result<()> {
    let mut reader = csv::Reader::from_path(input)?;
    let mut songs: Vec<SongRecord> = vec![];
    for song in reader.deserialize() {
        songs.push(song?)
    }

    let seed = 17;

    let songs: Vec<_> = songs
        .iter()
        .map(|s| {
            (
                Song {
                    id: XxHash32::oneshot(seed, s.url.as_bytes()),
                    title: s.title.clone(),
                    artist: s.artist.clone(),
                    year: s.year,
                },
                s.url.clone(),
            )
        })
        .collect();

    download_songs(&songs, download_dir)?;

    let songs = Songs {
        songs: songs.into_iter().map(|(s, _)| s).collect(),
    };

    let mut file = File::create(output)?;
    file.write_all(serde_json::to_string(&songs)?.as_bytes())?;

    println!("written song data for Hitrelease to {output}");

    Ok(())
}

fn download_songs(songs: &[(Song, String)], output_dir: &String) -> anyhow::Result<()> {
    println!("downloading songs...");
    let output = songs
        .par_iter()
        .progress_count(songs.len() as u64)
        .map(|(song, url)| {
            Command::new("yt-dlp")
                .arg("--extract-audio")
                .arg("--audio-format")
                .arg("mp3")
                .arg("--output")
                .arg(format!("{output_dir}/{}.%(ext)s", song.id))
                .arg(url)
                .output()
        })
        .collect::<Result<Vec<_>, _>>()?;

    // Count yt-dlp errors
    let ytdlp_errors = output.iter().filter(|out| !out.status.success()).count();

    // Count those that were already downloaded and didn't fail validation
    let skipped = output
        .iter()
        .filter(|out| String::from_utf8_lossy(&out.stdout).contains("has already been downloaded"))
        .filter(|out| out.status.success())
        .count();

    let downloaded = songs.len() - ytdlp_errors - skipped;

    if downloaded > 0 {
        println!(
            "downloaded {} songs to {output_dir}/",
            songs.len() - ytdlp_errors - skipped
        );
    }
    if skipped > 0 {
        println!("skipped {skipped} songs because they were already downloaded");
    }
    if ytdlp_errors > 0 {
        println!("failed to download {ytdlp_errors} songs")
    }

    Ok(())
}
