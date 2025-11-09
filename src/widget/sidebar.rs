use crate::Message;
use iced::border::Radius;
use iced::widget::container::Style;
use iced::widget::{button, container, text};
use iced::{Background, Border, Color, Element, Length, Padding, Task};
use std::collections::HashMap;

pub fn update(message: SidebarMessage) -> Task<Message> {
    match message {
        SidebarMessage::AddMusicFolderClicked => {}
        SidebarMessage::MusicFolderClicked => {}
    }

    Task::none()
}

#[derive(Debug, Clone)]
pub enum SidebarMessage {
    AddMusicFolderClicked,
    MusicFolderClicked,
}

#[derive(Default)]
pub struct SidebarState {}

#[derive(Debug, Default)]
pub enum SelectedSidebar {
    #[default]
    None,
    MusicLibrary,
    Liked,
    PlayList,
}

#[derive(Default)]
pub struct LibraryState {
    songs: HashMap<u64, u64>,
    albums: HashMap<u64, u64>,
    artists: HashMap<u64, u64>,
}

pub fn sidebar() -> Element<'static, Message> {
    let column = iced::widget::Column::with_children(vec![
        button(text!("music library"))
            .width(Length::Fill)
            .style(|_theme, _status| sidebar_button_style())
            .into(),
        button(text!("liked"))
            .style(|_theme, _status| sidebar_button_style())
            .width(Length::Fill)
            .into(),
        button(text!("default music folder"))
            .width(Length::Fill)
            .style(|_theme, _status| sidebar_button_style())
            .into(),
    ])
    .padding(Padding::from(10));

    container(column)
        .style(|_theme| Style {
            text_color: None,
            background: Some(Background::Color(Color::from_rgb8(112, 112, 122))),
            border: Default::default(),
            shadow: Default::default(),
        })
        .width(Length::Fixed(200.0))
        .height(Length::Fill)
        .into()
}

fn sidebar_button_style() -> button::Style {
    button::Style {
        background: None,
        text_color: Color::WHITE,
        border: Border {
            color: Default::default(),
            width: 0.0,
            radius: Radius::from(8),
        },
        shadow: Default::default(),
    }
}
