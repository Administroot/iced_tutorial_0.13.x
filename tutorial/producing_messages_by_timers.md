# Producing Messages By Timers

To use build-in timers, we need to enable one of the following features: [tokio](https://docs.rs/crate/iced/0.12.1/features#tokio), [async-std](https://docs.rs/crate/iced/0.12.1/features#async-std), or [smol](https://docs.rs/crate/iced/0.12.1/features#smol).
In this tutorial, we use [tokio](https://docs.rs/crate/iced/0.12.1/features#tokio) feature.
The dependencies of `Cargo.toml` should look like this:

```toml
[dependencies]
iced = { version = "0.13.1", features = ["tokio"] }
```

We use [time::every](https://docs.rs/iced/0.12.1/iced/time/fn.every.html) function to obtain [Subscription](https://docs.rs/iced/0.12.1/iced/struct.Subscription.html)\<[Instant](https://docs.rs/iced/0.12.1/iced/time/struct.Instant.html)> struct.
Then we map the struct to [Subscription](https://docs.rs/iced/0.12.1/iced/struct.Subscription.html)\<`MyAppMessage`> by [Subscription::map](https://docs.rs/iced/0.12.1/iced/struct.Subscription.html#method.map) method.
The result will be returned in the [subscription](https://docs.rs/iced/0.12.1/iced/application/trait.Application.html#method.subscription) method of [Application](https://docs.rs/iced/0.12.1/iced/application/trait.Application.html).
The corresponding `MyAppMessage` will be received in the [update](https://docs.rs/iced/0.12.1/iced/application/trait.Application.html#tymethod.update) method.

```rust
use iced::{
    time::{self, Duration},

    widget::text,

    Element, Subscription, Task,

};

  

fn main() -> iced::Result {

    iced::application(

        "producing messages by timers",

        MyApp::update,

        MyApp::view,

    )

    .subscription(MyApp::subscription)

    .run()

}

  

struct MyApp {

    seconds: u32,

}

  

impl Default for MyApp {

    fn default() -> Self {

        MyApp::new()

    }

}

  

#[derive(Debug, Clone)]

enum Message {

    Update,

}

  

impl MyApp {

    fn new() -> Self {

        Self { seconds: 0u32 }

    }

  

    fn update(&mut self, message: Message) -> Task<Message> {

        match message {

            Message::Update => {

                self.seconds += 1;

                return Task::none();

            }

        }

    }

  

    fn view(&self) -> Element<Message> {

        text(self.seconds).into()

    }

  

    fn subscription(&self) -> Subscription<Message> {

        time::every(Duration::from_secs(1)).map(|_| Message::Update)

    }

}
```

![Producing messages by timers](./pic/producing_messages_by_timers.png)

:arrow_right:  Next: [Batch Subscriptions](./batch_subscriptions.md)

:blue_book: Back: [Table of contents](./../README.md)
