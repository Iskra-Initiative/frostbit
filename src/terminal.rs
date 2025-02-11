use iced::widget::{button, scrollable, text, text_input, Scrollable};
use iced::widget::{column, container};
use iced::{Alignment, Element, Length};

use tracing::{event, Level};

use crate::Message;

#[derive(Debug, Clone)]
pub enum TerminalPaneMessage {
    InputChanged(String),
    InputSubmit,
}

/// TerminalPane state
#[derive(Debug, Default)]
pub struct TerminalPane {
    pub input_value: String,
    pub display_value: String,
    line_num: u32,
    char_num: u32,
}

impl TerminalPane {
    fn reg_data(&mut self, new_data: &String) {
        // match character amount
        match self.char_num {
            0 => return,
            _ => {}
        }

        // match line amount and limit it to 30
        match self.line_num {
            (0..=30) => {
                self.display_value.push_str(new_data);
                self.display_value.push('\n');
                self.line_num += 1;
            }
            _ => {
                let mut first_newline = self.display_value.chars().position(|c| c == '\n').unwrap();
                first_newline += 1;
                self.display_value = self.display_value[first_newline..].to_string();
                self.display_value.push_str(new_data);
                self.display_value.push('\n');
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let input_row = text_input(">", &self.input_value)
            .on_input(|value| {
                Message::TerminalPaneMessage(TerminalPaneMessage::InputChanged(value))
            })
            .on_submit(Message::TerminalPaneMessage(
                TerminalPaneMessage::InputSubmit,
            ))
            .width(Length::Fill)
            .line_height(2.0)
            .align_x(Alignment::Start);

        let scroll: Scrollable<'_, Message, iced::Theme, iced::Renderer> = scrollable(
            column![text(&self.display_value)]
                .width(Length::Fill)
                .align_x(Alignment::Start),
        )
        .height(Length::Fill)
        .direction(scrollable::Direction::Vertical(
            scrollable::Scrollbar::default().width(5).scroller_width(5),
        ));

        let input_row_w_button = button(container(text("btn"))).on_press(
            Message::TerminalPaneMessage(TerminalPaneMessage::InputSubmit),
        );

        container(column![scroll, input_row, input_row_w_button]).into()
    }

    pub fn update(&mut self, message: TerminalPaneMessage) {
        match message {
            TerminalPaneMessage::InputChanged(value) => {
                self.input_value = value;
                self.char_num = self.input_value.chars().count() as u32;
            }
            TerminalPaneMessage::InputSubmit => {
                event!(Level::INFO, "w");
                self.reg_data(&(self.input_value.clone()));
                self.input_value.clear();
                self.char_num = 0;
            }
        }
    }
}
