use iced::{
    widget::{
        column, text, Slider, slider
    },
    Element
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
enum MyAppMessage {
    DoNothing,
}

impl MyApp {
    fn new() -> Self {
        Self {
            _state: String::new(),
        }
    }

    fn update(&mut self, message: MyAppMessage) {
        match message {
            MyAppMessage::DoNothing => {},
        }
    }

    fn view(&self) -> Element<MyAppMessage> {
        column!(
            text("Construct from struct"),
            Slider::new(0..=100, 50, |_| MyAppMessage::DoNothing),
        ).into()
    }
}
