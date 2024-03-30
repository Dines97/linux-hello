mod cli;
mod cycle_controller;
mod types;

use clap::Parser;
use cli::Runnable;
use env_logger::{Builder, Target};
use std::{
    fs::{File, OpenOptions},
    io::{BufReader, BufWriter, Read},
    path::Path,
    sync::RwLock,
};
use types::{Data, GLOBAL_DATA};

fn main() -> color_eyre::Result<()> {
    Builder::from_default_env()
        .target(Target::Stdout)
        .filter_level(log::LevelFilter::Trace)
        .init();

    opencv_sys::set_num_threads(1);

    let file_path = "data.json";
    let data: types::Data = if Path::new(file_path).exists() {
        let file = File::open(file_path).expect("Failed to read file");
        let reader = BufReader::new(file);
        serde_json::from_reader(reader).expect("Failed to deserialize")
    } else {
        types::Data::new()
    };
    GLOBAL_DATA.set(RwLock::new(data));

    let cli = cli::Cli::parse();
    let _ = cli.command.run();

    let output_file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(file_path)
        .expect("Failed to open output file");
    let writer = BufWriter::new(output_file);

    let _ = serde_json::to_writer(writer, GLOBAL_DATA.get());

    Ok(())
}

// fn main() {
//     let mut video_capture: VideoCapture = VideoCapture::new(2, VideoCaptureAPIs::CapAny);
//
//     let mut mat: Mat = Mat::new();
//
//     named_window("hello");
//
//     loop {
//         stream_extraction(&mut video_capture, &mut mat);
//         imshow("hello", &mut mat);
//         wait_key(50);
//     }
// }
