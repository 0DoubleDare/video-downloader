mod controller;
mod downloader;
mod model;
mod view;

use controller::{Message, update};
use iced::Task;
use model::AppModel;

fn main() -> iced::Result {
    iced::application("YouTube Video Helper", update, view::view)
        .run_with(|| (AppModel::default(), Task::none()))
}

#[allow(dead_code)]
fn _message_type_check(_: Message) {}