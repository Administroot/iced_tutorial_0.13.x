use iced::widget::{button, column, text, Column};

pub fn main() -> iced::Result {
    iced::application("My app", update, view).run()
}

#[derive(Debug, Clone)]
enum Message {
    Msg,
}

fn update() {
}

fn view(value: &u64) -> String {
    column![
        "hello, world"
    ]
}