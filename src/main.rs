use iced::widget::{column, Column, pick_list, PickList};

pub fn main() -> iced::Result {
    iced::application("My app", update, view).run()
}

#[derive(Debug, Clone)]

enum MyAppMessage {
    DoNothing,
    Update3(String),
}

#[derive(Default)]
struct State {
    pick_list_3: Option<String>, 
}

fn update(state: &mut State, message: MyAppMessage) {
    match message {
        MyAppMessage::DoNothing => {},
        MyAppMessage::Update3(s) => {
            state.pick_list_3 = Some(s);
        }
    }
}

fn view(state: &State) -> Column<MyAppMessage> {
    column![
        PickList::new(
            vec!["Construct from struct"],
            Some("Construct from struct"),
            |_| MyAppMessage::DoNothing
        ),
        pick_list(
            vec!["Construct from function"],
            Some("Construct from function"),
            |_| MyAppMessage::DoNothing
        ),
        pick_list(
            ["Functional pick list", "Other choices 1", "Other choices 2"]
                .map(|s| s.to_string())
                .to_vec(),
            S
            state.pick_list_3.clone(),
            |s| MyAppMessage::Update3(s)
        ),
    ].into()
}
