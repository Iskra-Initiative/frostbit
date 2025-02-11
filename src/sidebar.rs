use iced::widget::{button, column, container, text};
use iced::Element;

use crate::Message;

pub struct Sidebar {
    width: u32,
    height: u32,
    items: Vec<String>,
}

#[derive(Debug, Clone)]
pub enum SidebarMessage {
    ItemSelected(usize),
}

impl Default for Sidebar {
    fn default() -> Self {
        Self {
            width: 100,
            height: 100,
            items: vec!["Item 1".to_string(), "Item 2".to_string()],
        }
    }
}

impl Sidebar {
    pub fn new(width: u32, height: u32, items: Vec<String>) -> Sidebar {
        Sidebar {
            width,
            height,
            items,
        }
    }

    pub fn update(&mut self, message: SidebarMessage) {
        match message {
            SidebarMessage::ItemSelected(index) => {
                println!("Item selected: {}", self.items[index]);
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let left_sidebar = column![
            button("+c").on_press(Message::SidebarMessage(SidebarMessage::ItemSelected(0))),
        ]
        .padding(10)
        .spacing(10);

        container(left_sidebar).into()
    }
}
