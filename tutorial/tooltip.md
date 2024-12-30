# Tooltip

The [Tooltip](https://docs.rs/iced/0.12.1/iced/widget/tooltip/struct.Tooltip.html) widget displays a text when the mouse is over a specified widget.
It has two methods of constructions.
It is able to change styles of the text.
We can add padding around the text inside.
We can also change the space between the tooltip and the target widget.
If the tooltip is allowed to be out of the window, the parts outside are clipped.

```rust
use iced::{
    widget::{button, column, tooltip, tooltip::Position, Tooltip},
    Element,
};

fn main() -> iced::Result {
    iced::run("My First App", MyApp::update, MyApp::view)
}

struct MyApp {
    _state: String,
}

impl Default for MyApp {
    fn default() -> Self {
        MyApp::new()
    }
}

#[derive(Debug, Clone)]
enum Message {
    _Message1,
}

impl MyApp {
    fn new() -> Self {
        Self {
            _state: String::new(),
        }
    }

    fn update(&mut self, _message: Message) {
        todo!()
    }
  
    fn view(&self) -> Element<Message> {
        column!(
            Tooltip::new(
                button("Mouse over to see the tool"),
                "Constuct from struct",
                Position::Right,
            ),
            tooltip(
                button("Mouse over to see the tool"),
                "Construct from function",
                Position::Right,
            ),
            tooltip(
                button("Mouseover to see the tooltip"),
                "With padding",
                Position::Right
            )
            .padding(20),
            tooltip(
                button("Mouseover to see the tooltip"),
                "Far away from the widget",
                Position::Right
            )
            .gap(50),
            tooltip(
                button("Mouseover to see the tooltip"),
                "Parts out of the window are clipped",
                Position::Right
            )
            .snap_within_viewport(false),
            tooltip(
                button("Mouseover to see the tooltip"),
                "Follow the cursor",
                Position::FollowCursor
            )
        )
        .into()
    }
}
```

![Tooltip](./pic/tooltip.png)

:arrow_right:  Next: [Rule](./rule.md)

:blue_book: Back: [Table of contents](./../README.md)
