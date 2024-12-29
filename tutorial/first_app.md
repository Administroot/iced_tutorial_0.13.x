# First App - Hello World!

We need a struct to implement [Sandbox](https://docs.rs/iced/0.12.1/iced/trait.Sandbox.html), and call its [run](https://docs.rs/iced/0.12.1/iced/trait.Sandbox.html#method.run) method from `main`.
All widgets should be placed inside the [view](https://docs.rs/iced/0.12.1/iced/trait.Sandbox.html#tymethod.view) method.

```rust
use iced::{
    widget::{
        column, text
    },
    Element
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
            text("Hello World!".to_string()),
        ).into()
    }
}
```

![First app](./pic/first_app.png)

:arrow_right:  Next: [Explanation of Sandbox Trait](./explanation_of_sandbox_trait.md)

:blue_book: Back: [Table of contents](./../README.md)
