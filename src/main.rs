use iced::widget::{column, Column, Radio, radio, row, text};

pub fn main() -> iced::Result {
    iced::application("My app", update, view).run()
}

#[derive(Debug, Clone)]

enum MyAppMessage {
    DoNothing,
    Update3(u32),
}

#[derive(Default)]
struct State {
    radio3: Option<u32>,
}

fn update(state: &mut State, message: MyAppMessage) {
    match message {
        MyAppMessage::DoNothing => {},
        MyAppMessage::Update3(u) => {
           state.radio3 = Some(u); 
        }
    }
}

fn view(state: &State) -> Column<MyAppMessage> {
    column![
        Radio::new("Construct from struct", 0, None, |_| {
            MyAppMessage::DoNothing
        }),
        radio("Construct from function", 0, None, |_| {
            MyAppMessage::DoNothing
        }),
        row![
            text("Functional radio"),
            radio("A", 1, state.radio3, |i| MyAppMessage::Update3(i)),
            radio("B", 2, state.radio3, |i| MyAppMessage::Update3(i)),
            radio("C", 3, state.radio3, |i| MyAppMessage::Update3(i)),
        ],
        row![
            text("Radio of String values"),
            radio("A", &"a".to_string(), self.radio4.as_ref(), |s| {
                MyAppMessage::Update4(s.into())
            }),
            radio("B", &"b".to_string(), self.radio4.as_ref(), |s| {
                MyAppMessage::Update4(s.into())
            }),
            radio("C", &"c".to_string(), self.radio4.as_ref(), |s| {
                MyAppMessage::Update4(s.into())
            }),
        ],
    ]
}
