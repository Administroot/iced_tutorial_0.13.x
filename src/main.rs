use iced::widget::{button, column, text, Column};

pub fn main() -> iced::Result {
    iced::application("A counter", update, view).run()
}

#[derive(Debug, Clone)]
enum Message {
    Increment,
}

fn update(_value: &mut u64, _message: Message) {
}

fn view(value: &u64) -> Message {
    // column![
    //     text(value),
    //     button("+").on_press(Message::Increment),
    // ]
}