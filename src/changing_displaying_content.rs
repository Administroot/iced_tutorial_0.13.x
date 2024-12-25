use iced::widget::{button, column, text, Column};

pub fn main() -> iced::Result {
    iced::application("A counter", update, view).run()
}

#[derive(Debug, Clone)]

enum Message {
    Increment,
}

fn update(value: &mut u64, message: Message) {
    match message {
        Message::Increment => *value += 1,
    }
}

fn view(value: &u64) -> Column<Message> {
    column![text(value), button("Increase").on_press(Message::Increment),]
}
