# Toggler

The [Toggler](https://docs.rs/iced/0.12.1/iced/widget/toggler/struct.Toggler.html) widget represents a boolean value.
It has two methods of constructions.
It supports reactions to clicking and touching.
It is able to change styles of the button and the text and the space between them.
It can also align its text.

```rust
use iced::{
    alignment::Horizontal,
    font::Family,
    widget::{column, text::Shaping, toggler, Column, Toggler},
    Font, Length,
};

pub fn main() -> iced::Result {
    iced::application("My app", update, view).run()
}

#[derive(Debug, Clone)] 

enum MyAppMessage {
    DoNothing,
    Update3(bool),
    Update4(bool),
}

#[derive(Default)]
struct State {
    toggler3: bool,
    toggler4: bool,
}

fn update(state: &mut State, message: MyAppMessage) {
    match message {
        MyAppMessage::DoNothing => {}
        MyAppMessage::Update3(b) => {
            state.toggler3 = b;
        }
        MyAppMessage::Update4(b) => {
            state.toggler4 = b;
        }
    }
}
  
fn view(state: &State) -> Column<MyAppMessage> {
    column![
        Toggler::new(false)
            .label("Construct from struct")
            .on_toggle(|_| MyAppMessage::DoNothing),
        toggler(false)
            .label("Construct from function")
            .on_toggle(|_| MyAppMessage::DoNothing),
        toggler(state.toggler3)
            .label("Functional toggler")
            .on_toggle(|b| MyAppMessage::Update3(b)),
        toggler(state.toggler4)
            .label("Shorter parameter")
            .on_toggle(MyAppMessage::Update4),
        toggler(false)
            .label("Larger button")
            .on_toggle(|_| MyAppMessage::DoNothing)
            .size(30),
        toggler(false)
            .label("Different font")
            .on_toggle(|_| MyAppMessage::DoNothing)
            .font(Font {
                family: Family::Fantasy,
                ..Font::DEFAULT
            }),
        toggler(false)
            .label("Larger text")
            .on_toggle(|_| MyAppMessage::DoNothing)
            .text_size(24),
        toggler(false)
            .label("Special character 😊")
            .on_toggle(|_| MyAppMessage::DoNothing)
            .text_shaping(Shaping::Advanced),
        toggler(false)
            .label("Space between button and text")
            .on_toggle(|_| MyAppMessage::DoNothing)
            .spacing(30),
        toggler(false)
            .label("Centered text")
            .on_toggle(|_| MyAppMessage::DoNothing)
            .width(Length::Fill)
            .text_alignment(Horizontal::Center),
    ]
}
```

![Toggler](./pic/toggler.png)

:arrow_right:  Next: [Radio](./radio.md)

:blue_book: Back: [Table of contents](./../README.md)
