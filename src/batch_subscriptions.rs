use iced::{
    event::{self, Status},
    keyboard::{key::Named, Event::KeyPressed, Key},
    time::{self, Duration},
    widget::text,
    Element, Event, Subscription, Task,
};

fn main() -> iced::Result {
    iced::application("batch subscriptions", MyApp::update, MyApp::view)
        .subscription(MyApp::subscription)
        .run()
}

struct MyApp {
    running: bool,
    seconds: u32,
}

impl Default for MyApp {
    fn default() -> Self {
        MyApp::new()
    }
}

#[derive(Debug, Clone)]
enum Message {
    StartOrStop,
    Update,
}

impl MyApp {
    fn new() -> Self {
        Self {
            running: false,
            seconds: 0u32,
        }
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::StartOrStop => {
                self.running = !self.running;
            }
            Message::Update => {
                self.seconds += 1;
            }
        }
        Task::none()
    }

    fn view(&self) -> Element<Message> {
        text(self.seconds).into()
    }

    fn subscription(&self) -> Subscription<Message> {
        let subscr_key = event::listen_with(|event, status, _window| match (event, status) {
            (
                Event::Keyboard(KeyPressed {
                    key: Key::Named(Named::Space),
                    ..
                }),
                Status::Ignored,
            ) => Some(Message::StartOrStop),
            _ => None,
        });

        if self.running {
            Subscription::batch(vec![
                subscr_key,
                time::every(Duration::from_secs(1)).map(|_| Message::Update),
            ])
        } else {
            subscr_key
        }
    }
}
