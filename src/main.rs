use std::time::Duration;

use iced::{
    widget::{text, button, column},
    Element, Task,
};

fn main() -> iced::Result {
    iced::application(
        "controlling widgets by commands",
        MyApp::update,
        MyApp::view,
    )
    .run()
}

struct MyApp {
    state: String,
}

impl Default for MyApp {
    fn default() -> Self {
        MyApp::new().0
    }
}

#[derive(Debug, Clone)]
enum Message {
    Execute,
    Done,
}

impl MyApp {
    fn new() -> (Self, Task<Message>) {
        (
            Self {
                state: String::new(),
            },
            Task::none(),
        )
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Execute => {
                self.state = "Executing".into();
                // Executing an asynchronous function.
                return Task::perform(tokio::time::sleep(Duration::from_secs(1)), |_| {
                    Message::Done
                });
            },
            Message::Done => {
                self.state = "Done".into();
            }
        }
        Task::none()
    }

    fn view(&self) -> Element<Message> {
        column!(
            button("Execute").on_press(Message::Execute),
            text(self.state.as_str()),
        )
        .into()
    }
}