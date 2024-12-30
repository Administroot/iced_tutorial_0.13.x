use iced::{
    widget::{column, image, text, Image},
    Element,
    ContentFit,
};

fn main() -> iced::Result {
    iced::run("My First App", MyApp::update, MyApp::view)
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
            text("Construct from struct"),
            Image::new("ferris.png"),
            text("Construct from function"),
            image("ferris.png"),
            text("Different content fit"),
            image("ferris.png").content_fit(ContentFit::Cover)
        ).into()
    }
}
