use iced::{
    widget::{column, text},
    window, Element, Point, Size,
};

fn main() -> iced::Result {
    let window_setting = window::settings::Settings {
        size: Size {
            width: 70.,
            height: 30.,
        },
        position: window::Position::Specific(Point { x: 50., y: 60. }),
        ..Default::default()
    };
    iced::application("initializing a different window", MyApp::update, MyApp::view)
        .window(window_setting)
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
        column!(text("Hello World!".to_string()),).into()
    }
}
