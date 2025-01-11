# Texts In Widgets

In addition to draw a [Quad](https://docs.rs/iced/latest/iced/advanced/renderer/struct.Quad.html), we can also draw texts in our widgets.

For example, suppose we would like to draw a string slice named `CONTENT`.

```rust
struct MyWidgetWithText;

impl MyWidgetWithText {
    const CONTENT: &'static str = "  My Widget  ";

    fn new() -> Self {
        Self
    }
}
```

We use the [Renderer](https://docs.rs/iced/latest/iced/advanced/text/trait.Renderer.html)'s [fill_text](https://docs.rs/iced/latest/iced/advanced/text/trait.Renderer.html#tymethod.fill_text) method to draw the text.

```rust
fn draw(
    &self,
    _state: &Tree,
    renderer: &mut Renderer,
    _theme: &Theme,
    _style: &renderer::Style,
    layout: Layout<'_>,
    _cursor: mouse::Cursor,
    viewport: &Rectangle,
) {
    // ...

    let bounds = layout.bounds();

    renderer.fill_text(
        Text {
            content: Self::CONTENT,
            bounds: bounds.size(),
            size: renderer.default_size(),
            line_height: LineHeight::default(),
            font: renderer.default_font(),
            horizontal_alignment: Horizontal::Center,
            vertical_alignment: Vertical::Center,
            shaping: Shaping::default(),
        },
        bounds.center(),
        Color::from_rgb(0.6, 0.8, 1.0),
        *viewport,
    );
}
```

The [fill_text](https://docs.rs/iced/latest/iced/advanced/text/trait.Renderer.html#tymethod.fill_text) method needs the `Renderer` type to implement [iced::advanced::text::Renderer](https://docs.rs/iced/latest/iced/advanced/text/trait.Renderer.html).
Thus we have to require this in our [Widget](https://docs.rs/iced/latest/iced/advanced/widget/trait.Widget.html) implementation.

```rust
impl<Message, Renderer> Widget<Message, Theme, Renderer> for MyWidgetWithText
where
    Renderer: iced::advanced::Renderer + iced::advanced::text::Renderer,
```

Since the requirement of the `Renderer` type is changed, we have to change the requirement in `From<MyWidgetWithText>` for [Element](https://docs.rs/iced/latest/iced/type.Element.html), too.

```rust
impl<'a, Message, Renderer> From<MyWidgetWithText> for Element<'a, Message, Theme, Renderer>
where
    Renderer: iced::advanced::Renderer + iced::advanced::text::Renderer,
```

The full code is as follows:

```rust

```

![Texts In Widgets](./pic/texts_in_widgets.png)

:arrow_right:  Next: [Custom Background](./custom_background.md)

:blue_book: Back: [Table of contents](./../README.md)
