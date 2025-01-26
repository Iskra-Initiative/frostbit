mod config;
mod controller;
mod myserial;

use std::f32::consts::LN_10;

use config::{APP_SETTINGS, WINDOW_SETTINGS, WINDOW_TITLE};
use controller::list_available_ports;
use controller::TerminalController;
use iced::widget::{column, row, Button, Container, Rule, Text, TextInput};
use iced::widget::{scrollable, vertical_space};
use iced::{Alignment, Element, Length};

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
                let mut term_runner = TerminalController::new(1);

                let sinfo = myserial::SerialPortInfo::new(
                    "COM1".to_string(),
                    9600,
                    myserial::DataBits::Eight,
                    myserial::Parity::None,
                    myserial::StopBits::One,
                    myserial::FlowControl::None,
                );

                term_runner.create_stream(&sinfo);
                term_runner.end_stream();

                let ports = controller::list_available_ports();
                match ports {
                    Ok(ports) => {
                        for port in ports {
                            println!("{:?}", port);
                        }
                    }
                    Err(e) => {
                        println!("{:?}", e);
                    }
                }
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
        // left sidebar
        let left_sidebar = column![Button::new(Text::new("+")).on_press(Message::ButtonClick)]
            .padding(10)
            .spacing(10);

        // data display
        let display_row = Text::new(&self.display_value)
            .size(20)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(iced::alignment::Horizontal::Left);

        // user input box
        let input_row = TextInput::new("->", &self.input_value)
            .on_input(Message::InputChanged)
            .on_submit(Message::Submit)
            .width(Length::Fixed(400.0))
            .line_height(2.0)
            .padding(10);

        let input_row_with_button = row![
            input_row,
            Button::new(Text::new("Send")).on_press(Message::Submit)
        ]
        .spacing(10);

        // combine display and input box
        let main_content = column![
            Container::new(display_row)
                .width(Length::Fill)
                .height(Length::FillPortion(9)),
            Container::new(input_row_with_button)
                .width(Length::Fill)
                .height(Length::Shrink)
        ]
        .spacing(10)
        .padding(10);

        // combine left sidebar and main content
        let layout = row![
            Container::new(left_sidebar)
                .width(Length::Shrink)
                .height(Length::Fill)
                .padding(10),
            Rule::vertical(2), // border between left sidebar and main content
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
