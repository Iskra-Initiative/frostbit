use iced::widget::{column, container, pick_list, text};
use iced::{Element, Theme};

use crate::Message;

pub struct Style {
    theme: Theme,
}

#[derive(Clone, Debug)]
pub enum StyleMessage {
    ThemeChanged(Theme),
}

impl Default for Style {
    fn default() -> Self {
        Self {
            theme: Theme::Oxocarbon,
        }
    }
}

impl Style {
    pub fn update(&mut self, message: StyleMessage) {
        match message {
            StyleMessage::ThemeChanged(theme) => {
                self.theme = theme;
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let choose_theme = column![
            text("theme:"),
            pick_list(
                Theme::ALL,
                Some(&self.theme),
                |theme| Message::StyleMessage(StyleMessage::ThemeChanged(theme))
            ),
        ]
        .spacing(10);

        container(choose_theme).into()
    }

    pub fn theme(&self) -> Theme {
        self.theme.clone()
    }
}
