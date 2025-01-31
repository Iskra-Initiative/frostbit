// separate terminal panes component
mod terminal;

mod config;
mod controller;
mod myserial;

use config::{APP_SETTINGS, WINDOW_SETTINGS, WINDOW_TITLE};
use controller::TerminalController;
use iced::widget::{column, row, Button, Container, Rule, Text};
use iced::{Alignment, Element, Length};

use terminal::Terminal;

#[derive(Default)]
struct State {
    new_disp_val: Vec<String>,
    terminal: Terminal,
}

#[derive(Default)]
struct App {
    state: State,
}

#[derive(Debug, Clone)]
enum Message {
    ButtonClick,
    TerminalMessage(terminal::TerminalMessage),
}

impl App {
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

            Message::TerminalMessage(e) => {
                self.state.terminal.update(e);
            }
        }
    }

    fn view(&self) -> Element<Message> {
        // left sidebar
        let left_sidebar = column![Button::new(Text::new("+")).on_press(Message::ButtonClick)]
            .padding(10)
            .spacing(10);

        let main_content = self.state.terminal.view();

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
    iced::application(WINDOW_TITLE, App::update, App::view)
        .settings(APP_SETTINGS)
        .window(WINDOW_SETTINGS)
        .run()
}
