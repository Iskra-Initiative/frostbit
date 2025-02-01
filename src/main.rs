mod config;
mod controller;
mod myserial;
mod terminal;

use config::{APP_SETTINGS, WINDOW_SETTINGS, WINDOW_TITLE};
use controller::TerminalController;
use iced::widget::{column, row, Button, Container, Rule, Text};
use iced::{Alignment, Element, Length};

use terminal::TerminalPane;

#[derive(Default)]
struct State {
    terminal: TerminalPane,
}

#[derive(Default)]
struct App {
    state: State,
}

#[derive(Debug, Clone)]
enum Message {
    SpawnTerminalSession,
    TerminalPaneMessage(terminal::TerminalPaneMessage),
}

impl App {
    fn update(&mut self, message: Message) {
        match message {
            Message::SpawnTerminalSession => {
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

            // pass TerminalMessage to terminal component for handling
            Message::TerminalPaneMessage(e) => {
                self.state.terminal.update(e);
            }
        }
    }

    fn view(&self) -> Element<Message> {
        // render left sidebar
        let left_sidebar =
            column![Button::new(Text::new("+")).on_press(Message::SpawnTerminalSession)]
                .padding(10)
                .spacing(10);

        // render terminal pane view
        let main_content = self.state.terminal.view();

        // combine app layout elements
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

        // render app layout
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
