# Changing Displaying Content

To change the content in [view](https://docs.rs/iced/0.12.1/iced/trait.Sandbox.html#tymethod.view) dynamically, we can do the following:

* Add some fields (e.g., `counter`) to the main struct `MyApp`.
* Display the fields in [view](https://docs.rs/iced/0.12.1/iced/trait.Sandbox.html#tymethod.view) (e.g., `text(self.counter)`).
* Produce some messages in [view](https://docs.rs/iced/0.12.1/iced/trait.Sandbox.html#tymethod.view) (e.g., `button(...).on_press(MyAppMessage::ButtonPressed)`).
* Update the fields when messages are received in [update](https://docs.rs/iced/0.12.1/iced/trait.Sandbox.html#tymethod.update) (e.g., `self.counter += 1`).

```rust
use iced::widget::{button, column, text, Column};

pub fn main() -> iced::Result {
    iced::application("A counter", update, view).run()
}

#[derive(Debug, Clone)]

enum Message {
    Increment,
}

fn update(value: &mut u64, message: Message) {
    match message {
        Message::Increment => *value += 1,
    }
}

fn view(value: &u64) -> Column<Message> {
    column![
        text(value),
        button("Increase").on_press(Message::Increment),
    ]
}
```

![Producing and receiving messages](./pic/changing_displaying_content.png)

:arrow_right:  Next: [Text](./text.md)

:blue_book: Back: [Table of contents](./../README.md)
