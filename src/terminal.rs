use iced::widget::{button, scrollable, text, text_input, Scrollable};
use iced::widget::{column, container};
use iced::{Alignment, Element, Length};

use crate::Message;

#[derive(Debug, Clone)]
pub enum TerminalMessage {
    InputChanged(String),
    Submit,
}

#[derive(Default)]
pub struct Terminal {
    pub input_value: String,
    pub display_value: String,
}

impl Terminal {
    pub fn view(&self) -> Element<'_, Message> {
        // user input box
        let input_row = text_input("a", &self.input_value)
            .on_input(|value| Message::TerminalMessage(TerminalMessage::InputChanged(value)))
            .width(Length::Fill)
            .line_height(2.0)
            .align_x(Alignment::Start);

        // display box
        // let display_row = container(
        //     text(&self.display_value)
        //         .size(20)
        //         .width(Length::Fill)
        //         .height(Length::Fill)
        //         .align_x(Alignment::Start),
        // );

        let n_display_row = Scrollable::new(
            container(column![text(&self.display_value)])
                .width(Length::Fill)
                .padding([0, 8]),
        )
        .direction(scrollable::Direction::Vertical(
            scrollable::Scrollbar::default().width(5).scroller_width(5),
        ));

        // submit button
        let input_row_w_button = button(container(text("btn")))
            .on_press(Message::TerminalMessage(TerminalMessage::Submit));

        container(column![n_display_row, input_row, input_row_w_button]).into()
    }

    pub fn update(&mut self, message: TerminalMessage) {
        match message {
            TerminalMessage::InputChanged(value) => {
                self.input_value = value;
            }
            TerminalMessage::Submit => {
                self.display_value = self.input_value.clone();
                self.input_value.clear();
            }
        }
    }
}
