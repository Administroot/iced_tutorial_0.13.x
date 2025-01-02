use iced::{
    widget::{column, text, radio},
    Element,
};

fn main() -> iced::Result {
    iced::application("My First App", MyApp::update, MyApp::view).run()
}

struct MyApp {
    state: String,
}

impl Default for MyApp {
    fn default() -> Self {
        MyApp::new()
    }
}

#[derive(Debug, Clone)]
enum Message {
    Choose(&str)
}

impl MyApp {
    fn new() -> Self {
        Self {
            state: String::new(),
        }
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Choose(s) => {
                self.state = s.to_string();
            }
        }
    }

    fn view(&self) -> Element<Message> {
        column!(
            radio("Choice A", &"A".to_string(), self.state.as_ref(), |s| {
                Message::
            })
        ).into()
    }
}
