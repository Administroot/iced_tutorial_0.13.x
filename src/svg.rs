use iced::{
    widget::{column, svg, svg::Handle, text, Svg},
    ContentFit, Element,
};

fn main() -> iced::Result {
    iced::run("svg", MyApp::update, MyApp::view)
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
            Svg::from_path("pic.svg"),
            text("Construct from function"),
            svg(Handle::from_path("pic.svg")),
            text("Different content fit"),
            svg(Handle::from_path("pic.svg")).content_fit(ContentFit::None),
        )
        .into()
    }
}
