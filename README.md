# video-downloader

Небольшой pet-проект для скачивания видео.

## Stack

- GUI: `iced`
- Async runtime: `tokio`
- Downloader: `yt-dlp`

## Архитектура
Код разделен на:
- `src/model.rs` — Состояние приложения (Model)
- `src/view.rs` — Отрисовка UI (View)
- `src/controller.rs` — Обработка сообщений и бизнес-логика (Controller)
- `src/downloader.rs` — Логика скачивания
