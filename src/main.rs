use iced::widget::{column, Column, Toggler, toggler};

pub fn main() -> iced::Result {
    iced::application("My app", update, view).run()
}

#[derive(Debug, Clone)]

enum MyAppMessage {
    _Increment,
}

#[derive(Default)]
struct State {
    _checkbox4: bool,
}

fn update(_value: &mut u64, _message: MyAppMessage) {}

fn view(_value: &u64) -> Column<MyAppMessage> {
    column![
        Toggler::new(Some("Construct from struct".into()), false, |_| {
            MyAppMessage::DoNothing
        }),
    ]
}
