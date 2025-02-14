mod config;
mod controller;
mod myserial;
mod sidebar;
mod terminal;
mod theme;

use config::{APP_SETTINGS, WINDOW_TITLE, window_settings};
use controller::TerminalController;
use iced::widget::{column, container, row, Button, Container, Rule, Text};
use iced::{Alignment, Element, Length, Theme};

use tracing::level_filters::LevelFilter;
use tracing::{event, Level};
use tracing_subscriber;

use terminal::TerminalPane;

#[derive(Default)]
struct State {
    terminal: TerminalPane,
    left_sidebar: sidebar::Sidebar,
}

#[derive(Default)]
struct App {
    state: State,
}

#[derive(Debug, Clone)]
enum Message {
    SpawnTerminalSession,
    TerminalPaneMessage(terminal::TerminalPaneMessage),
    SidebarMessage(sidebar::SidebarMessage),
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

            Message::TerminalPaneMessage(e) => {
                self.state.terminal.update(e);
            }

            Message::SidebarMessage(e) => {
                self.state.left_sidebar.update(e);
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let left_sidebar: Element<Message> = self.state.left_sidebar.view();
        let main_content = self.state.terminal.view();
        let layout = row![
            container(left_sidebar)
                .width(Length::Shrink)
                .height(Length::Fill)
                .padding(10),
            Rule::vertical(2),
            container(main_content)
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
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive("frostbit=debug".parse().unwrap()),
        )
        .init();

    iced::application(WINDOW_TITLE, App::update, App::view)
        .settings(APP_SETTINGS)
        .window(window_settings())
        .theme(|_| Theme::Dracula)
        .run()
}
