use iced::widget::{column, Column, Checkbox, checkbox};

pub fn main() -> iced::Result {
    iced::application("My app", update, view).run()
}

#[derive(Debug, Clone)]

enum MyAppMessage {
    DoNothing,
}

#[de]
struct State{
    checkbox4: bool,
}

fn update(_state: &mut State, message: MyAppMessage) {
    match message {
        MyAppMessage::DoNothing => {}
    }
}

fn view(state: &State) -> Column<MyAppMessage> {
    column![
        Checkbox::new("Construct from struct", false),
        checkbox("Construct from function", false),
        checkbox("Enabled checkbox", false).on_toggle(|_| MyAppMessage::DoNothing),
        checkbox("Functional checkbox", state.checkbox4).on_toggle(|b| MyAppMessage::Update4(b)),
    ]
}
