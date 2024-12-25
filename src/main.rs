use iced::widget::{column, pane_grid::state, toggler, Column, Toggler};

pub fn main() -> iced::Result {
    iced::application("My app", update, view).run()
}

#[derive(Debug, Clone)]

enum MyAppMessage {
    DoNothing,
}

#[derive(Default)]
struct State {
    _checkbox4: bool,
}

fn update(_value: &mut u64, _message: MyAppMessage) {}

fn view(state: &State) -> Column<MyAppMessage> {
    column![
        Toggler::new(false).label("Construct from struct"),
    ]
}
