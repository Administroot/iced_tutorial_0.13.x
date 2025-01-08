use iced::{
    widget::{button, row},
    window, Element, Task,
};

fn main() -> iced::Result {
    iced::application("closing the window on demand", MyApp::update, MyApp::view).run()
}

struct MyApp {}

impl Default for MyApp {
    fn default() -> Self {
        MyApp::new().0
    }
}

#[derive(Debug, Clone)]
enum Message {
    CloseWindow,
}

impl MyApp {
    fn new() -> (Self, Task<Message>) {
        (Self {}, Task::none())
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::CloseWindow => return window::get_oldest().and_then(window::close),
        }
    }

    fn view(&self) -> Element<Message> {
        row!(button("Close window").on_press(Message::CloseWindow),).into()
    }
}
