use std::{error::Error, path::PathBuf};
use std::sync::Arc;
use std::time::Duration;
use iced::futures::lock::Mutex;
use yt_dlp::client::Libraries;
use yt_dlp::model::{AudioCodecPreference, AudioQuality};
use yt_dlp::prelude::VideoCodecPreference;
use yt_dlp::{DownloadPriority, Youtube};

use crate::model::{AppModel, SelectableVideoQuality, StatusState};

pub async fn download_video(
    download_directory: &PathBuf,
    video_url: &String,
    mut model: Arc<Mutex<AppModel>>,
) -> Result<(), Box<dyn Error>> {
    let libraries_dir = PathBuf::from("libs");
    let youtube = libraries_dir.join("yt-dlp");
    let ffmpeg = libraries_dir.join("ffmpeg");

    if !youtube.exists() || !ffmpeg.exists() {
        download_library().await?;
    }

    let libraries = Libraries::new(youtube, ffmpeg);
    let mut fetcher = Youtube::new(libraries, download_directory).await?;
    fetcher.timeout = Duration::from_secs(600);
    fetcher.with_arg("--no-playlist");

    let video = fetcher.fetch_video_infos(video_url.clone()).await?;
    let title = video.title;

    fetcher.download(video_url, format!("{title}.mp4"))
        .video_quality({
            let model = model.lock().await;
            model.video_quality_preset.ok_or("Video quality preset not set")?
        })
        .video_codec(VideoCodecPreference::AV1)
        .audio_quality(AudioQuality::Best)
        .audio_codec(AudioCodecPreference::Opus)
        .priority(DownloadPriority::High)
        .with_progress({
            let model = Arc::clone(&model);
            move |progress| {
                let value = progress as f32;
                println!("Current progress: {}%", (value * 100.0).round());
                let model: Arc<Mutex<AppModel>> = Arc::clone(&model);
                iced::futures::executor::block_on(async move {
                    let mut model = model.lock().await;
                    model.download_progress = value;
                });
            }
        })
        .execute()
        .await?;

    Ok(())
}

pub async fn download_library() -> Result<(), Box<dyn Error>> {
    let executables_dir = PathBuf::from("libs");
    let output_dir = PathBuf::from("output");

    Youtube::with_new_binaries(executables_dir, output_dir).await?;
    Ok(())
}