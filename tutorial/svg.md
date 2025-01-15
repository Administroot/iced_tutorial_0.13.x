# Svg

The [Svg](https://docs.rs/iced/0.12.1/iced/widget/svg/struct.Svg.html) widget is able to display an [SVG](https://en.wikipedia.org/wiki/SVG) image.
It has two methods of constructions.
We can set how to fit the image content into the widget bounds.

To use the widget, we have to enable the [svg](https://docs.rs/crate/iced/0.12.1/features#svg) feature.
The `Cargo.toml` dependencies should look like this:

```toml
[dependencies]
iced = { version = "0.13.1", features = ["svg"] }
```

Let's add an [SVG](https://en.wikipedia.org/wiki/SVG) image named `pic.svg` into the project root directory, i.e., the image has the path `my_project/pic.svg` where `my_project` is the name of our project.
The file `pic.svg` contains the following content:

```svg
<svg viewBox="0 0 400 300" xmlns="http://www.w3.org/2000/svg">
    <rect width="400" height="300" style="fill:rgb(100,130,160)"/>
    <circle cx="200" cy="150" r="100" style="fill:rgb(180,210,240)"/>
</svg>
```

Our example is as follows:

```rust
use iced::{
    widget::{column, svg, svg::Handle, text, Svg},
    ContentFit, Element,
};

fn main() -> iced::Result {
    iced::run("svg", MyApp::update, MyApp::view)
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
            text("Construct from struct"),
            Svg::from_path("pic.svg"),
            text("Construct from function"),
            svg(Handle::from_path("pic.svg")),
            text("Different content fit"),
           svg(Handle::from_path("pic.svg")).content_fit(ContentFit::None),
        )
        .into()
    }
}
```

![Svg](./pic/svg.png)

:arrow_right:  Next: [Width And Height](./width_and_height.md)

:blue_book: Back: [Table of contents](./../README.md)
