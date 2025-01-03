use iced::{
    widget::{column, radio}, Element
};

fn main() -> iced::Result {
    iced::application("My First App", MyApp::update, MyApp::view).run()
}

struct MyApp {
    select: bool,
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
            select: false,
        }
    }

    fn update(&mut self, _message: Message) {
    }

    fn view(&self) -> Element<Message> {
        column![
            radio("Choice A", &"A".to_string(), Some("A"), |s| Message::Choose(
                s.to_string()
            ))
            .style(if self.select {
                    style::radio_selected
                } else {
                    style::radio_unselected
                }
            ),
        ].into()
    }
}

mod style {
    use iced::{widget::radio, Color, Theme, Background};

    pub fn radio_selected(_theme: &Theme, _status: radio::Status) -> radio::Style {
        radio::Style {
            text_color: Some(Color::from_rgb(0., 0., 1.)),
            background: Background::Color(Color::from_rgb(1., 1., 1.)),
            dot_color: Color::from_rgb(0., 0., 1.),
            border_width: 1.0,
            border_color: Color::from_rgb(0., 0., 1.),
        }
    }

    pub fn radio_unselected(_theme: &Theme, _status: radio::Status) -> radio::Style {
        radio::Style {
            text_color: Some(Color::from_rgb(0.5, 0.5, 0.5)),
            background: Background::Color(Color::from_rgb(1., 1., 1.)),
            dot_color: Color::from_rgb(0., 0., 1.),
            border_width: 1.0,
            border_color: Color::from_rgb(0., 0., 1.),
        }
    }
}