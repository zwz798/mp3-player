use crate::Message;
use iced::widget::container::Style;
use iced::widget::{container, text};
use iced::{Background, Color, Element, Length, Task};

#[derive(Debug, Clone)]
pub enum PlaybackMessage {
    PlayPauseClicked,
    PreviousClicked,
    NextClicked,
}

#[derive(Default)]
pub struct PlaybackState {
    current_song_id: Option<u32>,
    status: PlaybackStatus,
    process_ms: u64,
    volume: f32,
    shuffle: bool,
}

#[derive(Default)]
pub enum PlaybackStatus {
    #[default]
    Stopped,
    Playing,
    Paused,
}

pub fn update(message: PlaybackMessage) -> Task<Message> {
    match message {
        PlaybackMessage::PlayPauseClicked => {}
        PlaybackMessage::PreviousClicked => {}
        PlaybackMessage::NextClicked => {}
    }

    Task::none()
}

pub fn footer() -> Element<'static, Message> {
    container(view_music_bar())
        .center(Length::Fill)
        .height(200.0)
        .style(|_theme| Style {
            text_color: None,
            background: Some(Background::Color(Color::from_rgb8(40, 40, 52))),
            border: Default::default(),
            shadow: Default::default(),
        })
        .into()
}

// music bar
fn view_music_bar() -> Element<'static, Message> {
    text!("music bar").into()
}

fn album_art() {}

fn song_info() {}

fn controls() {}

fn volume_control() {}
