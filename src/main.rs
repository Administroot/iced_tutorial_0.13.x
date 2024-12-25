use iced::widget::{column, Column, Checkbox, checkbox};

pub fn main() -> iced::Result {
    iced::application("My app", update, view).run()
}

#[derive(Debug, Clone)]

enum MyAppMessage {
    DoNothing,
}

fn update(_value: &mut u64, message: MyAppMessage) {
    match message {
        
    }
}

fn view(_value: &u64) -> Column<MyAppMessage> {
    column![Checkbox::new("Construct from struct", false),
            checkbox("Construct from function", false),
            checkbox("Enabled checkbox", false).on_toggle(|_| MyAppMessage::DoNothing),]
}
