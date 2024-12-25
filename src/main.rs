use iced::widget::{column, Column, combo_box, ComboBox, ComboBox::State};

pub fn main() -> iced::Result {
    iced::application("My app", update, view).run()
}

#[derive(Debug, Clone)]

enum MyAppMessage {
    DoNothing,
}

#[derive(Default)]
struct MyApp {
    state1: State(u32),
    state2:State(u32),
}

fn update(_state: &mut MyApp, message: MyAppMessage) {
    match message {
        MyAppMessage::DoNothing => {};
    }
}

fn view(_state: &MyApp) -> Column<MyAppMessage> {
    column![
        ComboBox::new(&self.state1, "Construct from struct", None, |_| {
            MyAppMessage::DoNothing
        }),
        combo_box(&self.state2, "Construct from function", None, |_| {
            MyAppMessage::DoNothing
        }),
    ].into()
}
