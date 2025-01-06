# Controlling Widgets By Commands

We can use [Command](https://docs.rs/iced/0.12.1/iced/struct.Command.html) to control widgets.
Some widgets have [functions](https://doc.rust-lang.org/stable/book/ch03-03-how-functions-work.html) to change their behavior.
These [functions](https://doc.rust-lang.org/stable/book/ch03-03-how-functions-work.html) are located at their respective modules.
For example, [focus](https://docs.rs/iced/0.12.1/iced/widget/text_input/fn.focus.html) is a function in [text_input](https://docs.rs/iced/0.12.1/iced/widget/text_input/index.html) module that makes a [TextInput](https://docs.rs/iced/0.12.1/iced/widget/text_input/struct.TextInput.html) gaining the focus.
This function takes a parameter [text_input::Id](https://docs.rs/iced/0.12.1/iced/widget/text_input/struct.Id.html), which can be specified by [id](https://docs.rs/iced/0.12.1/iced/widget/text_input/struct.TextInput.html#method.id) method of [TextInput](https://docs.rs/iced/0.12.1/iced/widget/text_input/struct.TextInput.html).
The function returns a [Command](https://docs.rs/iced/0.12.1/iced/struct.Command.html) and we can return the [Command](https://docs.rs/iced/0.12.1/iced/struct.Command.html) in [update](https://docs.rs/iced/0.12.1/iced.rs/iced/application/trait.Application.html#tymethod.update) or [new](https://docs.rs/iced/0.12.1/iced/application/trait.Application.html#tymethod.new) methods of [Application](https://docs.rs/iced/0.12.1/iced/application/trait.Application.html).

```rust
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
    EditText,
    UpdateText(String),
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
            Message::EditText => {
                return text_input::focus(text_input::Id::new(MY_TEXT_ID));
            },
            Message::UpdateText(s) => self.some_text = s,
        }
        Task::none()
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
```

![Controlling widgets by commands](./pic/controlling_widgets_by_commands.png)

:arrow_right:  Next: [Batch Commands](./batch_commands.md)

:blue_book: Back: [Table of contents](./../README.md)
