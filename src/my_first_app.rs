use iced::widget::{column, Column};

pub fn main() -> iced::Result {
    iced::application("My app", update, view).run()
}

#[derive(Debug, Clone)]

enum Message {
    _Increment,
}

fn update(_value: &mut u64, _message: Message) {}

fn view(_value: &u64) -> Column<Message> {
    column!["hello, world"]
}
