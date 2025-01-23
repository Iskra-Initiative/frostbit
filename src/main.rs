use config::{APP_SETTINGS, WINDOW_SETTINGS, WINDOW_TITLE};
use iced::widget::{text, Column};
mod config;

#[derive(Default)]
struct Terminal {}

#[derive(Debug, Clone)]
enum Message {
    ButtonClick,
}

impl Terminal {
    fn update(&mut self, message: Message) {
        // update app state
        match message {
            Message::ButtonClick => {
                println!("Button clicked");
            }
        }
    }

    fn view(&self) -> Column<Message> {
        // app view
        Column::new()
            .push(text("This is where you will show the view of your app"))
            .push(iced::widget::button(text("click")).on_press(Message::ButtonClick))
    }
}

fn main() -> iced::Result {
    iced::application(WINDOW_TITLE, Terminal::update, Terminal::view)
        .settings(APP_SETTINGS)
        .window(WINDOW_SETTINGS)
        .run()
}
