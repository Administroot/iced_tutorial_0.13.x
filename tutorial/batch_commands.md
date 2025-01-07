# Batch Commands

Sometimes, we want to execute two or more `Commands` at the same time.
In this case, we can use `Command::batch` function.

```rust
use iced::{
    widget::{button, column, text_input},
    Element, Task,
};

const MY_TEXT_ID: &str = "my_text";

fn main() -> iced::Result {
    iced::application(
        "batch_commands",
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
            Message::SelectAll => {
                return Task::batch(vec![
                    // Produces a Task that focuses the TextInput with the given Id.
                    text_input::focus(text_input::Id::new(MY_TEXT_ID)),
                    // Produces a Task that selects all the content of the TextInput with the given Id.
                    text_input::select_all(text_input::Id::new(MY_TEXT_ID)),
                ]);
            }
        }
        Task::none()
    }
  
    fn view(&self) -> Element<Message> {
        column![
            text_input("", &self.some_text)
                // Sets the Id of the TextInput.
                .id(text_input::Id::new(MY_TEXT_ID))
                .on_input(Message::UpdateText),
            button("Select all").on_press(Message::SelectAll),
        ]
        .into()
    }
}
```

![Batch commands](./pic/batch_commands.png)

:arrow_right:  Next: [Executing Custom Commands](./executing_custom_commands.md)

:blue_book: Back: [Table of contents](./../README.md)
