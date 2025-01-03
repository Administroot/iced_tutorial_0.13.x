use iced::{
    widget::{column, radio},
    Element,
};

fn main() -> iced::Result {
    iced::application("My First App", MyApp::update, MyApp::view).run()
}

struct MyApp {
    selection: Option<Choice>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Choice {
    A,
    B,
    C,
}

impl Default for MyApp {
    fn default() -> Self {
        MyApp::new()
    }
}

#[derive(Debug, Clone)]
enum Message {
    RadioSelected(Choice),
}

impl MyApp {
    fn new() -> Self {
        Self { selection: None }
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::RadioSelected(c) => {
                self.selection = Some(c);
            }
        }
    }

    fn view(&self) -> Element<Message> {
        column![
            radio(
                "Choice A",
                Choice::A,
                self.selection,
                Message::RadioSelected
            )
            .style(if self.selection == Some(Choice::A) {
                style::radio_selected
            } else {
                style::radio_unselected
            }),
            radio(
                "Choice B",
                Choice::B,
                self.selection,
                Message::RadioSelected
            )
            .style(if self.selection == Some(Choice::B) {
                style::radio_selected
            } else {
                style::radio_unselected
            }),
            radio(
                "Choice C",
                Choice::C,
                self.selection,
                Message::RadioSelected
            ),
        ]
        .into()
    }
}

mod style {
    use iced::{widget::radio, Background, Color, Theme};

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
