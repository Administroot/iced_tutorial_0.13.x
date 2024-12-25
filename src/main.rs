use iced::widget::{column, Column};

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

fn update(_state: &mut State, _message: MyAppMessage) {}

fn view(_state: &State) -> Column<MyAppMessage> {
    column!["hello, world"].into()
}
