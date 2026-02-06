#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![expect(rustdoc::missing_crate_level_docs)]

use eframe::egui;
use std::{ path::PathBuf, error::Error };
use std::time::Duration;
use rfd::FileDialog;
use yt_dlp::prelude::{LibraryInstaller};
use yt_dlp::{Youtube, client::deps::Libraries, DownloadPriority};
use yt_dlp::model::selector::{VideoQuality, AudioQuality, VideoCodecPreference};
use tokio::sync::watch;
use yt_dlp::model::AudioCodecPreference;

// use std::sync::OnceLock;
/// Расширение исполняемого файла. Зависит от операционной системы.
///
/// Windows - ".exe"
///
/// Linux - "" (Пустая строка)
// static EXTENSION: OnceLock<String> = OnceLock::new();
// static EXECUTABLES_DIR: OnceLock<String> = OnceLock::new();

enum VideoDownloadState {
    Downloading,
    Succeeded,
    Error,
    State
}
struct MyApp {
    text_url: String,
    file_path: Option<PathBuf>,
    download_state: VideoDownloadState,
    download_progress: f32,
    rx: Option<watch::Receiver<f32>>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            text_url: "".to_string(),
            file_path: Some(PathBuf::from("~/")),
            download_state: VideoDownloadState::State,
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

            // ui.selectable_label();

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

//FIXME: Исправить лимит timeout с 30 секунд до 600
//TODO: Добавить изменения цвета прогресс бара в разных состояниях скачивания видео.
//TODO: Добавить отдельную вкладку объединения аудио и видео
//TODO: Добавить возможность отмены загрузки
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
        check_or_download_library(&youtube, &ffmpeg).await?;
        println!("Needed libraries has been install");
    }

    let libraries = Libraries::new(youtube, ffmpeg);
    let mut fetcher = Youtube::new(libraries, output_dir).await?;
    fetcher.timeout = Duration::from_secs(600);

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
        .video_codec(VideoCodecPreference::Any)
        .audio_quality(AudioQuality::Best)
        .audio_codec(AudioCodecPreference::Opus)
        .priority(DownloadPriority::High)
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

pub async fn check_or_download_library(youtube: &PathBuf, ffmpeg: &PathBuf) -> Result<(), Box<dyn Error>> {
    let executables_dir = PathBuf::from("libs");
    let installer = LibraryInstaller::new(executables_dir);

    if !youtube.exists() {
        let youtube = installer
            .install_youtube(None)
            .await
            .unwrap();
    }
    if !ffmpeg.exists() {
        let ffmpeg = installer
            .install_ffmpeg(None)
            .await
            .unwrap();
    }

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
