use std::path::PathBuf;

use iced::Task;

use crate::downloader::download_video;
use crate::model::AppModel;

#[derive(Debug, Clone)]
pub enum Message {
    UrlChanged(String),
    PickFolder,
    FolderPicked(Option<PathBuf>),
    StartDownload,
    DownloadFinished(Result<(), String>),
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
            if model.is_downloading {
                return Task::none();
            }

            let Some(path) = model.download_dir.clone() else {
                model.status = "Сначала выберите папку".to_string();
                return Task::none();
            };

            if model.video_url.trim().is_empty() {
                model.status = "Сначала вставьте ссылку".to_string();
                return Task::none();
            }

            model.is_downloading = true;
            model.status = "Скачивание началось...".to_string();

            let url = model.video_url.clone();
            Task::perform(
                async move { download_video(path, url).await.map_err(|e| e.to_string()) },
                Message::DownloadFinished,
            )
        }
        Message::DownloadFinished(result) => {
            model.is_downloading = false;
            model.status = match result {
                Ok(()) => "Скачивание завершено".to_string(),
                Err(err) => format!("Ошибка: {err}"),
            };
            Task::none()
        }
    }
}
