use std::{
    fs,
    io::Write,
    path::{Path, PathBuf},
    process::Command,
};

use clap::Args;
use reqwest::IntoUrl;

use crate::config::GLOBAL_CONFIG;

use super::Runnable;
use color_eyre::{eyre::Ok, Result};

#[derive(Debug, Args)]
pub(crate) struct DownloadArgs {}

fn download_file<U: IntoUrl, P: AsRef<Path>>(url: U, destination_directory: P) -> Result<PathBuf> {
    let url = url.into_url()?;
    let destination_directory = destination_directory.as_ref();

    let file_name = url
        .path_segments()
        .and_then(|segments| segments.last())
        .expect("Can't find file name from url");

    let file_path = destination_directory.join(file_name);

    log::info!("Downloading {} into {}", &url, &destination_directory.display());

    let response = reqwest::blocking::get(url.clone())?;

    if response.status().is_success() {
        let content = response.bytes()?;

        let mut out = fs::File::create(file_path.clone())?;

        out.write_all(&content)?;
    } else {
        log::error!("Failed to download file: {}", response.status());
    }

    log::info!("Downloaded {} into {}", &url, &destination_directory.display());

    Ok(file_path)
}

fn extract_bz2<P: AsRef<Path>>(file_path: P) -> Result<()> {
    let file_path = file_path.as_ref();

    log::info!("Extracting {}", file_path.display());

    let output = Command::new("bzip2")
        .arg("-d")
        .arg(file_path)
        .output()
        .expect("Failed to execute command");

    log::debug!("Output {:?}", String::from_utf8(output.stdout));
    log::debug!("Err {:?}", String::from_utf8(output.stderr));

    Ok(())
}

impl Runnable for DownloadArgs {
    fn run(&self) -> Result<()> {
        let config = GLOBAL_CONFIG.read().unwrap();

        fs::create_dir_all(Path::new(&config.models.models_dir)).expect("Unable to create model directory");

        let path = download_file(&config.models.shape_predictor.url, &config.models.models_dir)?;
        extract_bz2(path)?;

        let path = download_file(&config.models.face_recognition.url, &config.models.models_dir)?;
        extract_bz2(path)?;

        Ok(())
    }
}
