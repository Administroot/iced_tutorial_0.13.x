use iced::{
    executor, widget::{button, column, text_input}, Element, Task
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
        MyApp::new()
    }
}

#[derive(Debug, Clone)]
enum Message {
    EditText,
    UpdateText(String),
}

impl MyApp {
    fn new() -> Self {
        Self {
            some_text: String::new(),
        }
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::EditText => {
                text_input::focus(MY_TEXT_ID)
            },
            Message::UpdateText(s) => self.some_text = s,
        }
    }

    fn view(&self) -> Element<Message> {
        column!(
            button("Edit text").on_press(Message::EditText),
            text_input("", &self.some_text)
                // Sets the Id of the TextInput.
                .id(text_input::Id::new(MY_TEXT_ID))
                .on_input(Message::UpdateText),
        )
        .into()
    }
}
