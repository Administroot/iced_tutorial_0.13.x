use iced::{widget::mouse_area, Element};

fn main() -> iced::Result {
    iced::application(
        "on pressed released of some widgets",
        MyApp::update,
        MyApp::view,
    )
    .run()
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
    Pressed,
    Released,
}

impl MyApp {
    fn new() -> Self {
        Self {
            state: "Start".into(),
        }
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Pressed => {
                self.state = "Pressed".into();
            }
            Message::Released => {
                self.state = "Released".into();
            }
        }
    }

    fn view(&self) -> Element<Message> {
        mouse_area(self.state.as_str())
            .on_press(Message::Pressed)
            .on_release(Message::Released)
            .into()
    }
}
