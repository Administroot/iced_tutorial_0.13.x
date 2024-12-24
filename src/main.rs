use iced::widget::{text, column, text_input, Column, TextInput};

pub fn main() -> iced::Result {
    iced::application("My app", update, view).run()
}

#[derive(Debug, Clone)]

enum MyAppMessage {
    Update3(String),
    Update4(String),
    Update5(String),
    Update6(String),
    Paste5(String),
    Submit6,
}

#[derive(Default)]
struct State{
    text3: String,
    text4: String,
    text5: String,
    text6: String,
    info5: String,
    info6: String,
}

fn update(state: &mut State, message: MyAppMessage) {
    match message {
        MyAppMessage::Update3(s) => {state.text3 = s;}
        MyAppMessage::Update4(s) => {state.text4 = s;}
        MyAppMessage::Update5(s) => {
            state.text5 = s;
            state.info5 = "".into();
        }
        MyAppMessage::Paste5(s) => {
            state.text5  = s;
            state.info5 = "Pasted".into();
        }
        MyAppMessage::Update6(s) => {
            state.text6 = s;
            state.info6 = "".into();
        },
        MyAppMessage::Submit6 => todo!(),
    }
}

fn view(state: &State) -> Column<MyAppMessage> {
    column![
        text_input("Construct from function", ""),
        TextInput::new("Construct from struct", ""),
        text_input("Enabled text input", state.text3.as_str())
            .on_input(|s| MyAppMessage::Update3(s)),
        text_input("Shorter on_input", state.text4.as_str()).on_input(MyAppMessage::Update4),

        // A line of pasted text will be displayed below the text input
        text_input("Press Ctrl/Cmd + V", state.text5.as_str())
                .on_input(MyAppMessage::Update5)
                .on_paste(MyAppMessage::Paste5),
        text(state.info5.as_str()),

        text_input("Press enter", state.text6.as_str())
                .on_input(MyAppMessage::Update6)
                .on_submit(MyAppMessage::Submit6),
        text(state.info6.as_str()),
    ]
    .into()
}
