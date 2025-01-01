# Changing Themes

We can implement [theme](https://docs.rs/iced/0.12.1/iced/trait.Sandbox.html#method.theme) method in [Sandbox](https://docs.rs/iced/0.12.1/iced/trait.Sandbox.html) to return the desired theme.

```rust
use iced::{
    widget::{column, text},
    Element,
};

fn main() -> iced::Result {
    iced::application("changing_theme", MyApp::update, MyApp::view)
        .theme(MyApp::theme)
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

    fn theme(&self) -> iced::Theme {
        iced::Theme::Dark
        // or
        // iced::Theme::Light
    }
}
```

![Changing themes](./pic/changing_themes.png)

:arrow_right:  Next: [Changing Styles](./changing_styles.md)

:blue_book: Back: [Table of contents](./../README.md)
