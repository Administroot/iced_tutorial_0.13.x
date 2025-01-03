use iced::{
    theme, widget::{column, radio, Radio}, Element, Theme, Color
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
            radio("Choice A", "A", Some("A"), |s| Message::Choose(
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
    use iced::{widget::{radio::{self, default}, Radio}, Color, Theme};

    pub fn radio_selected(theme: &Theme) -> radio::Style {
        radio::Style {
            text_color: Some(Color::from_rgb(0., 0., 1.)),
            background: Radio::DE,
            dot_color: todo!(),
            border_width: todo!(),
            border_color: todo!(),
        }
    }
}