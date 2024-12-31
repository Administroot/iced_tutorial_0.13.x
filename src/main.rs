use iced::{
    widget::{button, column, row, Space, horizontal_space},
    Element, Length,
};

fn main() -> iced::Result {
    iced::run("Space", MyApp::update, MyApp::view)
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
            row![
                button("Horizontal space 1A"),
                Space::with_width(50),
                button("Horizontal space 1B"),
            ],
            row![
                // Button 2A on the far left
                button("Horizontal space 2A"),
                Space::with_width(Length::Fill),
                // Button 2B on the far right
                button("Horizontal space 2B"),
            ],
            // The same 
            row![
                button("Horizontal space 3A"),
                horizontal_space(),
                button("Horizontal space 3B"),
            ]
        ).into()
    }
}
