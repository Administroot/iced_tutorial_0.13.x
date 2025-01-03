use iced::{
    theme, widget::{column, radio, Radio}, Element, Theme, Color
};

fn main() -> iced::Result {
    iced::application("My First App", MyApp::update, MyApp::view).run()
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
    Choose(String)
}

impl MyApp {
    fn new() -> Self {
        Self {
            _state: String::new(),
        }
    }

    fn update(&mut self, _message: Message) {
    }

    fn view(&self) -> Element<Message> {
        let is_selected: bool = self.select;
        column![
            radio("Choice A", "A", Some("A"), |s| Message::Choose(
                s.to_string()
            ))
            .style(if is_selected {
                Color::from_rgb(0., 0., 1.)
            } else {
                Color::from_rgb(0.5, 0.5, 0.5)
            }
            ),
        ].into()
    }
}