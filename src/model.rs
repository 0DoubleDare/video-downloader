use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct AppModel {
    pub video_url: String,
    pub download_dir: Option<PathBuf>,
    pub status: String,
    pub is_downloading: bool,
}

impl Default for AppModel {
    fn default() -> Self {
        Self {
            video_url: String::new(),
            download_dir: None,
            status: "Выберите папку и вставьте ссылку".to_string(),
            is_downloading: false,
        }
    }
}
