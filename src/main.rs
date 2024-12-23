use iced::{application, Settings, Element};

fn main() -> iced::Result {
    application(title, update, view).run()
}

enum Message {
    
}

fn title() -> String {
    String::from("My App")
}

fn update(_value: &mut u64, _message: Message) {
}

fn view(_value: &u64) -> Element<Message> {
    "hello, world".into()
}