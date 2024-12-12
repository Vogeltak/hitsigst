use std::collections::HashMap;
use std::path::PathBuf;
use std::process::Command;
use std::str::FromStr;

use hitsigst_util::Songs;
use new_string_template::template::Template;
use qrcode::render::svg;
use qrcode::QrCode;

const CARD_TEMPLATE: &str = include_str!("template.typ");

struct SongDataTypst {
    artist: String,
    year: i32,
    title: String,
    color_degree: usize,
    card_deck: String,
    card_nr: usize,
    qr_path: String,
}

impl From<SongDataTypst> for HashMap<&str, String> {
    fn from(value: SongDataTypst) -> Self {
        let mut map = HashMap::new();
        map.insert("artist", value.artist);
        map.insert("year", value.year.to_string());
        map.insert("title", value.title);
        map.insert("color_degree", value.color_degree.to_string());
        map.insert("color_degree_2", (value.color_degree + 90).to_string());
        map.insert("card_deck", value.card_deck);
        map.insert("card_nr", value.card_nr.to_string());
        map.insert("qr_path", value.qr_path);
        map
    }
}

pub(crate) fn build(input: &PathBuf, output: &str) -> anyhow::Result<()> {
    // Deserialize songs from JSON
    let songs_json = std::fs::read_to_string(input)?;
    let songs: Songs = serde_json::from_str(&songs_json)?;

    // Load the template
    let templ = Template::new(CARD_TEMPLATE);

    // Ensure a directory for QR codes exists
    let mut qr_dir = PathBuf::from_str("hitsigst-qr-codes")?;
    if !qr_dir.try_exists()? {
        std::fs::create_dir_all(&qr_dir)?;
    }

    // Path template for individual QR-code files. They can be
    // customized by calling `.with_file_name()` on `qr_dir`.
    qr_dir.push("qr.svg");

    // Get the base of the URL we're encoding in the QR code
    let base_url = std::env::var("QR_URL").expect("should set QR_URL");

    // Generate QR codes and write them to /tmp
    let data_for_typst = songs.songs.into_iter().enumerate().map(|(i, s)| {
        // SAFETY: URLSs should remain within length, since id is always 32 bits
        let code = QrCode::new(format!("{}/song/{}", base_url, s.id)).unwrap();
        let image = code.render::<svg::Color>().build();
        let qr_path = qr_dir.with_file_name(format!("{}.svg", s.id));
        std::fs::write(&qr_path, image).unwrap();

        SongDataTypst {
            artist: s.artist,
            year: s.year,
            title: s.title,
            color_degree: (i * 2) % 360,
            card_deck: s.deck,
            card_nr: i + 1,
            qr_path: qr_path.display().to_string(),
        }
    });

    // Fill typst template, write to /tmp
    let rendered: String = data_for_typst
        .map(|data| templ.render(&data.into()).unwrap())
        .collect();

    let hitsigst_typst = "hitsigst-cards.typ";
    std::fs::write(hitsigst_typst, rendered)?;

    // Execute Typst to compile into a PDF
    Command::new("typst")
        .arg("compile")
        .arg(hitsigst_typst)
        .arg(output)
        .output()?;

    Ok(())
}
