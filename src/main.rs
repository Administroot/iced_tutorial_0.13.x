use iced::{
    widget::{button, column, text_input}, Element, Task,
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
    some_text: String,
}

impl Default for MyApp {
    fn default() -> Self {
        MyApp::new().0
    }
}

#[derive(Debug, Clone)]
enum Message {
    UpdateText(String),
    SelectAll,
}

impl MyApp {
    fn new() -> (Self, Task<Message>) {
        (
            Self {
                some_text: String::new(),
            },
            Task::none(),
        )
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::UpdateText(s) => self.some_text = s,
            Message::SelectAll => {}
        }
        Task::none()
    }

    fn view(&self) -> Element<Message> {
        column!(
            // button("Edit text").on_press(Message::EditText),
            text_input("", &self.some_text)
                // Sets the Id of the TextInput.
                .id(text_input::Id::new(MY_TEXT_ID))
                .on_input(Message::UpdateText),
            button("Select all").on_press(Message::)
        )
        .into()
    }
}
