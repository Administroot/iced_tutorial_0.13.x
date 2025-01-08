use iced::{
     event, event::Status, Event, Subscription, Task, Point, mouse::Event::CursorMoved,
    touch::Event::FingerMoved,
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
    PointUpdated(Point)
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
                return Task::none();
            }
        }
    }

    fn view(&self) -> Subscription<Message> {
        event::listen_with(|event, status, _window| { match (event, status) {
                // Using mouse
                (Event::Mouse(CursorMoved { position }), Status::Ignored)
                // Or using touchboard
                | (Event::Touch(FingerMoved {position, ..}), Status::Ignored) => {
                    Some(Message::PointUpdated(position))
                }
                _ => None
            }
        })
    }
}