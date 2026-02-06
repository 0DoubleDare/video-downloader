use std::{error::Error, path::PathBuf};

use yt_dlp::prelude::{VideoCodecPreference, VideoQuality};
use yt_dlp::{Youtube, client::Libraries, model::AudioQuality};

pub async fn download_video(
    download_directory: PathBuf,
    video_url: String,
) -> Result<(), Box<dyn Error>> {
    let libraries_dir = PathBuf::from("libs");
    let youtube = libraries_dir.join("yt-dlp");
    let ffmpeg = libraries_dir.join("ffmpeg");

    if !youtube.exists() || !ffmpeg.exists() {
        download_library().await?;
    }

    let libraries = Libraries::new(youtube, ffmpeg);
    let fetcher = Youtube::new(libraries, download_directory.clone()).await?;

    let video = fetcher.fetch_video_infos(video_url.clone()).await?;
    let title = video.title;

    fetcher
        .download(video_url, format!("{title}.mp4"))
        .video_quality(VideoQuality::Best)
        .video_codec(VideoCodecPreference::AV1)
        .audio_quality(AudioQuality::Best)
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
