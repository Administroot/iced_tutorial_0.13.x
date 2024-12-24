# Text

The [Text](https://docs.rs/iced/0.12.1/iced/widget/type.Text.html) widget is able to display texts.
It has three methods of constructions.
It is able to change the font, the size of the font, and display special characters.
The text inside the widget can be horizontally or vertically centered.

```rust
use iced::{
    alignment::{Horizontal, Vertical},
    font::Family,
    widget::{column, Column, text, text::Shaping, Text},
    Font, Length
};


pub fn main() -> iced::Result {
    iced::application("My app", update, view).run()
}


#[derive(Debug, Clone)]

enum Message {
    _Increment,
}

fn update(_value: &mut u64, _message: Message) {}

fn view(_value: &u64) -> Column<Message> {

column![

        "Construct from &str",

        text("Construct from function"),

        Text::new("Construct from struct"),

        text("Different font").font(Font {

            family: Family::Fantasy,

            ..Font::DEFAULT

        }),

        text("Larger text").size(24),

        text("Special character 😊").shaping(Shaping::Advanced),

        text("Center")

            .width(Length::Fill)

            .align_x(Horizontal::Center)

            .align_y(Vertical::Center),

        text("Vertical center")

            .height(Length::Fill)

            .align_y(Vertical::Center),

    ]

    .into()

}
```

![Text](./pic/text.png)

:arrow_right:  Next: [Button](./button.md)

:blue_book: Back: [Table of contents](./../README.md)
