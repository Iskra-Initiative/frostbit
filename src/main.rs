mod config;
mod controller;
mod myserial;
mod sidebar;
mod terminal;
mod theme;

use tracing_subscriber;

use config::{window_settings, APP_SETTINGS, WINDOW_TITLE};
use controller::TerminalController;
use iced::application;
use iced::widget::{container, row, Container, Rule};
use iced::{Alignment, Element, Length, Subscription};
use myserial::SerialPortInfo;
use std::time::Duration;

struct State {
    terminal: terminal::TerminalPane,
    left_sidebar: sidebar::Sidebar,
    style: theme::theme::Style,
    available_ports: Vec<SerialPortInfo>,
    selected_port: Option<SerialPortInfo>,
    terminal_controller: Option<TerminalController>,
    is_connected: bool,
}

impl Default for State {
    fn default() -> Self {
        let available_ports = controller::list_available_ports().unwrap_or_default();
        Self {
            terminal: terminal::TerminalPane::default(),
            left_sidebar: sidebar::Sidebar::default(),
            style: theme::theme::Style::default(),
            available_ports,
            selected_port: None,
            terminal_controller: None,
            is_connected: false,
        }
    }
}

#[derive(Default)]
struct App {
    state: State,
}

#[derive(Debug, Clone)]
enum Message {
    TerminalPaneMessage(terminal::TerminalPaneMessage),
    SidebarMessage(sidebar::SidebarMessage),
    StyleMessage(theme::theme::StyleMessage),
    PortSelected(SerialPortInfo),
    ConnectToPort,
    DisconnectFromPort,
    SendData(String),
    RefreshPorts,
    ReceivedData(String),
    CheckForReceivedData,
}

impl App {
    fn update(&mut self, message: Message) {
        match message {
            Message::TerminalPaneMessage(msg) => {
                match &msg {
                    terminal::TerminalPaneMessage::InputSubmit => {
                        // Send data to connected port if available
                        if let Some(controller) = &self.state.terminal_controller {
                            let data_to_send = self.state.terminal.input_value.clone();
                            println!("UI: Attempting to send data: '{}'", data_to_send);

                            if !data_to_send.trim().is_empty() {
                                match controller.push(data_to_send.clone()) {
                                    Ok(_) => {
                                        println!("UI: Successfully queued data for transmission");
                                        self.state
                                            .terminal
                                            .add_message(&format!("Sent: {}", data_to_send));
                                    }
                                    Err(e) => {
                                        println!("UI: Failed to queue data: {:?}", e);
                                        self.state
                                            .terminal
                                            .add_message(&format!("Error sending data: {}", e));
                                    }
                                }
                            } else {
                                println!("UI: Not sending empty data");
                            }
                        } else {
                            println!("UI: No controller available for sending data");
                            self.state.terminal.add_message("Not connected to any port");
                        }
                    }
                    terminal::TerminalPaneMessage::InputChanged(_) => {
                        // Check for received data on every input change
                        if let Some(controller) = &self.state.terminal_controller {
                            while let Some(data) = controller.try_receive_data() {
                                println!("UI received data: {}", data);
                                self.state.terminal.add_message(&data);
                            }
                        }
                    }
                }
                self.state.terminal.update(msg);
            }

            Message::SidebarMessage(msg) => {
                match &msg {
                    sidebar::SidebarMessage::ConnectPressed => {
                        self.update(Message::ConnectToPort);
                    }
                    sidebar::SidebarMessage::DisconnectPressed => {
                        self.update(Message::DisconnectFromPort);
                    }
                    sidebar::SidebarMessage::RefreshPressed => {
                        self.update(Message::RefreshPorts);
                    }
                }
                self.state.left_sidebar.update(msg);

                // Check for received data on sidebar interactions
                if let Some(controller) = &self.state.terminal_controller {
                    while let Some(data) = controller.try_receive_data() {
                        self.state.terminal.add_message(&data);
                    }
                }
            }

            Message::StyleMessage(e) => {
                self.state.style.update(e);
            }

            Message::PortSelected(port) => {
                self.state.selected_port = Some(port);
            }

            Message::ConnectToPort => {
                if let Some(port) = &self.state.selected_port {
                    if !self.state.is_connected {
                        println!("UI: Attempting to connect to port: {}", port.name);
                        let mut controller = TerminalController::new(1);

                        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                            controller.create_stream(port);
                        })) {
                            Ok(_) => {
                                self.state.terminal_controller = Some(controller);
                                self.state.is_connected = true;
                                self.state
                                    .terminal
                                    .add_message(&format!("Connected to {}", port.name));
                                println!("UI: Successfully connected to {}", port.name);
                            }
                            Err(_) => {
                                self.state
                                    .terminal
                                    .add_message(&format!("Failed to connect to {}", port.name));
                                println!("UI: Failed to connect to {}", port.name);
                            }
                        }
                    } else {
                        println!("UI: Already connected");
                    }
                } else {
                    self.state.terminal.add_message("No port selected");
                    println!("UI: No port selected for connection");
                }
            }

            Message::DisconnectFromPort => {
                if self.state.is_connected {
                    if let Some(mut controller) = self.state.terminal_controller.take() {
                        controller.end_stream();
                    }
                    self.state.is_connected = false;
                    self.state.terminal.add_message("Disconnected");
                }
            }

            Message::SendData(data) => {
                if let Some(controller) = &self.state.terminal_controller {
                    if let Err(e) = controller.push(data.clone()) {
                        self.state
                            .terminal
                            .add_message(&format!("Error sending data: {}", e));
                    } else {
                        self.state.terminal.add_message(&format!("Sent: {}", data));
                    }
                }
            }

            Message::RefreshPorts => {
                self.state.available_ports = controller::list_available_ports().unwrap_or_default();
            }

            Message::ReceivedData(data) => {
                self.state.terminal.add_message(&data);
            }

            Message::CheckForReceivedData => {
                if let Some(controller) = &self.state.terminal_controller {
                    // Check for multiple messages in the queue
                    let mut received_any = false;
                    while let Some(data) = controller.try_receive_data() {
                        println!("UI processing received data: {}", data);
                        self.state.terminal.add_message(&data);
                        received_any = true;
                    }
                    if received_any {
                        println!("Processed received data in UI");
                    }
                }
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let left_sidebar: Element<Message> = self.state.left_sidebar.view(
            &self.state.available_ports,
            &self.state.selected_port,
            self.state.is_connected,
        );
        let main_content = self.state.terminal.view();

        let style = self.state.style.view();

        let layout = row![
            container(left_sidebar)
                .width(Length::Shrink)
                .height(Length::Fill)
                .padding(10),
            Rule::vertical(2),
            container(main_content)
                .width(Length::Fill)
                .height(Length::Fill)
                .padding(10),
            Rule::vertical(2),
            container(style)
                .width(Length::Shrink)
                .height(Length::Fill)
                .padding(10),
        ]
        .spacing(10)
        .align_y(Alignment::Start);

        Container::new(layout)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn subscription(&self) -> Subscription<Message> {
        if self.state.is_connected {
            // Check for received data every 50ms when connected
            iced::time::every(Duration::from_millis(50)).map(|_| Message::CheckForReceivedData)
        } else {
            Subscription::none()
        }
    }
}

fn main() -> iced::Result {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive("frostbit=debug".parse().unwrap())
                // Suppress WGPU/DirectX 12 resource state errors and warnings
                .add_directive("wgpu_hal::auxil::dxgi::exception=off".parse().unwrap())
                .add_directive("wgpu_hal::dx12=error".parse().unwrap())
                // Suppress Vulkan validation layer warnings
                .add_directive("wgpu_hal::vulkan::instance=error".parse().unwrap())
                .add_directive("wgpu_hal=warn".parse().unwrap())
                .add_directive("wgpu_core=warn".parse().unwrap())
                .add_directive("wgpu=warn".parse().unwrap()),
        )
        .init();

    application(WINDOW_TITLE, App::update, App::view)
        .settings(APP_SETTINGS)
        .window(window_settings())
        .theme(|app| app.state.style.theme())
        .subscription(App::subscription)
        .run()
}
