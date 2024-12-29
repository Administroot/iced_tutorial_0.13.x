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
}

impl MyApp {
    fn new() -> Self {
        Self {
            value: 0.,
            status: String::new(),
        }
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::PressButton => {
                self.value = 0.0;
                loop {
                    self.value += 1.0;
                    sleep(Duration::from_secs(1u64));
                    self.status = self.value.to_string();
                    if self.value > 100.0 {
                        self.status = String::from("Done");
                        break
                    }
                    println!("{}", self.value);
                }
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
            text(&self.status).center(),
        ).into()
    }
}