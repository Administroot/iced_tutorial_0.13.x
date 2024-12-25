use iced::widget::{column, toggler, Column, Toggler};

pub fn main() -> iced::Result {
    iced::application("My app", update, view).run()
}

#[derive(Debug, Clone)]

enum MyAppMessage {
    DoNothing,
    Update3(bool),
}

#[derive(Default)]
struct State {
    toggler3: bool,
}

fn update(state: &mut State, message: MyAppMessage) {
    match message {
        MyAppMessage::DoNothing => {},
        MyAppMessage::Update3(b) => {
            state.toggler3 = b;
        }
    }
}

fn view(state: &State) -> Column<MyAppMessage> {
    column![
        Toggler::new(false).label("Construct from struct").on_toggle(|_| MyAppMessage::DoNothing),
        toggler(false).label("Construct from function").on_toggle(|_| MyAppMessage::DoNothing),
        toggler(state.toggler3).label("Functional toggler").on_toggle(|b| MyAppMessage::Update3(b))
    ]
}
