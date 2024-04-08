use std::{fs, io::Write, path::PathBuf, process::Command, str::FromStr};

use clap::Args;
use reqwest::Url;

use crate::config::GLOBAL_CONFIG;

use super::Runnable;
use color_eyre::{eyre::Ok, Result};

#[derive(Debug, Args)]
pub(crate) struct DownloadArgs {}

fn download_file(url_str: &str, destination_path: &PathBuf) -> Result<()> {
    let url = Url::parse(url_str)?;

    log::info!("Downloading {} into {}", url_str, &destination_path.display());

    let response = reqwest::blocking::get(url)?;

    if response.status().is_success() {
        let content = response.bytes()?;

        fs::create_dir_all(destination_path)?;
        let mut out = fs::File::create(destination_path)?;
        out.write_all(&content)?;
    } else {
        log::error!("Failed to download file: {}", response.status());
    }

    log::info!("Downloaded {} into {}", url_str, &destination_path.display());

    Ok(())
}

fn extract_bz2(file_path: &String) -> Result<()> {
    log::info!("Extracting {}", file_path);

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

        download_file(
            &config.models.shape_predictor.url,
            &PathBuf::from_str(&config.models.shape_predictor.file_path)?,
        )?;
        extract_bz2(&config.models.shape_predictor.file_path)?;

        download_file(
            &config.models.face_recognition.url,
            &PathBuf::from_str(&config.models.face_recognition.file_path)?,
        )?;
        extract_bz2(&config.models.face_recognition.file_path)?;

        Ok(())
    }
}
