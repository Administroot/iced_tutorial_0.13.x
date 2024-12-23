# First App - Hello World!

We need a struct to implement [Sandbox](https://docs.rs/iced/0.12.1/iced/trait.Sandbox.html), and call its [run](https://docs.rs/iced/0.12.1/iced/trait.Sandbox.html#method.run) method from `main`.
All widgets should be placed inside the [view](https://docs.rs/iced/0.12.1/iced/trait.Sandbox.html#tymethod.view) method.

```rust
use iced::widget::{Column, column};

pub fn main() -> iced::Result {
    iced::application("My app", update, view).run()
}

#[derive(Debug, Clone)]

enum Message {
    _Increment,
}

fn update(_value: &mut u64, _message: Message) {

}

fn view(_value: &u64) -> Column<Message> {
    column![
        "hello, world"
    ]
}
```

![First app](./pic/first_app.png)

:arrow_right:  Next: [Explanation of Sandbox Trait](./explanation_of_sandbox_trait.md)

:blue_book: Back: [Table of contents](./../README.md)
