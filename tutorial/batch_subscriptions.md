# Batch Subscriptions

This tutorial follows from the previous two tutorials ([keyboard events](./producing_messages_by_keyboard_events.md) and [timers](./producing_messages_by_timers.md)).
We combine the two [Subscriptions](https://docs.rs/iced/0.12.1/iced/subscription/struct.Subscription.html) of keyboard events and timers.
This is done by [Subscription::batch](https://docs.rs/iced/0.12.1/iced/subscription/struct.Subscription.html#method.batch) function.

In the following app, press the space key to start or stop the timer.

```rust
use iced::{
    event::{self, Status},
    keyboard::{key::Named, Event::KeyPressed, Key},
    time::{self, Duration},
    widget::text,
    Element, Event, Subscription, Task,
};

fn main() -> iced::Result {
    iced::application("batch subscriptions", MyApp::update, MyApp::view)
        .subscription(MyApp::subscription)
        .run()
}

struct MyApp {
    running: bool,
    seconds: u32,
}

impl Default for MyApp {
    fn default() -> Self {
        MyApp::new()
    }
}

#[derive(Debug, Clone)]
enum Message {
    StartOrStop,
    Update,
}

impl MyApp {
    fn new() -> Self {
        Self {
            running: false,
            seconds: 0u32,
        }
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::StartOrStop => {
                self.running = !self.running;
            }
            Message::Update => {
                self.seconds += 1;
            }
        }
        Task::none()
    }



    fn view(&self) -> Element<Message> {

        text(self.seconds).into()

    }

  

    fn subscription(&self) -> Subscription<Message> {

        let subscr_key = event::listen_with(|event, status, _window| match (event, status) {

            (

                Event::Keyboard(KeyPressed {

                    key: Key::Named(Named::Space),

                    ..

                }),

                Status::Ignored,

            ) => Some(Message::StartOrStop),

            _ => None,

        });

  

        if self.running {

            Subscription::batch(vec![

                subscr_key,

                time::every(Duration::from_secs(1)).map(|_| Message::Update),

            ])

        } else {

            subscr_key

        }

    }

}
```

![Batch subscriptions](./pic/batch_subscriptions.png)

:arrow_right:  Next: [Drawing Shapes](./drawing_shapes.md)

:blue_book: Back: [Table of contents](./../README.md)
