# Drawing Shapes

[Canvas](https://docs.rs/iced/0.12.1/iced/widget/canvas/struct.Canvas.html) is a widget that helps us drawing free-style shapes.
To use the widget, we need to enable the [canvas](https://docs.rs/crate/iced/0.12.1/features#canvas) feature.

```toml
[dependencies]
iced = { version = "0.13.1", features = ["canvas"] }
```

We use [Canvas::new](https://docs.rs/iced/0.12.1/iced/widget/canvas/struct.Canvas.html#method.new) to obtain the canvas widget.
This function accepts a [Program](https://docs.rs/iced/0.12.1/iced/widget/canvas/trait.Program.html) trait.
We can create a struct (say, `MyProgram`) to implement this trait.

```rust
impl<Message> Program<Message> for MyProgram {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        // ...
    }
}
```

There is a generic data type `Message` when we implement the [Program](https://docs.rs/iced/0.12.1/iced/widget/canvas/trait.Program.html) trait.
This helps us adapting to our [Message](https://docs.rs/iced/0.12.1/iced/trait.Sandbox.html#associatedtype.Message) in [Sandbox](https://docs.rs/iced/0.12.1/iced/trait.Sandbox.html).

The associated type [State](https://docs.rs/iced/0.12.1/iced/widget/canvas/trait.Program.html#associatedtype.State) is not used in our example, so we set it to `()`.

The key of [Program](https://docs.rs/iced/0.12.1/iced/widget/canvas/trait.Program.html) is the [draw](https://docs.rs/iced/0.12.1/iced/widget/canvas/trait.Program.html#tymethod.draw) method.
In the method, we define what shapes we are going to draw.
We use [Frame](https://docs.rs/iced/0.12.1/iced/widget/canvas/enum.Frame.html) as a *pen* to draw shapes.
For example, we use the [fill_rectangle](https://docs.rs/iced/0.12.1/iced/widget/canvas/enum.Frame.html#method.fill_rectangle) method of [Frame](https://docs.rs/iced/0.12.1/iced/widget/canvas/enum.Frame.html) to draw a filled rectangle.
Or we can stroke and fill any [Path](https://docs.rs/iced/0.12.1/iced/widget/canvas/struct.Path.html).
Finally, we use the [into_geometry](https://docs.rs/iced/0.12.1/iced/widget/canvas/enum.Frame.html#method.into_geometry) method of [Frame](https://docs.rs/iced/0.12.1/iced/widget/canvas/enum.Frame.html) to return the [Geometry](https://docs.rs/iced/0.12.1/iced/widget/canvas/enum.Geometry.html) as required by the [draw](https://docs.rs/iced/0.12.1/iced/widget/canvas/trait.Program.html#tymethod.draw) method.

```rust
use iced::{
    mouse::Cursor,
    widget::{
        canvas::{Frame, Geometry, Path, Program, Stroke},
        column, Canvas,
    },
    Color, Element, Length, Point, Rectangle, Renderer, Theme, Vector,
};

fn main() -> iced::Result {
    iced::application("drawing shapes", MyApp::update, MyApp::view).run()
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
            "A Canvas",
            Canvas::new(MyProgram)
                .width(Length::Fill)
                .height(Length::Fill)
        )
        .into()
    }
}
  
// Struct for canvas
struct MyProgram;
  
impl<Message> Program<Message> for MyProgram {
    type State = ();
  
    // Required method
    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: Cursor,
    ) -> Vec<Geometry> {
        let mut frame = Frame::new(renderer, bounds.size());
        frame.fill_rectangle(Point::ORIGIN, bounds.size(), Color::from_rgb(0.0, 0.2, 0.4));
        frame.fill(
            &Path::circle(frame.center(), frame.width().min(frame.height()) / 4.0),
            Color::from_rgb(0.6, 0.8, 1.0),
        );
  
        // Draws the stroke of the given Path on the Frame with the provided style.
        frame.stroke(
            &Path::line(
                // Note: the stroke won't scale with screen zoom.
                frame.center() + Vector::new(-250.0, 100.0),
                frame.center() + Vector::new(250.0, -100.0),
            ),
            Stroke {
                style: Color::WHITE.into(),
                width: 50.0,
                ..Default::default()
            },
        );
  
        vec![frame.into_geometry()]
    }
}
```

![Drawing Shapes](./pic/drawing_shapes.png)

:arrow_right:  Next: [Drawing With Caches](./drawing_with_caches.md)

:blue_book: Back: [Table of contents](./../README.md)
