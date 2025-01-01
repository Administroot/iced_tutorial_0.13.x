use iced::{
    settings, theme, widget::{column, text}, Element, Settings
};

fn main() -> iced::Result {
    // iced::run("My First App", MyApp::update, MyApp::view)
    // iced::daemon("changing_themes", MyApp)
    iced::a
}

struct MyApp {
    _state: String,
}

impl Default for MyApp {
    fn default() -> Self {
        MyApp::new()
    }
}

#[derive(Debug, Clone)]
enum Message {
    _Message1,
}

impl MyApp {
    fn new() -> Self {
        Self {
            _state: String::new(),
        }
    }

    fn update(&mut self, _message: Message) {
        todo!()
    }

    fn view(&self) -> Element<Message> {
        column!(text("Hello World!".to_string()),).into()
    }

    fn theme(&self) -> iced::Theme {
        iced::Theme::Dark
        // or
        // iced::Theme::Li