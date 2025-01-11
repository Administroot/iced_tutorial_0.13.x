# Custom Background

We can also draw an image on a [Widget](https://docs.rs/iced/0.12.1/iced/advanced/widget/trait.Widget.html).

Similar to the [Image](https://docs.rs/iced/0.12.1/iced/widget/image/struct.Image.html) widget, we have to enable the [image](https://docs.rs/crate/iced/latest/features#image) feature.

```toml
[dependencies]
iced = { version = "0.13.1", features = ["image", "advanced"] }
```

Assume we have an image, named `ferris.png`,  in the Cargo root directory.
We load this image when we initialize our widget.

```rust
struct MyWidgetWithImage {
    handle: Handle,
}

impl MyWidgetWithImage {
    fn new() -> Self {
        Self {
            handle: Handle::from_path("ferris.png"),
        }
    }
}
```

Then, we use the [iced::widget::image::layout](https://docs.rs/iced/0.12.1/iced/widget/image/fn.layout.html) function to determine the [layout](https://docs.rs/iced/0.12.1/iced/advanced/widget/trait.Widget.html#tymethod.layout) of our widget.

```rust
fn layout(&self, _tree: &mut Tree, renderer: &Renderer, limits: &layout::Limits) -> layout::Node {
    iced::widget::image::layout(
        renderer,
        limits,
        &self.handle,
        Length::Fixed(200.),
        Length::Fixed(200.),
        iced::ContentFit::Contain,
    )
}
```

And we draw the image by the [iced::widget::image::draw](https://docs.rs/iced/0.12.1/iced/widget/image/fn.draw.html) function.

```rust
fn draw(
    &self,
    _state: &Tree,
    renderer: &mut Renderer,
    _theme: &Renderer::Theme,
    _style: &renderer::Style,
    layout: Layout<'_>,
    _cursor: mouse::Cursor,
    _viewport: &Rectangle,
) {
    // ...

    iced::widget::image::draw(
        renderer,
        layout,
        &self.handle,
        iced::ContentFit::Contain,
        iced::widget::image::FilterMethod::Linear,
    );
}
```

Both functions require the `Renderer` to implement [iced::advanced::image::Renderer](https://docs.rs/iced/0.12.1/iced/advanced/image/trait.Renderer.html).

```rust
impl<Message, Renderer> Widget<Message, Theme, Renderer> for MyWidgetWithImage
where
    Renderer: iced::advanced::Renderer + iced::advanced::image::Renderer<Handle = Handle>,
```

The full code is as follows:

```rust

```

![Custom Background](./pic/custom_background.png)

:arrow_right:  Next: [Widgets With Children](./widgets_with_children.md)

:blue_book: Back: [Table of contents](./../README.md)
