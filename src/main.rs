use iced::{Settings, run};
use iced::widget::{text, Column};

#[derive(Default)]
struct Terminal {}

#[derive(Debug, Clone)]
enum Message {
    Button_Click,
}

impl Terminal {
	fn update(&mut self, message: Message) {
		// update app state
        match message {
            Message::Button_Click => {
                println!("Button clicked");
            }
        }
	}

	fn view(&self) -> Column<Message> {
		Column::new()
			.push(text("This is where you will show the view of your app"))

        .push(iced::widget::button(text("click")).on_press(Message::Button_Click))
	}
}

fn main() -> iced::Result {
    iced::run("t", Terminal::update, Terminal::view)
}
