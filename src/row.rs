use iced::{
    widget::{column, row, Row},
    Alignment, Element, Length,
};

fn main() -> iced::Result {
    iced::run("Row", MyApp::update, MyApp::view)
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
            Row::with_children(vec![
                "Construct from the with_children function".into(),
                "another element".into()
            ]),
            Row::new()
                .push("Construct from the new function and the push method")
                .push("another element again"),
            row(vec!["Constuct from function".into()]),
            row!["Construct from macro"],
            row!["With padding"].padding(20),
            row!["space with elements", "space with elements"].spacing(20),
            row!["Different alignment"]
                .height(Length::Fill)
                .align_y(Alignment::Center),
        )
        .into()
    }
}
