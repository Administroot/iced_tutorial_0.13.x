use iced::{
    event::{self, Status},
    mouse::Event::CursorMoved,
    touch::Event::FingerMoved,
    widget::text,
    Element, Event, Point, Subscription, Task,
};

fn main() -> iced::Result {
    iced::application(
        "producing_messages_by_mouse_events",
        MyApp::update,
        MyApp::view,
    )
    .subscription(MyApp::subscription)
    .run()
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
    PointUpdated(Point),
}

impl MyApp {
    fn new() -> Self {
        Self {
            mouse_point: Point::ORIGIN,
        }
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::PointUpdated(p) => {
                self.mouse_point = p;
                return Task::none();
            }
        }
    }

    fn view(&self) -> Element<Message> {
        text(format!("{:?}", self.mouse_point)).into()
    }

    fn subscription(&self) -> Subscription<Message> {
        event::listen_with(|event, status, _window| {
            match (event, status) {
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
