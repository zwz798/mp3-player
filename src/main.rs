use iced::widget::{button, container, text};
use iced::{Element, Task};

fn main() -> Result<(), iced::Error> {
    iced::application::application(title, update, view).run()
}

#[derive(Default)]
struct State {}

#[derive(Debug, Clone)]
enum Message {}

fn title(_state: &State) -> String {
    "mp3-player".to_string()
}

fn update(state: &mut State, message: Message) -> Task<Message> {
    Task::none()
}

fn view(state: &State) -> Element<Message> {
    iced::widget::Column::with_children(vec![view_content(), view_bottom()]).into()
}

fn view_content() -> Element<'static, Message> {
    iced::widget::Row::with_children(vec![view_folder(), view_content_right()]).into()
}

// music folder
fn view_folder() -> Element<'static, Message> {
    iced::widget::Column::with_children(vec![
        button(text!("button1")).into(),
        button(text!("button2")).into(),
        button(text!("button3")).into(),
        button(text!("button4")).into(),
    ])
    .into()
}

fn view_content_right() -> Element<'static, Message> {
    container(text!("content_right")).into()
}

fn view_bottom() -> Element<'static, Message> {
    container(view_music_bar()).into()
}

// music bar
fn view_music_bar() -> Element<'static, Message> {
    text!("music bar").into()
}
