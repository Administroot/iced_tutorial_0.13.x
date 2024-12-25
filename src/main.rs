/* A widget for searching and selecting a single value from a list of options.
This widget is composed by a TextInput that can be filled with the text to search
for corresponding values from the list of options that are displayed as a Menu. */

use iced::widget::{column, combo_box, combo_box::State, Column, ComboBox};

pub fn main() -> iced::Result {
    iced::application("My app", update, view).run()
}

#[derive(Debug, Clone)]

enum MyAppMessage {
    DoNothing,
}

#[derive(Default)]
struct MyApp {
    state1: State<u32>,
    state2: State<u32>,
}

fn update(_state: &mut MyApp, message: MyAppMessage) {
    match message {
        MyAppMessage::DoNothing => {}
    }
}

fn view(myapp: &MyApp) -> Column<MyAppMessage> {
    column![
        ComboBox::new(&myapp.state1, "Construct from struct", None, |_| {
            MyAppMessage::DoNothing
        }),
        combo_box(&myapp.state2, "Construct from function", None, |_| {
            MyAppMessage::DoNothing
        }),
    ]
    .into()
}
