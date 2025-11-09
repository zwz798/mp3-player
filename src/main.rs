mod widget;

use crate::widget::footer::{PlaybackMessage, PlaybackState};
use crate::widget::main_panel::PlayListMessage;
use crate::widget::sidebar::{LibraryState, SidebarMessage};
use iced::{Element, Task};
use rusqlite::Connection;

fn main() -> Result<(), iced::Error> {
    iced::application::application(title, update, view).run()
}

#[derive(Default)]
struct State {
    playback_state: PlaybackState,
    library_state: LibraryState,
}

#[derive(Debug, Clone)]
enum Message {
    PlaybackMessage(PlaybackMessage),
    PlayListMessage(PlayListMessage),
    SidebarMessage(SidebarMessage),
}

fn title(_state: &State) -> String {
    "mp3-player".to_string()
}

fn update(state: &mut State, message: Message) -> Task<Message> {
    match message {
        Message::PlaybackMessage(playback_message) => widget::footer::update(playback_message),
        Message::PlayListMessage(play_list_message) => {
            widget::main_panel::update(play_list_message)
        }
        Message::SidebarMessage(sidebar_message) => widget::sidebar::update(sidebar_message),
    }
}

fn view(state: &State) -> Element<Message> {
    widget::view()
}

#[derive(Debug, Clone)]
pub enum DbError {
    RusqliteError(String),
}

// 这是 `Task::perform` 调用的异步函数
async fn load_folders_from_db() -> Result<Vec<String>, DbError> {
    tokio::task::spawn_blocking(|| {
        // find the db path
        let db_path = match dirs::data_dir() {
            Some(mut path) => {
                path.push("mps-players");
                if !path.exists() {
                    std::fs::create_dir_all(&path)
                        .map_err(|e| DbError::RusqliteError(e.to_string()))?
                }

                path.push("music_player.db");
                path
            }
            None => {
                return Err(DbError::RusqliteError(
                    "Could not find user data directory".to_string(),
                ));
            }
        };

        let conn = Connection::open(db_path).map_err(|e| DbError::RusqliteError(e.to_string()))?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS music_folders (
                id INTEGER PRIMARY KEY,
                path TEXT NOT NULL UNIQUE
            )",
            [],
        )
        .map_err(|e| DbError::RusqliteError(e.to_string()))?;

        let mut stmt = conn
            .prepare("SELECT path FROM music_folders")
            .map_err(|e| DbError::RusqliteError(e.to_string()))?;

        let res_iter = stmt
            .query_map([], |row| Ok(row.get::<_, String>(0)?))
            .map_err(|e| DbError::RusqliteError(e.to_string()))?;

        let mut folder_names: Vec<String> = Vec::new();
        for res in res_iter {
            if let Ok(folder_name) = res {
                let folder_names: &mut Vec<String> = folder_names.as_mut();
                folder_names.push(folder_name)
            }
        }

        if folder_names.is_empty() {

        }

        Ok(folder_names)
    })
    .await
    .map_err(|e| DbError::RusqliteError(e.to_string()))?
}
