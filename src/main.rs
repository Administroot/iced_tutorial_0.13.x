use iced::{
    event::{self, Status}, keyboard::{key::Named, Event::KeyPressed, Key}, time::{self, Duration}, widget::text, Element, Event, Subscription, Task
};

fn main() -> iced::Result {
    iced::application(
        "producing messages by timers",
        MyApp::update,
        MyApp::view,
    )
    .subscription(MyApp::subscription)
    .run()
}

struct MyApp {
    running: bool,
    pressed_key: String,
    seconds: u32,
}

impl Default for MyApp {
    fn default() -> Self {
        MyApp::new()
    }
}

#[derive(Debug, Clone)]
enum Message {
    KeyPressed(String),
    Update,
}

impl MyApp {
    fn new() -> Self {
        Self { pressed_key: String::new(), seconds: 0u32 }
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Update => {
                self.seconds += 1;
                return Task::none();
            }
        }
    }

    fn view(&self) -> Element<Message> {
        text(self.seconds).into()
    }

    fn subscription(&self) -> Subscription<Message> {
        let subscrkey = event::listen_with(|event, status, _window| match (event, status) {
            (
                Event::Keyboard(KeyPressed {
                    key: Key::Named(Named::Space),
                    ..
                }),
                Status::Ignored,
            ) => Some(Message::KeyPressed("Space".into())),
            _ => None,
        });

        if self.running {
            Subscription::batch(vec![
                
            ]
                
            )
        }
        time::every(Duration::from_secs(1)).map(|_| Message::Update)
    }
}