use iced::{mouse::Button, widget::{button, pane_grid::Content, Column, Text}, Theme};
use iced::widget::text;
use iced::widget::column;
use iced::run;

#[derive(Default)]
struct Counter {
    value: i64,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Increment,
    Decrement,
}

impl Counter {
    fn update(&mut self, message: Message) {
        match message {
            Message::Increment => {self.value += 1}
            Message::Decrement => {self.value -= 1}
        }
    }

    fn view(&self) -> Column<Message> {
        column![
            button("+").on_press(Message::Increment),
            text(self.value),
            button("-").on_press(Message::Decrement),
        ]
    }
}

pub fn main() -> iced::Result {
    let mut counter = Counter::default();
    let interface = counter.view();
    iced::run("test", Counter::update, Counter::view)
}

#[test]
fn it_counts_properly() {
    let mut counter = Counter { value: 0 };

    counter.update(Message::Increment);
    counter.update(Message::Increment);
    counter.update(Message::Decrement);

    assert_eq!(counter.value, 1);
}
