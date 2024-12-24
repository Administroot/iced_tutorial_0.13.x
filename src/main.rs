use iced::widget::{button, column, Button, Column};

pub fn main() -> iced::Result {
    iced::application("My app", update, view).run()
}

#[derive(Debug, Clone)]

enum Message {
    DoSomething,
}

fn update(_value: &mut u64, _message: Message) {}

fn view(_value: &u64) -> Column<Message> {
    column![
        Button::new("Disabled button"),
        button("Construct from function"),
        button("Enabled button").on_press(Message::DoSomething),
        button("With padding").padding(20),
    ]
    .into()
}
