# TextInput

The [TextInput](https://docs.rs/iced/0.12.1/iced/widget/struct.TextInput.html) widget let users to input texts.
It has two methods of constructions.
If the [on_input](https://docs.rs/iced/0.12.1/iced/widget/struct.TextInput.html#method.on_input) method is set, it is enabled, and is disabled otherwise.
It supports reactions to pasting texts or keyboard submissions.
It is able to change fonts and text sizes.
We can add padding around the text inside.
We can also add an optional icon.

```rust
use iced::{
    font::Family,
    widget::{column, text, text_input, text_input::{Icon, Side}, Column, TextInput},
    Font,
};

pub fn main() -> iced::Result {
    iced::application("My app", update, view).run()
}

#[derive(Debug, Clone)]

enum MyAppMessage {
    Update3(String),
    Update4(String),
    Update5(String),
    Update6(String),
    Update7(String),
    Update11(String),
    Paste5(String),
    Submit6,
}

#[derive(Default)]

struct State {
    text3: String,
    text4: String,
    text5: String,
    text6: String,
    text7: String,
    text11: String,
    info5: String,
    info6: String,
}

fn update(state: &mut State, message: MyAppMessage) {
    match message {
        MyAppMessage::Update3(s) => {
            state.text3 = s;
        }
        MyAppMessage::Update4(s) => {
            state.text4 = s;
        }
        MyAppMessage::Update5(s) => {
            state.text5 = s;
            state.info5 = "".into();
        }
        MyAppMessage::Paste5(s) => {
            state.text5 = s;
            state.info5 = "Pasted".into();
        }
        MyAppMessage::Update6(s) => {
            state.text6 = s;
            state.info6 = "".into();
        }
        MyAppMessage::Submit6 => {
            state.info6 = "Submitted".into();
        }
        MyAppMessage::Update7(s) => {
            state.text7 = s;
        }
        MyAppMessage::Update11(s) => {
            state.text11 = s;
        }
    }
}
  

fn view(state: &State) -> Column<MyAppMessage> {
    column![
        text_input("Construct from function", ""),
        TextInput::new("Construct from struct", ""),
        text_input("Enabled text input", state.text3.as_str())
            .on_input(|s| MyAppMessage::Update3(s)),
        text_input("Shorter on_input", state.text4.as_str()).on_input(MyAppMessage::Update4),
        // A line of pasted text will be displayed below the text input
        text_input("Press Ctrl/Cmd + V", state.text5.as_str())
            .on_input(MyAppMessage::Update5)
            .on_paste(MyAppMessage::Paste5),
        text(state.info5.as_str()),

        // "Submitted" will be displayed below the text input after pressing ENTER
        text_input("Press enter", state.text6.as_str())
            .on_input(MyAppMessage::Update6)
            .on_submit(MyAppMessage::Submit6),
        text(state.info6.as_str()),
        // Display input characters in ciphertext
        text_input("Password", state.text7.as_str())
            .secure(true)
            .on_input(MyAppMessage::Update7),
        // Settings
        text_input("Different font", "").font(Font {
            family: Family::Fantasy,
            ..Font::DEFAULT
        }),

        text_input("Larger text", "").size(24),
        text_input("With padding", "").padding(20),
        // ✅
        text_input("Icon", state.text11.as_str())
            .icon(Icon {
                font: Font::DEFAULT,
                code_point: '\u{2705}',
                size: None,
                spacing: 10.,
                side: Side::Left,
            })
            .on_input(MyAppMessage::Update11),
    ]
    .into()
}
```

![TextInput](./pic/text_input.png)

:arrow_right:  Next: [Checkbox](./checkbox.md)

:blue_book: Back: [Table of contents](./../README.md)
