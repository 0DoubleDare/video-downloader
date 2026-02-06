use iced::widget::{button, column, container, row, text, text_input};
use iced::{Alignment, Element, Length};

use crate::controller::Message;
use crate::model::AppModel;

pub fn view(model: &AppModel) -> Element<'_, Message> {
    let selected_path = model
        .download_dir
        .as_ref()
        .map(|p| p.display().to_string())
        .unwrap_or_else(|| "Не выбрана".to_string());

    let content = column![
        text("Video downloader (Iced + MVC)").size(32),
        text(format!("Папка: {selected_path}")),
        row![button("Выбрать папку").on_press(Message::PickFolder)]
            .spacing(8)
            .align_y(Alignment::Center),
        text_input("Вставьте ссылку на видео", &model.video_url)
            .on_input(Message::UrlChanged)
            .padding(10)
            .size(18),
        button(if model.is_downloading {
            "Скачивание..."
        } else {
            "Начать скачивание"
        })
        .on_press_maybe((!model.is_downloading).then_some(Message::StartDownload)),
        text(&model.status),
    ]
    .spacing(16)
    .padding(20)
    .width(Length::Fill)
    .align_x(Alignment::Start);

    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .into()
}
