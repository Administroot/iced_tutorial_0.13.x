use iced::{
    widget::{button, row, text_input},
    window, Element, Size, Task,
};

fn main() -> iced::Result {
    iced::application(
        "changing the window dynamically",
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
            }
            Message::UpdateHeight(h) => {
                self.height = h;
            }

            Message::ResizeWindow => {
                // If don't define var "width" and "height", error 'E0521' will occur.
                let width = self.width.parse().unwrap();
                let height = self.height.parse().unwrap();

                // Id in Task<Option<Id>> will be delivered to Task::and_then() method, then to the closure.
                return window::get_oldest()
                    .and_then(move |window| window::resize(window, Size::new(width, height)));
            }
        }
        Task::none()
    }

    fn view(&self) -> Element<Message> {
        row!(
            text_input("Width", &self.width).on_input(Message::UpdateWidth),
            text_input("Height", &self.height).on_input(Message::UpdateHeight),
            button("Resize window").on_press(Message::ResizeWindow),
        )
        .into()
    }
}
