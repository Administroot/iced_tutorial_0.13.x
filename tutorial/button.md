# Button

The [Button](https://docs.rs/iced/0.12.1/iced/widget/button/struct.Button.html) widget supports reactions to pressing/touching events.
It has two methods of constructions.
If the method [on_press](https://docs.rs/iced/0.12.1/iced/widget/button/struct.Button.html#method.on_press) is set, the button is enabled, and is disabled otherwise.
We can also set padding around the text of the button.

```rust
use iced::widget::{button, column, Button, Column};

pub fn main() -> iced::Result {
    iced::application("My app", update, view).run()
}

#[derive(Debug, Clone)]

enum Message {
    DoSomething,
}

fn update(_value: &mut u64, _message: Message) {}

fn view(_value: &u64) -> Column<Message> {
    column![
        Button::new("Disabled button"),
        button("Construct from function"),
        button("Enabled button").on_press(Message::DoSomething),
        button("With padding").padding(20),
    ]
    .into()
}
```

![Button](./pic/button.png)

:arrow_right:  Next: [TextInput](./text_input.md)

:blue_book: Back: [Table of contents](./../README.md)
