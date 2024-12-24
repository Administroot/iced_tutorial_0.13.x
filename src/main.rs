use iced::widget::{column, Column, TextInput, text_input};

pub fn main() -> iced::Result {
    iced::application("My app", update, view).run()
}

#[derive(Debug, Clone)]

enum MyAppMessage {
    Update3(String),
    Update4(String),
}

#[derive(Default)]
struct State{
    text3: String,
    text4: String,
}

fn update(state: &mut State, message: MyAppMessage) {
    match message {
        MyAppMessage::Update3(s) => {state.text3 = s;}
        MyAppMessage::Update4(s) => {state.text4 = s;}
    }
}

fn view(state: &State) -> Column<MyAppMessage> {
    column![
        text_input("Construct from function", ""),
        TextInput::new("Construct from struct", ""),
        text_input("Enabled text input", state.text3.as_str())
            .on_input(|s| MyAppMessage::Update3(s)),
        text_input("Shorter on_input", state.text4.as_str()).on_input(MyAppMessage::Update4),
    ]
    .into()
}
