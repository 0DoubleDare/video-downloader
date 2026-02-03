#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![expect(rustdoc::missing_crate_level_docs)] // it's an example

use eframe::egui;
use std::path::PathBuf;
use rfd::FileDialog;
use yt_dlp::cache::{VideoCodecPreference, VideoQuality};
use yt_dlp::{Youtube, client::Libraries, model::AudioQuality};
use std::sync::mpsc;
use std::sync::mpsc::{Sender, Receiver, channel };
use egui::text_selection::accesskit_text::update_accesskit_for_text_widget;

struct MyApp {
    text_url: String,
    file_path: Option<PathBuf>,
    download_progress: f32,
    rx: Option<Receiver<f32>>,
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
                while let Ok(val) = rx.try_recv() {
                    self.download_progress = val;
                    ctx.request_repaint();
                }
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
                let (tx, rx) = channel::<f32>();

                self.rx = Some(rx);

                tokio::spawn(async move {
                   let _ = download_video(path, url.clone(), tx).await;
                });
            }

            ui.add(egui::ProgressBar::new(self.download_progress)
                .show_percentage())
        });
    }
}

fn main() -> Result<(), eframe::Error>{
    let rt = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(2)
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

async fn download_video(
    output_dir: PathBuf,
    url: String,
    tx: Sender<f32>
) -> Result<(), Box<dyn std::error::Error>> {
    let libraries_dir = PathBuf::from("libs");
    let youtube = libraries_dir.join("yt-dlp");
    let ffmpeg = libraries_dir.join("ffmpeg");

    println!("yt-dlp path: {:?}", youtube);
    println!("ffmpeg path: {:?}", ffmpeg);

    let libraries = Libraries::new(youtube, ffmpeg);
    let fetcher = Youtube::new(libraries, output_dir.clone()).await.expect("Fetcher error");

    println!("...Start download...");
    let _ = tx.send(0.5);

    let video_path =
        fetcher.download(url.clone(), output_dir.join("my-video.mp4"))
            .video_quality(VideoQuality::Best)
            .video_codec(VideoCodecPreference::AV1)
            .audio_quality(AudioQuality::Best)
            .execute().await?;

    let _ = tx.send(1.0);
    println!("Downloaded to: {:?}", video_path);
    Ok(())
}
