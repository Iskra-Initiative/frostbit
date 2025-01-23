use config::{APP_SETTINGS, WINDOW_SETTINGS, WINDOW_TITLE};
use iced::widget::{column, row, Button, Container, Text, TextInput};
use iced::{Alignment, Element, Length};
mod config;

#[derive(Default)]
struct Terminal {
    input_value: String,
    display_value: String,
}

#[derive(Debug, Clone)]
enum Message {
    ButtonClick,
    InputChanged(String),
    Submit,
}

impl Terminal {
    fn update(&mut self, message: Message) {
        match message {
            Message::ButtonClick => {
                println!("btn clicked");
            }
            Message::InputChanged(value) => {
                self.input_value = value;
            }
            Message::Submit => {
                self.display_value = self.input_value.clone();
                self.input_value.clear();
            }
        }
    }
    fn view(&self) -> Element<Message> {
        use iced::widget::Rule;

        // Left sidebar
        let left_sidebar = column![
            Button::new(Text::new("1")).on_press(Message::ButtonClick),
            Button::new(Text::new("2")).on_press(Message::ButtonClick),
            Button::new(Text::new("3")).on_press(Message::ButtonClick)
        ]
        .padding(10)
        .spacing(10);

        // Main content area
        let display_row = Text::new(&self.display_value)
            .size(20)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(iced::alignment::Horizontal::Center);

        let input_row = TextInput::new("->", &self.input_value)
            .on_input(Message::InputChanged)
            .on_submit(Message::Submit)
            .width(Length::Fixed(400.0)) // Fixed width
            .line_height(2.0) // Fixed height
            .padding(10);

        let input_row_with_button = row![
            input_row,
            Button::new(Text::new("Send")).on_press(Message::Submit)
        ]
        .spacing(10);

        let main_content = column![
            Container::new(display_row)
                .width(Length::Fill)
                .height(Length::FillPortion(9)), // Takes 90% of the height
            Container::new(input_row_with_button)
                .width(Length::Fill)
                .height(Length::Shrink) // Shrinks to fit fixed size
        ]
        .spacing(10)
        .padding(10);

        // Combine left sidebar and main content
        let layout = row![
            Container::new(left_sidebar)
                .width(Length::Shrink)
                .height(Length::Fill)
                .padding(10),
            Rule::vertical(2), // Adds a vertical line (border) between the sidebar and the main content
            Container::new(main_content)
                .width(Length::Fill)
                .height(Length::Fill)
                .padding(10)
        ]
        .spacing(10)
        .align_y(Alignment::Start);

        Container::new(layout)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

fn main() -> iced::Result {
    iced::application(WINDOW_TITLE, Terminal::update, Terminal::view)
        .settings(APP_SETTINGS)
        .window(WINDOW_SETTINGS)
        .run()
}
