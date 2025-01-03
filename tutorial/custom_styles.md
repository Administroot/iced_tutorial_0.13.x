# Custom Styles

We mentioned in the [previous tutorial](./changing_styles.md) about how to change styles of widgets by the [enums](https://doc.rust-lang.org/std/keyword.enum.html) in [theme](https://docs.rs/iced/0.12.1/iced/theme/index.html) module.
Most [enums](https://doc.rust-lang.org/std/keyword.enum.html) in [theme](https://docs.rs/iced/0.12.1/iced/theme/index.html) module support `Custom` variant, e.g., [theme::Radio::Custom(...)](https://docs.rs/iced/0.12.1/iced/theme/enum.Radio.html#variant.Custom).
This variant takes [radio::StyleSheet](https://docs.rs/iced/0.12.1/iced/widget/radio/trait.StyleSheet.html) trait as its parameter.
To use the variant, we need to implement the trait (such as `RadioStyle` struct in the following code).
The [associated type](https://doc.rust-lang.org/stable/book/ch19-03-advanced-traits.html#specifying-placeholder-types-in-trait-definitions-with-associated-types) of the trait should be set to [iced::Theme](https://docs.rs/iced/0.12.1/iced/enum.Theme.html).
The methods in the trait return [radio::Appearance](https://docs.rs/iced/0.12.1/iced/widget/radio/struct.Appearance.html).
We can use [theme::Radio::Default](https://docs.rs/iced/0.12.1/iced/theme/enum.Radio.html#variant.Default) to obtain the default value of [radio::Appearance](https://docs.rs/iced/0.12.1/iced/widget/radio/struct.Appearance.html), e.g., `style.active(&theme::Radio::Default, is_selected)`.
After that, we can modify the default [radio::Appearance](https://docs.rs/iced/0.12.1/iced/widget/radio/struct.Appearance.html) based on our needs.

```rust
use iced::{
    widget::{column, radio},
    Element,
};

fn main() -> iced::Result {
    iced::application("custom_styles", MyApp::update, MyApp::view).run()
}

struct MyApp {
    selection: Option<Choice>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Choice {
    A,
    B,
    C,
}


impl Default for MyApp {
    fn default() -> Self {
        MyApp::new()
    }
}

#[derive(Debug, Clone)]
enum Message {
    RadioSelected(Choice),
}

impl MyApp {
    fn new() -> Self {
        Self { selection: None }
    }
  
    fn update(&mut self, message: Message) {
        match message {
            Message::RadioSelected(c) => {
                self.selection = Some(c);
            }
        }
    }
  
    fn view(&self) -> Element<Message> {
        column![
            radio(
                "Choice A",
                Choice::A,
                self.selection,
                Message::RadioSelected
            )
            .style(if self.selection == Some(Choice::A) {
                style::radio_selected
            } else {
                style::radio_unselected
            }),
            radio(
                "Choice B",
                Choice::B,
                self.selection,
                Message::RadioSelected
            )
            .style(if self.selection == Some(Choice::B) {
                style::radio_selected
            } else {
                style::radio_unselected
            }),
            radio(
                "Choice C",
                Choice::C,
                self.selection,
                Message::RadioSelected
            ),
        ]
        .into()
    }
}

mod style {
    use iced::{widget::radio, Background, Color, Theme};
  
    pub fn radio_selected(_theme: &Theme, _status: radio::Status) -> radio::Style {
        radio::Style {
            text_color: Some(Color::from_rgb(0., 0., 1.)),
            background: Background::Color(Color::from_rgb(1., 1., 1.)),
            dot_color: Color::from_rgb(0., 0., 1.),
            border_width: 1.0,
            border_color: Color::from_rgb(0., 0., 1.),
        }
    }

    pub fn radio_unselected(_theme: &Theme, _status: radio::Status) -> radio::Style {
        radio::Style {
            text_color: Some(Color::from_rgb(0.5, 0.5, 0.5)),
            background: Background::Color(Color::from_rgb(1., 1., 1.)),
            dot_color: Color::from_rgb(0., 0., 1.),
            border_width: 1.0,
            border_color: Color::from_rgb(0., 0., 1.),
        }
    }
}
```

![Custom styles](./pic/custom_styles.png)

:arrow_right:  Next: [More Than One Page](./more_than_one_page.md)

:blue_book: Back: [Table of contents](./../README.md)
