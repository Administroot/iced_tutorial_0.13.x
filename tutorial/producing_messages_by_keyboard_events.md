# Producing Messages By Keyboard Events

This tutorial follows from the [previous tutorial](./producing_messages_by_mouse_events.md).
Instead of capturing [Event::Mouse](https://docs.rs/iced/0.12.1/iced/event/enum.Event.html#variant.Mouse) and [Event::Touch](https://docs.rs/iced/0.12.1/iced/event/enum.Event.html#variant.Touch), we capture [Event::Keyboard](https://docs.rs/iced/0.12.1/iced/event/enum.Event.html#variant.Keyboard) in the [listen_with](https://docs.rs/iced/0.12.1/iced/event/fn.listen_with.html) function.

```rust
use iced::{
    event::{self, Status},
    keyboard::{key::Named, Event::KeyPressed, Key},
    widget::text,
    Element, Event, Subscription, Task,
};

fn main() -> iced::Result {
    iced::application(
        "producing_messages_by_keyboard_events",
        MyApp::update,
        MyApp::view,
    )
    .subscription(MyApp::subscription)
    .run()
}

struct MyApp {
    pressed_key: String,
}

impl Default for MyApp {
    fn default() -> Self {
        MyApp::new()
    }
}

#[derive(Debug, Clone)]
enum Message {
    KeyPressed(String),
}
  
impl MyApp {
    fn new() -> Self {
        Self {
            pressed_key: String::new(),
        }
    }
  
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::KeyPressed(s) => {
                self.pressed_key = s;
                return Task::none();
            }
        }
    }

    fn view(&self) -> Element<Message> {
        text(self.pressed_key.as_str()).into()
    }
  
    fn subscription(&self) -> Subscription<Message> {
        event::listen_with(|event, status, _window| match (event, status) {
            (
                Event::Keyboard(KeyPressed {
                    key: Key::Named(Named::Space),
                    ..
                }),
                Status::Ignored,
            ) => Some(Message::KeyPressed("Space".into())),
            _ => None,
        })
    }
}
```

![Producing messages by keyboard events](./pic/producing_messages_by_keyboard_events.png)

:arrow_right:  Next: [Producing Messages By Timers](./producing_messages_by_timers.md)

:blue_book: Back: [Table of contents](./../README.md)
