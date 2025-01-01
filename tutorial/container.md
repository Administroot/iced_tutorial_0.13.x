# Container

[Container](https://docs.rs/iced/0.12.1/iced/widget/container/struct.Container.html) is another way to help us to lay out widgets, especially when we need to align a widget center both horizontally and vertically.
[Container](https://docs.rs/iced/0.12.1/iced/widget/container/struct.Container.html) accepts any widget including [Column](https://docs.rs/iced/0.12.1/iced/widget/struct.Column.html) and [Row](https://docs.rs/iced/0.12.1/iced/widget/struct.Row.html).

```rust
use iced::{
    widget::{column, container, Container},
    Alignment, Element, Length,
};

fn main() -> iced::Result {
    iced::run("My First App", MyApp::update, MyApp::view)
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
            Container::new("Construct from struct"),
            container("Constuct from function"),
            container("With padding").padding(20),
            container("Different alignment")
                .width(Length::Fill)
                .align_x(Alignment::Center),
            container("Different alignment for vertical axis")
                .height(Length::Fill)
                .align_y(Alignment::Center),
        )
        .into()
    }
}
```

![Container](./pic/container.png)

If we want to center a widget both horizontally and vertically, we can use the following code:

```rust
container("Center").width(Length::Fill).height(Length::Fill).center_x().center_y()
```

:arrow_right:  Next: [Scrollable](./scrollable.md)

:blue_book: Back: [Table of contents](./../README.md)
