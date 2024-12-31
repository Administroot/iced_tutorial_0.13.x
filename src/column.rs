use iced::{
    widget::{column, Column},
    Alignment, Element, Length,
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
        column![
            Column::with_children(vec![
                "Construct from the with_children function".into(),
                "another element".into()
            ]),
            Column::new()
                .push("Construct from the new function and the push method")
                .push("another element again"),
            column(vec!["Construct from function".into()]),
            column!["Construct from macro"],
            column!["With padding"].padding(20),
            column!["Different alignment"]
                .width(Length::Fill)
                .align_x(Alignment::Center),
            column!["Space between elements", "Space between elements"].spacing(20),
        ]
        .into()
    }
}
