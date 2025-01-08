# Producing Messages By Mouse Events

To capture events of the window, we implement [subscription](https://docs.rs/iced/0.12.1/iced/application/trait.Application.html#method.subscription) method in [Application](https://docs.rs/iced/0.12.1/iced/application/trait.Application.html).
This method returns [Subscription](https://docs.rs/iced/0.12.1/iced/struct.Subscription.html) struct, which allows us to specify how to handle events.
We can use [listen_with](https://docs.rs/iced/0.12.1/iced/event/fn.listen_with.html) function to construct a [Subscription](https://docs.rs/iced/0.12.1/iced/struct.Subscription.html).
The [listen_with](https://docs.rs/iced/0.12.1/iced/event/fn.listen_with.html) function takes a function as its input.
The input function takes two parameters, [Event](https://docs.rs/iced/0.12.1/iced/event/enum.Event.html) and [Status](https://docs.rs/iced/0.12.1/iced/event/enum.Status.html), and returns [Option](https://doc.rust-lang.org/std/option/enum.Option.html)\<`MyAppMessage`>, which means this function is capable of transforming [Event](https://docs.rs/iced/0.12.1/iced/event/enum.Event.html) to `MyAppMessage`.
We then receive the transformed `MyAppMessage` in [update](https://docs.rs/iced/0.12.1/iced/application/trait.Application.html#tymethod.update) method.

In the input function, we only care about ignored events (i.e., events that is not handled by widgets) by checking if [Status](https://docs.rs/iced/0.12.1/iced/widget/canvas/event/enum.Status.html) is [Status::Ignored](https://docs.rs/iced/0.12.1/iced/widget/canvas/event/enum.Status.html#variant.Ignored).

In this tutorial, we capture [Event::Mouse(...)](https://docs.rs/iced/0.12.1/iced/enum.Event.html#variant.Mouse) and [Event::Touch(...)](https://docs.rs/iced/0.12.1/iced/enum.Event.html#variant.Touch) and produce messages.

```rust

```

![Producing messages by mouse events](./pic/producing_messages_by_mouse_events.png)

:arrow_right:  Next: [Producing Messages By Keyboard Events](./producing_messages_by_keyboard_events.md)

:blue_book: Back: [Table of contents](./../README.md)
