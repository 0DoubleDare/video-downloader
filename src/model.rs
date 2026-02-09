use std::path::PathBuf;
use yt_dlp::prelude::VideoQuality;

#[derive(Debug, Clone, PartialEq)]
pub enum StatusState {
    IsDownloading,
    Idle,
    Error,
}

pub struct State {
    selection: Option<SelectableVideoQuality>,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelectableVideoCodec {

}
pub enum SelectableAudioCodec {

}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelectableVideoQuality {

}
pub enum SelectableAudioQuality {

}
#[derive(Debug, Clone)]
pub struct AppModel {
    pub video_url: String,
    pub download_dir: Option<PathBuf>,
    pub status_message: String,
    pub status_enum: StatusState,
    pub download_progress: f32,
    pub video_quality_preset: Option<VideoQuality>,
}

impl Default for AppModel {
    fn default() -> Self {
        Self {
            video_url: String::new(),
            download_dir: None,
            status_message: "Выберите папку и вставьте ссылку".to_string(),
            status_enum: StatusState::Idle,
            download_progress: 0.0,
            video_quality_preset: None,
        }
    }
}