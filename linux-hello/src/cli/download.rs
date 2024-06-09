use super::Runnable;
use crate::config;
use clap::Args;
use color_eyre::{eyre::Ok, Result};
use reqwest::IntoUrl;
use std::{
    env::temp_dir,
    fs,
    io::Write,
    path::{Path, PathBuf},
    process::Command,
};

#[derive(Debug, Args)]
pub(crate) struct DownloadArgs {}

impl Runnable for DownloadArgs {
    fn run(&self) -> Result<()> {
        let config = crate::config::read();

        let path = download_file(&config.models.shape_predictor.url)?;
        let path = extract_bz2(path.clone())?;
        std::fs::copy(path, &config.models.shape_predictor.filepath).expect("Unable to copy file");

        let path = download_file(&config.models.face_recognition.url)?;
        let path = extract_bz2(path.clone())?;
        std::fs::copy(path, &config.models.face_recognition.filepath).expect("Unable to copy file");

        Ok(())
    }
}

fn download_file<U: IntoUrl>(url: U) -> Result<PathBuf> {
    let url = url.into_url()?;

    let filename = url
        .path_segments()
        .and_then(|segments| segments.last())
        .expect("Can't find file name from url");
    let filepath = temp_dir().join(filename);
    let mut out = fs::File::create(filepath.clone())?;

    let response = reqwest::blocking::get(url.clone())?;

    if response.status().is_success() {
        let content = response.bytes()?;
        out.write_all(&content)?;
    } else {
        log::error!("Failed to download file: {}", response.status());
    }

    log::info!("Downloaded {} into {}", &url, &filepath.display());

    Ok(filepath)
}

fn extract_bz2<P: AsRef<Path>>(filepath: P) -> Result<PathBuf> {
    let filepath: &Path = filepath.as_ref();

    log::info!("Extracting {}", filepath.display(),);

    let output = Command::new("bzip2")
        .arg("--decompress")
        .arg("--force")
        .arg(filepath)
        .output()
        .expect("Failed to execute command");

    log::debug!("Output {:?}", String::from_utf8(output.stdout));
    log::debug!("Err {:?}", String::from_utf8(output.stderr));

    Ok(filepath.to_path_buf().with_extension(""))
}
