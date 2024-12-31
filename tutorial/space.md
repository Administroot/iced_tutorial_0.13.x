# Space

[Space](https://docs.rs/iced/0.12.1/iced/widget/space/struct.Space.html) is a convenient widget that helps us laying out our widgets.
It is an empty widget that occupies a space.
It has several constructions to help us allocating spaces horizontally, vertically or both.

```rust
use iced::{
    widget::{button, column, horizontal_space, row, vertical_space, Space},
    Alignment, Element, Length,
};

fn main() -> iced::Result {
    iced::run("Space", MyApp::update, MyApp::view)
}

struct MyApp {
    _state: String,
}

impl Default for MyApp {
    fn default() -> Self {
        MyApp::new()
    }
}

#[derive(Debug, Clone)]
enum Message {
    _Message1,
}

impl MyApp {
    fn new() -> Self {
        Self {
            _state: String::new(),
        }
    }

    fn update(&mut self, _message: Message) {
        todo!()
    }


    fn view(&self) -> Element<Message> {
        column!(
            row![
                button("Horizontal space 1A"),
                Space::with_width(50),
                button("Horizontal space 1B"),
            ],
            row![
                // Button 2A on the far left
                button("Horizontal space 2A"),
                Space::with_width(Length::Fill),
                // Button 2B on the far right
                button("Horizontal space 2B"),
            ],
            // The same effect with Space 2*
            row![
                button("Horizontal space 3A"),
                horizontal_space(),
                button("Horizontal space 3B"),
            ],
            button("Vertical space 1A"),
            Space::with_height(50),
            button("Vertical space 1B"),
            Space::with_height(Length::Fill),
            button("Vertical space 2A"),
            vertical_space(),
            button("Vertical space 2B"),
            button("Diagonal space A"),
            row![Space::new(50, 50), button("Diagonal space B"),].align_y(Alignment::End),
        )
        .into()
    }
}
```

![Space](./pic/space.png)

:arrow_right:  Next: [Container](./container.md)

:blue_book: Back: [Table of contents](./../README.md)
