use iced::{
     event, event::Status, widget::{column, text, tooltip::Position}, Element, Event, Subscription, Task, Point
};

fn main() -> iced::Result {
    iced::application("My First App", MyApp::update, MyApp::view).run()
}

struct MyApp {
    mouse_point: Point,
}

impl Default for MyApp {
    fn default() -> Self {
        MyApp::new()
    }
}

#[derive(Debug, Clone)]
enum Message {
    PointUpdated(Position)
}

impl MyApp {
    fn new() -> Self {
        Self {
            mouse_point: Point::ORIGIN
        }
    }

    fn update(&mut self, message: Message) -> Task<Message>{
        match message {
            Message::PointUpdated(p) => {
                self.mouse_point = p;
            }
            Task::None()
        }
    }

    fn view(&self) -> Subscription<Message> {
        event::listen_with(|event, status, _window| { match (event, status) {
                (Event::Mouse(Event::CursorMoved { position }), Status::Ignored)
                | (Event::Touch(Event::FingerMoved {position, ..}), Status::Ignored) => {
                    Some(Message::PointUpdated)
                }
                _ => None
            }
        })
    }
}