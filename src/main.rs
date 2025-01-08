use iced::{
    time::{self, Duration},
    widget::text,
    Element, Subscription, Task,
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
    seconds: u32,
}

impl Default for MyApp {
    fn default() -> Self {
        MyApp::new()
    }
}

#[derive(Debug, Clone)]
enum Message {
    Update,
}

impl MyApp {
    fn new() -> Self {
        Self { seconds: 0u32 }
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
        time::every(Duration::from_secs(1)).map(|_| Message::Update)
    }
}