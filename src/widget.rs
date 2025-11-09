use crate::Message;
use crate::widget::footer::footer;
use crate::widget::main_panel::main_panel;
use crate::widget::sidebar::sidebar;
use iced::{Element, Length};

pub mod footer;
pub mod main_panel;
pub mod sidebar;

pub fn view() -> Element<'static, Message> {
    iced::widget::Column::with_children(vec![content(), footer()]).into()
}

fn content() -> Element<'static, Message> {
    iced::widget::Row::with_children(vec![sidebar(), main_panel()])
        .height(Length::Fill)
        .into()
}
