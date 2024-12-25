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

fn update(_state: &mut State, message: MyAppMessage) {
    match message {
        MyAppMessage::DoNothing => {},
    }
}

fn view(_state: &State) -> Column<MyAppMessage> {
    column![
        Toggler::new(false).label("Construct from struct").on_toggle(|_| MyAppMessage::DoNothing),
        toggler(false).label("Functional togg")
    ]
}
