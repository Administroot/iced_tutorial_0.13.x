# Drawing Widgets

In addition to the build-in widgets, we can also design our own custom widgets.
To do so, we need to enable the [advanced](https://docs.rs/crate/iced/0.12.1/features#advanced) feature.
The dependencies of the `Cargo.toml` file should look like this:

```toml
[dependencies]
iced = { version = "0.13.1", features = ["advanced"] }
```

Then, we need a struct that implement [Widget](https://docs.rs/iced/0.12.1/iced/advanced/widget/trait.Widget.html) trait.

```rust
struct MyWidget;

impl<Message, Renderer> Widget<Message, Theme, Renderer> for MyWidget
where
    Renderer: iced::advanced::Renderer,
{
    fn size(&self) -> Size<Length> {
        // ...
    }

    fn layout(
        &self,
        _tree: &mut Tree,
        _renderer: &Renderer,
        _limits: &layout::Limits,
    ) -> layout::Node {
        // ...
    }

    fn draw(
        &self,
        _state: &Tree,
        _renderer: &mut Renderer,
        _theme: &Theme,
        _style: &renderer::Style,
        _layout: Layout<'_>,
        _cursor: mouse::Cursor,
        _viewport: &Rectangle,
    ) {
        // ...
    }
}
```

We define the size of `MyWidget` by the methods: [size](https://docs.rs/iced/0.12.1/iced/advanced/trait.Widget.html#tymethod.size) and [layout](https://docs.rs/iced/0.12.1/iced/advanced/widget/trait.Widget.html#tymethod.layout).
Currently, we set the width and height to [Length::Shrink](https://docs.rs/iced/0.12.1/iced/enum.Length.html#variant.Shrink), to tell the layout system that we use the least space for this widget.

```rust
fn size(&self) -> Size<Length> {
    Size {
        width: Length::Shrink,
        height: Length::Shrink,
    }
}
```

Then, we tell the layout system the precise size we are going to use for the widget.
In this example, our widget is of size `(100, 100)`.

```rust
fn layout(&self, _renderer: &Renderer, _limits: &layout::Limits) -> layout::Node {
    layout::Node::new([100, 100].into())
}
```

Usually, the [layout](https://docs.rs/iced/0.12.1/iced/advanced/widget/trait.Widget.html#tymethod.layout) method would consider the [Limits](https://docs.rs/iced/0.12.1/iced/advanced/layout/struct.Limits.html) parameter, which is the constraints from the layout system.
But now, we ignore it for simplicity.

Next, we draw our widget in the [draw](https://docs.rs/iced/0.12.1/iced/advanced/widget/trait.Widget.html#tymethod.draw) method.
We use the given [Renderer](https://docs.rs/iced/0.12.1/iced/advanced/trait.Renderer.html) to do so.
One may refer to the given [Theme](https://docs.rs/iced/0.12.1/iced/enum.Theme.html) and [Style](https://docs.rs/iced/0.12.1/iced/advanced/renderer/struct.Style.html) for the colors of the widget.

```rust
fn draw(
    &self,
    _state: &Tree,
    renderer: &mut Renderer,
    _theme: &Theme,
    _style: &renderer::Style,
    layout: Layout<'_>,
    _cursor: mouse::Cursor,
    _viewport: &Rectangle,
) {
    renderer.fill_quad(
        Quad {
            bounds: layout.bounds(),
            border: Border {
                color: Color::from_rgb(0.6, 0.8, 1.0),
                width: 1.0,
                radius: 10.0.into(),
            },
            shadow: Shadow::default(),
        },
        Color::from_rgb(0.0, 0.2, 0.4),
    );
}
```

The given [Layout](https://docs.rs/iced/0.12.1/iced/advanced/struct.Layout.html) parameter would be calculated automatically by the layout system according to the [size](https://docs.rs/iced/0.12.1/iced/advanced/trait.Widget.html#tymethod.size) and [layout](https://docs.rs/iced/0.12.1/iced/advanced/widget/trait.Widget.html#tymethod.layout) methods we defined before.

For convenience, we can implement `From<MyWidget>` for [Element](https://docs.rs/iced/0.12.1/iced/type.Element.html).

```rust
impl<'a, Message, Renderer> From<MyWidget> for Element<'a, Message, Theme, Renderer>
where
    Renderer: iced::advanced::Renderer,
{
    fn from(widget: MyWidget) -> Self {
        Self::new(widget)
    }
}
```

Finally, the widget can be added to our app by the following code.

```rust
fn view(&self) -> iced::Element<Self::Message> {
    container(MyWidget)
        .width(Length::Fill)
        .height(Length::Fill)
        .align_x(alignment::Horizontal::Center)
        .align_y(alignment::Vertical::Center)
        .into()
}
```

Note that it is not necessary to put `MyWidget` in a [Container](https://docs.rs/iced/0.12.1/iced/widget/container/struct.Container.html).
We can add the widget directly into our app.

```rust
fn view(&self) -> iced::Element<Self::Message> {
    MyWidget.into()
}
```

The full code is as follows:

```rust
use iced::{
    advanced::{
        layout,
        renderer::{self, Quad},
        widget, Layout, Widget,
    },
    alignment, mouse,
    widget::container,
    Border, Color, Element, Length, Rectangle, Shadow, Size, Theme,
};
  
fn main() -> iced::Result {
    iced::application("drawing widgets", MyApp::update, MyApp::view).run()
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
        container(MyWidget)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(alignment::Horizontal::Center)
            .align_y(alignment::Vertical::Center)
            .into()
    }
}

struct MyWidget;

impl<Message, Renderer> Widget<Message, Theme, Renderer> for MyWidget
where
    Renderer: iced::advanced::Renderer,
{
    fn size(&self) -> Size<Length> {
        Size {
            width: Length::Shrink,
            height: Length::Shrink,
        }
    }
  
    fn layout(
        &self,
        _tree: &mut widget::Tree,
        _renderer: &Renderer,
        _limits: &layout::Limits,
    ) -> layout::Node {
        layout::Node::new(Size::new(100., 100.))
    }

    fn draw(
        &self,
        _tree: &widget::Tree,
        renderer: &mut Renderer,
        _theme: &Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        _cursor: mouse::Cursor,
        _viewport: &Rectangle,
    ) {
        renderer.fill_quad(
            Quad {
                bounds: layout.bounds(),
                border: Border {
                    color: Color::from_rgb(0.6, 0.8, 1.0),
                    width: 1.0,
                    radius: 10.0.into(),
                },
                shadow: Shadow::default(),
            },
            Color::from_rgb(0.0, 0.2, 0.4),
        );
    }
}

impl<'a, Message, Renderer> From<MyWidget> for Element<'a, Message, Theme, Renderer>
where
    Renderer: iced::advanced::Renderer,
{
    fn from(widget: MyWidget) -> Self {
        Self::new(widget)
    }
}
```

![Drawing Widgets](./pic/drawing_widgets.png)

:arrow_right:  Next: [Updating Widgets From Outside](./updating_widgets_from_outside.md)

:blue_book: Back: [Table of contents](./../README.md)
