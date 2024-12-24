use iced::widget::{column, Column, TextInput, text_input};

pub fn main() -> iced::Result {
    iced::application("My app", update, view).run()
}

#[derive(Debug, Clone)]

enum MyAppMessage {
    Update3(String),
}

#[derive(Default)]
struct State{
    text3: String,
}

fn update(state: &mut State, message: MyAppMessage) {
    match message {
        MyAppMessage::Update3(s) => {state.text3 = s;}
    }
}

fn view(state: &State) -> Column<MyAppMessage> {
    column![
        text_input("Construct from function", ""),
        TextInput::new("Construct from struct", ""),
        text_input("Enabled text input", state.text3.as_str())
            .on_input(|s| MyAppMessage::Update3(s)),
    ]
    .into()
}
