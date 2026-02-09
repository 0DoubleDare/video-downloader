use iced::widget::{button, column, container,
                   row, text, text_input, progress_bar,
                   stack, radio, pick_list,};
use iced::{Alignment, Element, Length};

use yt_dlp::prelude::VideoQuality;

use crate::controller::Message;
use crate::model::{AppModel, SelectableVideoQuality, StatusState};
use crate::model;

pub fn view(model: &AppModel) -> Element<'_, Message> {
    let selected_path = model
        .download_dir
        .as_ref()
        .map(|p| p.display().to_string())
        .unwrap_or_else(|| "Не выбрана".to_string());
    let video_preset = model.video_quality_preset.clone();
    let message = Message::VideoQualitySelected;
    let quality_preset_select = row![
        radio(
            "Лучшее", VideoQuality::Best,
            video_preset, message
        ),
        radio(
            "Высокое", VideoQuality::High,
            video_preset, message
        ),
        radio(
            "Среднее", VideoQuality::Medium,
            video_preset, message
        ),
        radio(
            "Низкое", VideoQuality::Low,
            video_preset, message
        ),
        radio(
           "Худшее", VideoQuality::Worst,
            video_preset, message
        ),
    ].spacing(8);

    let content = column![
        text("Video downloader ").size(32),
        text(format!("Папка: {selected_path}")),
        row![button("Выбрать папку").on_press(Message::PickFolder)]
            .spacing(8)
            .align_y(Alignment::Center),
        text_input("Вставьте ссылку на видео", &model.video_url)
            .on_input(Message::UrlChanged)
            .padding(8)
            .size(14),
        button(if model.status_enum == StatusState::IsDownloading {
            "Скачивание..."
        } else {
            "Начать скачивание"
        })
        .on_press_maybe((model.status_enum != StatusState::IsDownloading).then_some(Message::StartDownload)),
        text(&model.status_message),

        progress_bar(0.0..=1.0, model.download_progress).height(25),

        quality_preset_select,
    ].spacing(16)
        .padding(20)
        .width(Length::Fill)
        .align_x(Alignment::Start);

    container(content).width(Length::Fill)
        .height(Length::Fill)
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .into()
}