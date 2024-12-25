use iced::{widget::{column, Column, pick_list, PickList, text::Shaping, pick_list::Handle}, font::Family, Font};

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
        // A typical pick list
        pick_list(
            ["Other choices 1", "Other choices 2"]
                .map(|s| s.to_string())
                .to_vec(),
            state.pick_list_3.clone(),
            |s| MyAppMessage::Update3(s)
        ).placeholder("Functional pick list"),
        pick_list(vec!["A", "B", "C"], None::<&str>, |_| {
            MyAppMessage::DoNothing
        })
        .placeholder("Placeholder"),
        // Personalization
        pick_list(vec!["Different font"], Some("Different font"), |_| {
            MyAppMessage::DoNothing
        })
        .font(Font {
            family: Family::Fantasy,
            ..Font::DEFAULT
        }),
        pick_list(vec!["Larger text"], Some("Larger text"), |_| {
            MyAppMessage::DoNothing
        })
        .text_size(24),
        pick_list(
            vec!["Special character 😊"],
            Some("Special character 😊"),
            |_| MyAppMessage::DoNothing
        )
        .text_shaping(Shaping::Advanced),
        pick_list(vec!["With padding"], Some("With padding"), |_| {
            MyAppMessage::DoNothing
        })
        .padding(20),
        pick_list(vec!["Different handle"], Some("Different handle"), |_| {
            MyAppMessage::DoNothing
        })
        .handle(Handle::Arrow {
            size: Some(Pixels(24.))
        }),
    ].into()
}
