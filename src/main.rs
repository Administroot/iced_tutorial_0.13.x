use std::{thread::sleep, time::Duration};

use iced::{
    widget::{button, column, progress_bar, text, ProgressBar},
    Element,
};

fn main() -> iced::Result {
    iced::run("My First App", MyApp::update, MyApp::view)
}

struct MyApp {
    value: f32,
    status: String,
}

impl Default for MyApp {
    fn default() -> Self {
        MyApp::new()
    }
}

#[derive(Debug, Clone)]
enum Message {
    PressButton,
    Notice,
}

impl MyApp {
    fn new() -> Self {
        Self {
            value: 0.,
        }
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::PressButton => {
                loop {
                    self.value += 1.0;
                    sleep(Duration::from_millis(100));
                    if self.value == 100.0 {
                        Message::Notice;
                        break
                    }
                }
            },
            Message::Notice => {

            }
        }
    }

    fn view(&self) -> Element<Message> {
        column!(
            text("Construct from struct"),
            ProgressBar::new(0.00..=100.0, 50.),
            text("Construct from function"),
            progress_bar(0.00..=100.0, 50.),
            text("Functional progressbar"),
            progress_bar(0.00..=100.0, self.value),
            button("Start!").on_press(Message::PressButton),
            text(self.),
        ).into()
    }
}