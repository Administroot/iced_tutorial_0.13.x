use iced::{widget::{column, Column, Checkbox, checkbox, text::{LineHeight, Shaping}}, Font, font::Family};

pub fn main() -> iced::Result {
    iced::application("My app", update, view).run()
}

#[derive(Debug, Clone)]
enum MyAppMessage {
    DoNothing,
    Update4(bool),
    Update5(bool),
}

#[derive(Default)]
struct State{
    checkbox4: bool,
    checkbox5: bool,
}

fn update(state: &mut State, message: MyAppMessage) {
    match message {
        MyAppMessage::DoNothing => {},
        MyAppMessage::Update4(b) => {
            state.checkbox4 = b;
        },
        MyAppMessage::Update5(b) => {
            state.checkbox5 = b;
        },
    }
}

fn view(state: &State) -> Column<MyAppMessage> {
    column![
        Checkbox::new("Construct from struct", false),
        checkbox("Construct from function", false),
        checkbox("Enabled checkbox", false).on_toggle(|_| MyAppMessage::DoNothing),
        // Two ways of programming typical checkboxes
        checkbox("Functional checkbox", state.checkbox4).on_toggle(|b| MyAppMessage::Update4(b)),
        checkbox("Shorter parameter", state.checkbox5).on_toggle(MyAppMessage::Update5),
        checkbox("Larger box", false)
            .on_toggle(|_| MyAppMessage::DoNothing)
            .size(30),
        checkbox("Different icon", true)
            .on_toggle(|_| MyAppMessage::DoNothing)
            .icon(Icon {
                font: Font::DEFAULT,
                code_point: '*',
                size: None,
                line_height: LineHeight::default(),
                shaping: Shaping::default()
            }),
    ]
}
