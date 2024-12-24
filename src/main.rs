use iced::{
    alignment::{Horizontal, Vertical},
    font::Family,
    widget::{column, Column, text, text::Shaping, Text},
    Font, Length
};

pub fn main() -> iced::Result {
    iced::application("My app", update, view).run()
}

#[derive(Debug, Clone)]

enum Message {
    _Increment,
}

fn update(_value: &mut u64, _message: Message) {}

fn view(_value: &u64) -> Column<Message> {
column![
        Button::new("Disabled button"),
        button("Construct from function"),
        button("Enabled button").on_press(MyAppMessage::DoSomething),
        button("With padding").padding(20),
    ]
    .into()
}
