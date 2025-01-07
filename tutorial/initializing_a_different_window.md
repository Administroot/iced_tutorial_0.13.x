# Initializing A Different Window

We can use [window::Settings](https://docs.rs/iced/0.12.1/iced/window/settings/struct.Settings.html) to change the properties of the window (such as [position](https://docs.rs/iced/0.12.1/iced/window/settings/struct.Settings.html#structfield.position) and [size](https://docs.rs/iced/0.12.1/iced/window/settings/struct.Settings.html#structfield.size)) when we call [run](https://docs.rs/iced/0.12.1/iced/trait.Sandbox.html#method.run) of a [Sandbox](https://docs.rs/iced/0.12.1/iced/trait.Sandbox.html) or [Application](https://docs.rs/iced/0.12.1/iced/application/trait.Application.html).
Developers might be interested in reading the document of [window::Settings](https://docs.rs/iced/0.12.1/iced/window/settings/struct.Settings.html) for other properties.

```rust
use iced::{
    widget::{column, text},
    window, Element, Point, Size,
};

fn main() -> iced::Result {
    let window_setting = window::settings::Settings {
        size: Size {
            width: 70.,
            height: 30.,
        },
        position: window::Position::Specific(Point { x: 50., y: 60. }),
        ..Default::default()
    };
    iced::application("initializing_a_different_window", MyApp::update, MyApp::view)

        .window(window_setting)

        .run()

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

        column!(text("Hello World!".to_string()),).into()

    }

}
```

![Initializing a different window](./pic/initializing_a_different_window.png)

:arrow_right:  Next: [Changing The Window Dynamically](./changing_the_window_dynamically.md)

:blue_book: Back: [Table of contents](./../README.md)
