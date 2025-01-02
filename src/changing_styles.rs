use iced::{
    widget::{button, column, row, text},
    Color, Element,
};

fn main() -> iced::Result {
    iced::application("changing_styles", MyApp::update, MyApp::view)
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
                    .style(button::secondary)
                    .on_press(Message::DummyMessage),
                button("Go!~~")
                    .style(button::primary)
                    .on_press(Message::DummyMessage)
            ]
        )
        .into()
    }
}
