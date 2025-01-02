use iced::{
    theme, widget::{column, row, text, button}, Color, Element
};

fn main() -> iced::Result {
    iced::application("My First App", MyApp::update, MyApp::view)
        .theme(MyApp::theme)
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
    DummyMessage,
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

    fn theme(&self) -> iced::Theme {
        iced::Theme::Dark
    }

    fn view(&self) -> Element<Message> {
        column!(
            text("Ready?").color(Color::from_rgb(1., 0.6, 0.2)),
            row![
                button("Cancel")
                    .style()
                    .on_press(Message::DummyMessage)
            ]
        ).into()
    }
}