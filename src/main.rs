use iced::{
    event::{self, Status}, widget::{column, text}, Element, Event, Subscription
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

    fn view(&self) -> Subscription<Message> {
        event::listen_with(|event, status| { match (event, status) {
                (Event::Mouse(Event::CursorMoved { position }), Status::Ignored)
                | (Event::Touch(FingerM))
            }
        })
    }
}