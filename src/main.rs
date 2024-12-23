use iced::{Application, Settings};

fn main() -> iced::Result {
    MyApp::Application::run("My App", update, )
}


fn update(value: &mut u64, _message: Message) {
}

struct MyApp;

impl Sandbox for MyApp {
    type Message = ();

    fn new() -> Self {
        Self
    }

    fn title(&self) -> String {
        String::from("My App")
    }

    fn update(&mut self, _message: Self::Message) {}

    fn view(&self) -> iced::Element<Self::Message> {
        "Hello World!".into()
    }
}