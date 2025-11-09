use crate::Message;
use iced::widget::{container, text};
use iced::{Element, Length, Task};

#[derive(Debug, Clone)]
pub enum PlayListMessage {
    SongSelected(u64),
}

pub fn update(message: PlayListMessage) -> Task<Message> {
    match message { 
        PlayListMessage::SongSelected(u64) => {
            
        }
    }
    
    Task::none()
}

pub fn main_panel() -> Element<'static, Message> {
    container(text!("content_right"))
        .width(Length::Fill)
        .height(Length::Fill)
        .center(Length::Fill)
        .into()
}
