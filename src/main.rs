#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![expect(rustdoc::missing_crate_level_docs)]

use eframe::egui;
use std::{ path::PathBuf, error::Error };
use rfd::FileDialog;
use yt_dlp::prelude::{VideoQuality, VideoCodecPreference, ManagerConfig};
use yt_dlp::{Youtube, client::Libraries, model::AudioQuality};
use std::sync::mpsc::{Sender, Receiver, channel };
use std::process::Command;
use std::path::Path;
use std::time::Duration;
use egui::FocusDirection::Down;
use tokio::sync::watch;
use yt_dlp::model::format::DownloaderOptions;
// use std::sync::OnceLock;
/// Расширение исполняемого файла. Зависит от операционной системы.
///
/// Windows - ".exe"
///
/// Linux - "" (Пустая строка)
// static EXTENSION: OnceLock<String> = OnceLock::new();
// static EXECUTABLES_DIR: OnceLock<String> = OnceLock::new();

struct MyApp {
    text_url: String,
    file_path: Option<PathBuf>,
    download_progress: f32,
    rx: Option<watch::Receiver<f32>>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            text_url: "".to_string(),
            file_path: None,
            download_progress: 0.0,
            rx: None,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(ref rx) = self.rx {
                self.download_progress = *rx.borrow();
                ctx.request_repaint();
            }
            ui.heading("Video downloader");

            ui.label(format!("Selected: {:?}", self.file_path));
            if ui.button("Select path").clicked() {
                if let Some(path) = FileDialog::new().pick_folder() {
                    self.file_path = Some(path);
                }
            }

            ui.text_edit_singleline(&mut self.text_url);

            if ui.button("Start download").clicked() {
                let url = self.text_url.clone();
                let path = self.file_path.clone().expect("Select path in wrong");
                let (tx, rx) =
                    watch::channel(0.0f32);

                self.rx = Some(rx);

                tokio::spawn(async move {
                    if let Err(e) = download_video(path, url, tx).await {
                        eprintln!("ERROR: {e}");
                    }
                });
            }

            ui.
                add(egui::ProgressBar::new(self.download_progress)
                    .animate(true)
                .show_percentage())
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();

    let _enter = rt.enter();

    let options = eframe::NativeOptions::default();

    eframe::run_native(
        "Everywhere Video Downloader",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    )
}

/// Описание фнукции скачивания видео
///
/// ```output_dir: PathBuf``` - Путь в котором будем хранится ваше видео
///
/// ```url: String``` - Ссылка на видео
///
/// ```tx: Sender<f32>``` - Для изменения прогресса скачивания в UI, отправляет значение типа f32
pub async fn download_video(
    download_directory: PathBuf,
    video_url: String,
    tx: watch::Sender<f32>
) -> Result<(), Box<dyn Error>> {
    let libraries_dir = PathBuf::from("libs");
    let output_dir = PathBuf::from(&download_directory);

    let youtube = libraries_dir.join("yt-dlp");
    let ffmpeg = libraries_dir.join("ffmpeg");

    println!("yt-dlp path: {:?}; Exists: {}", youtube, youtube.exists());
    println!("ffmpeg path: {:?}; Exists: {}", ffmpeg, ffmpeg.exists());

    if !youtube.exists() && !ffmpeg.exists() {
        println!("Needed libraries not found. Install...");
        download_library().await?;
        println!("Needed libraries has been install");
    }

    let libraries = Libraries::new(youtube, ffmpeg);
    let fetcher = Youtube::new(libraries, output_dir).await?;

    let video = fetcher.fetch_video_infos(video_url.clone()).await?;

    for format in &video.formats {
        println!("{:?}", format);
    }
    println!("video infos: {}", video);

    let video_title = video.title;
    let video_ext = "mp4".to_string();
    let tx_progress = tx.clone();

    let download_id = fetcher.download(video_url, format!("{video_title}.{video_ext}"))
        .video_quality(VideoQuality::Best)
        .video_codec(VideoCodecPreference::AV1)
        .audio_quality(AudioQuality::Best)
        .with_progress(move |progress| {
            let val = progress as f32;
            println!("progress: {}%", (val * 100.0).round());
            let _ = tx_progress.send(val).ok();
        })
        .execute()
        .await?;

    println!("Download complete: video available in {:?}", download_directory);
    let _ = tx.send(1.0).ok();
    Ok(())
}

pub async fn download_library() -> Result<(), Box<dyn Error>> {
    let executables_dir = PathBuf::from("libs");
    let output_dir = PathBuf::from("output");

    let fetcher =
        Youtube::with_new_binaries(executables_dir, output_dir).await?;

    Ok(())
}

// #[tokio::main]
// pub async fn update_library() -> Result<(), Box<dyn Error>> {
//     let libraries_dir = PathBuf::from("libs");
//     let output_dir = PathBuf::from("output");
//
//     let youtube = libraries_dir.join("yt-dlp");
//     let ffmpeg = libraries_dir.join("ffmpeg");
//
//     let libraries = Libraries::new(youtube, ffmpeg);
//     let fetcher = Youtube::new(libraries, output_dir);
//
//     fetcher.update_downloader().await?;
//     Ok(())
// }
