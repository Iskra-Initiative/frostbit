use iced::widget::{button, column, container, pick_list, text};
use iced::Element;

use crate::{myserial::SerialPortInfo, Message};

pub struct Sidebar {
    width: u32,
    height: u32,
}

#[derive(Debug, Clone)]
pub enum SidebarMessage {
    ConnectPressed,
    DisconnectPressed,
    RefreshPressed,
}

impl Default for Sidebar {
    fn default() -> Self {
        Self {
            width: 100,
            height: 100,
        }
    }
}

impl Sidebar {
    pub fn update(&mut self, message: SidebarMessage) {
        match message {
            SidebarMessage::ConnectPressed => {
                println!("Connect button pressed");
            }
            SidebarMessage::DisconnectPressed => {
                println!("Disconnect button pressed");
            }
            SidebarMessage::RefreshPressed => {
                println!("Refresh button pressed");
            }
        }
    }

    pub fn view<'a>(
        &self,
        available_ports: &'a [SerialPortInfo],
        selected_port: &'a Option<SerialPortInfo>,
        is_connected: bool,
    ) -> Element<'a, Message> {
        let port_dropdown = pick_list(available_ports, selected_port.as_ref(), |port| {
            Message::PortSelected(port.clone())
        })
        .placeholder("Select COM port...");

        let connect_button = if is_connected {
            button("-").on_press(Message::SidebarMessage(SidebarMessage::DisconnectPressed))
        } else {
            button("+").on_press(Message::SidebarMessage(SidebarMessage::ConnectPressed))
        };

        let refresh_button =
            button("Refresh").on_press(Message::SidebarMessage(SidebarMessage::RefreshPressed));

        let status_text = if is_connected {
            if let Some(port) = selected_port {
                text(format!("Connected to {}", port.name))
            } else {
                text("Connected")
            }
        } else {
            text("Disconnected")
        };

        let left_sidebar = column![
            text("COM Ports:"),
            port_dropdown,
            connect_button,
            refresh_button,
            status_text,
        ]
        .padding(10)
        .spacing(10);

        container(left_sidebar).into()
    }
}
