# Rule

The [Rule](https://docs.rs/iced/0.12.1/iced/widget/rule/struct.Rule.html) widget is a horizontal (or vertical) line for separating widgets clearly.
It has two methods of constructions.
We can change the space around it.
The widget can be set to be either horizontal or vertical.

```rust
use iced::{
    widget::{column, horizontal_rule, text, vertical_rule, Rule},
    Element,
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
            text("Construct from struct"),
            Rule::horizontal(0),
            text("Construct from function"),
            horizontal_rule(0),
            text("Different space"),
            horizontal_rule(50),
            text("Vertical rule"),
            vertical_rule(100),
        )
        .into()
    }
}
```

![Rule](./pic/rule.png)

:arrow_right:  Next: [Image](./image.md)

:blue_book: Back: [Table of contents](./../README.md)
