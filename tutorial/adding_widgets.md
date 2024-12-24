# Adding Widgets

Use [column!](https://docs.rs/iced/0.12.1/iced/widget/macro.column.html) and [row!](https://docs.rs/iced/0.12.1/iced/widget/macro.row.html) to group multiple widgets such as [text](https://docs.rs/iced/0.12.1/iced/widget/fn.text.html) and [button](https://docs.rs/iced/0.12.1/iced/widget/fn.button.html).

```rust
use iced::widget::{button, column, row, Column, text};

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
        text("Yes or No?"),
        row!(
            button("Yes"),
            button("No")
        )
    ].into()
}
```

![Adding widgets](./pic/adding_widgets.png)

:arrow_right:  Next: [Changing Displaying Content](./changing_displaying_content.md)

:blue_book: Back: [Table of contents](./../README.md)
