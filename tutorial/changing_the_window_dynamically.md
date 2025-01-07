# Changing The Window Dynamically

We can use functions provided in [window](https://docs.rs/iced/0.12.1/iced/window/index.html) module to change the window after it is initialized.
For example, to [resize](https://docs.rs/iced/0.12.1/iced/window/fn.resize.html) the window.
These functions return [Command](https://docs.rs/iced/0.12.1/iced/struct.Command.html), which can be used as the return value in [update](https://docs.rs/iced/0.12.1/iced/application/trait.Application.html#tymethod.update) method.
Developers might be interested in other [Commands](https://docs.rs/iced/0.12.1/iced/struct.Command.html) in [window](https://docs.rs/iced/0.12.1/iced/window/index.html) module.

The [resize](https://docs.rs/iced/0.12.1/iced/window/fn.resize.html) function needs an ID of the window we are going to resize.
Internally, Iced reserves [window::Id::MAIN](https://docs.rs/iced/0.12.1/iced/window/struct.Id.html#associatedconstant.MAIN) for the first window spawned.

```rust
use iced::{
    widget::{button, row, text_input},
    window, Element, Size, Task,
};

fn main() -> iced::Result {
    iced::application(
        "changing the window dynamically",
        MyApp::update,
        MyApp::view,
    )
    .run()
}

struct MyApp {
    width: String,
    height: String,
}

impl Default for MyApp {
    fn default() -> Self {
        MyApp::new().0
    }
}

#[derive(Debug, Clone)]
enum Message {
    UpdateWidth(String),
    UpdateHeight(String),
    ResizeWindow,
}
  
impl MyApp {
    fn new() -> (Self, Task<Message>) {
        (
            Self {
                width: String::new(),
                height: String::new(),
            },
            Task::none(),
        )
    }
  
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::UpdateWidth(w) => {
                self.width = w;
            }
            Message::UpdateHeight(h) => {
                self.height = h;
            }

            Message::ResizeWindow => {
                // If don't define var "width" and "height", error 'E0521' will occur.
                let width = self.width.parse().unwrap();
                let height = self.height.parse().unwrap();
  
                // Id in Task<Option<Id>> will be delivered to Task::and_then() method, then to the closure.
                return window::get_oldest()
                    .and_then(move |window| window::resize(window, Size::new(width, height)));
            }
        }
        Task::none()
    }
  
    fn view(&self) -> Element<Message> {
        row!(
            text_input("Width", &self.width).on_input(Message::UpdateWidth),
            text_input("Height", &self.height).on_input(Message::UpdateHeight),
            button("Resize window").on_press(Message::ResizeWindow),
        )
        .into()
    }
}
```

![Changing the window dynamically](./pic/changing_the_window_dynamically.png)

:arrow_right:  Next: [Closing The Window On Demand](./closing_the_window_on_demand.md)

:blue_book: Back: [Table of contents](./../README.md)
