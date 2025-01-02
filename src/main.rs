use iced::{
    widget::{column, text}, Color, Element
};

fn main() -> iced::Result {
    iced::application("My First App", MyApp::update, MyApp::view)
        .run()
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
        column!(
            text("Ready?").style(text::Style(
                Color::from_rgb(1., 0.6, 0.2)
            )),
        ).into()
    }
}