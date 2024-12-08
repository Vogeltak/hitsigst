use std::{
    fs::File,
    io::Write,
    path::PathBuf,
    process::{Command, Output},
};

use futures::stream::{self, StreamExt};
use hitrelease_util::{Song, Songs};
use indicatif::{ParallelProgressIterator, ProgressBar};
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

#[expect(dead_code)]
fn download_songs_async(songs: &[(Song, String)], output_dir: &String) -> anyhow::Result<()> {
    println!("downloading songs...");

    let rt = tokio::runtime::Runtime::new()?;
    rt.block_on(async {
        let pb = ProgressBar::new(songs.len() as u64);

        let download_tasks = songs.iter().map(|(song, url)| {
            let pb = pb.clone();
            let output_dir = output_dir.clone();
            let song_id = song.id;
            let url = url.clone();

            tokio::spawn(async move {
                pb.inc(1);
                tokio::process::Command::new("yt-dlp")
                    .arg("--no-playlist")
                    .arg("--extract-audio")
                    .arg("--audio-format")
                    .arg("mp3")
                    .arg("--output")
                    .arg(format!("{output_dir}/{}.%(ext)s", song_id))
                    .arg(url)
                    .output()
                    .await
            })
        });

        let results: Vec<_> = stream::iter(download_tasks)
            .buffer_unordered(16)
            .collect()
            .await;
        let outputs = results
            .into_iter()
            .filter_map(|res| res.ok())
            .collect::<Result<Vec<_>, _>>();
        let Ok(outputs) = outputs else {
            println!("failed while invoking yt-dlp: {}", outputs.err().unwrap());
            return;
        };

        // Count yt-dlp errors
        let ytdlp_errors: Vec<&Output> =
            outputs.iter().filter(|out| !out.status.success()).collect();

        // Count those that were already downloaded and didn't fail validation
        let skipped = outputs
            .iter()
            .filter(|out| {
                String::from_utf8_lossy(&out.stdout).contains("has already been downloaded")
            })
            .filter(|out| out.status.success())
            .count();

        let downloaded = songs.len() - ytdlp_errors.len() - skipped;

        if downloaded > 0 {
            println!("downloaded {downloaded} songs to {output_dir}/");
        }
        if skipped > 0 {
            println!("skipped {skipped} songs because they were already downloaded");
        }
        if !ytdlp_errors.is_empty() {
            println!(
                "failed to download {} songs: {:#?}",
                ytdlp_errors.len(),
                ytdlp_errors
                    .iter()
                    .map(|e| String::from_utf8_lossy(&e.stderr))
                    .collect::<Vec<_>>()
            );
        }
    });

    Ok(())
}

fn download_songs(songs: &[(Song, String)], output_dir: &String) -> anyhow::Result<()> {
    println!("downloading songs...");
    let outputs = songs
        .par_iter()
        .progress_count(songs.len() as u64)
        .map(|(song, url)| {
            Command::new("yt-dlp")
                .arg("--no-playlist")
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
    let ytdlp_errors: Vec<&Output> = outputs.iter().filter(|out| !out.status.success()).collect();

    // Count those that were already downloaded and didn't fail validation
    let skipped = outputs
        .iter()
        .filter(|out| String::from_utf8_lossy(&out.stdout).contains("has already been downloaded"))
        .filter(|out| out.status.success())
        .count();

    let downloaded = songs.len() - ytdlp_errors.len() - skipped;

    if downloaded > 0 {
        println!("downloaded {downloaded} songs to {output_dir}/");
    }
    if skipped > 0 {
        println!("skipped {skipped} songs because they were already downloaded");
    }
    if !ytdlp_errors.is_empty() {
        println!(
            "failed to download {} songs: {:#?}",
            ytdlp_errors.len(),
            ytdlp_errors
                .iter()
                .map(|e| String::from_utf8_lossy(&e.stderr))
                .collect::<Vec<_>>()
        );
    }

    Ok(())
}
