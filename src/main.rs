use iced::{
    widget::{button::text, column, progress_bar, text, ProgressBar},
    Element,
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
            ProgressBar::new(0.00..=100.0, 50.),
            text("Construct from function"),
            progress_bar(0.00..=100.0, 50.),
            text("Functional progressbar"),
            
        ).into()
    }
}