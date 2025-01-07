use iced::{
    widget::{button, column, text_input}, window, Element, Size, Task
};

const MY_TEXT_ID: &str = "my_text";

fn main() -> iced::Result {
    iced::application(
        "controlling widgets by commands",
        MyApp::update,
        MyApp::view,
    )
    .run()
}

struct MyApp {
    width: String,
    height: String,
}

impl Default for MyApp {
    fn default() -> Self {
        MyApp::new().0
    }
}

#[derive(Debug, Clone)]
enum Message {
    UpdateWidth(String),
    UpdateHeight(String),
    ResizeWindow,
}

impl MyApp {
    fn new() -> (Self, Task<Message>) {
        (
            Self {
                width: String::new(),
                height: String::new(),
            },
            Task::none(),
        )
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::UpdateWidth(w) => {
                self.width = w;
            },
            Message::UpdateHeight(h) => {
                self.height = h;
            },
            Message::ResizeWindow => {
                return window::get_oldest().and_then(
                    move |window| {
                        window::resize(window, Size::new(self.width.parse().unwrap(), self.height.parse().unwrap()))
                    }
                );
            },
        }
        Task::none()
    }

    fn view(&self) -> Element<Message> {
        column!(
            text_input("Width", &self.width)
                .on_input(Message::UpdateWidth),
            text_input("Height", &self.height)
                .on_input(Message::UpdateHeight),
            button("Resize window").on_press(Message::ResizeWindow),
        )
        .into()
    }
}
