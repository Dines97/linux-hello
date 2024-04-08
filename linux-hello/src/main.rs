mod cli;
mod config;
mod cycle_controller;
mod state;

use clap::Parser;
use cli::Runnable;
use env_logger::{Builder, Target};

fn main() -> color_eyre::Result<()> {
    Builder::from_default_env()
        .target(Target::Stdout)
        .filter_level(log::LevelFilter::Trace)
        .init();

    opencv_sys::set_num_threads(1);

    let file_path = "data.json";
    state::deserialize(file_path);

    let cli = cli::Cli::parse();
    let _ = cli.command.run();

    state::serialize(file_path);

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
