use std::path::PathBuf;
use std::sync::Arc;
use iced::futures::lock::Mutex;
use iced::Task;
use yt_dlp::model::{AudioQuality, VideoQuality};
use crate::downloader::download_video;
use crate::model;
use crate::model::{AppModel, SelectableVideoQuality, StatusState};

use crate::model::{};
#[derive(Debug, Clone)]
pub enum Message {
    UrlChanged(String),
    PickFolder,
    FolderPicked(Option<PathBuf>),
    StartDownload,
    DownloadFinished(Result<(), String>),
    // Selectable Audio / Video & Quality / Codec
    VideoQualitySelected(VideoQuality),
}

pub fn update(model: &mut AppModel, message: Message) -> Task<Message> {
    match message {
        Message::UrlChanged(url) => {
            model.video_url = url;
            Task::none()
        }
        Message::PickFolder => Task::perform(
            async { rfd::FileDialog::new().pick_folder() },
            Message::FolderPicked,
        ),
        Message::FolderPicked(path) => {
            model.download_dir = path;
            Task::none()
        }
        Message::StartDownload => {
            if model.status_enum == StatusState::IsDownloading {
                return Task::none();
            }

            let Some(path) = model.download_dir.clone() else {
                model.status_message = "Сначала выберите папку".to_string();
                return Task::none();
            };

            if model.video_url.trim().is_empty() {
                model.status_message = "Сначала вставьте ссылку".to_string();
                return Task::none();
            }

            model.status_enum = StatusState::IsDownloading;
            model.status_message = "Скачивание началось...".to_string();

            let url = model.video_url.clone();
            let async_model: Arc<Mutex<AppModel>> = Arc::new(Mutex::new(model.clone()));
            Task::perform(
                async move { download_video(&path, &url, Arc::clone(&async_model)).await.map_err(|e| e.to_string()) },
                Message::DownloadFinished,
            )
        }
        Message::DownloadFinished(result) => {
            model.status_enum = StatusState::Idle;
            model.status_message = match result {
                Ok(()) => "Скачивание завершено".to_string(),
                Err(err) => format!("Ошибка: {err}"),
            };
            Task::none()
        }
        Message::VideoQualitySelected(video_quality) => {
            model.video_quality_preset = Some(video_quality);
            println!("{:?}", model.video_quality_preset.as_ref().unwrap());
            Task::none()
        }
    }
}
