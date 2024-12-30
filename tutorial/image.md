# Image

The [Image](https://docs.rs/iced/0.12.1/iced/widget/image/struct.Image.html) widget is able to display an image.
It has two methods of constructions.
We can set how to fit the image content into the widget bounds.

To use the widget, we have to enable the [image](https://docs.rs/crate/iced/0.12.1/features#image) feature.
The `Cargo.toml` dependencies should look like this:

```toml
[dependencies]
iced = {version = "0.13.1", features = ["image"] }
```

Assume we have an image named `ferris.png` in the project root directory, i.e., the image has the path `my_project/ferris.png` where `my_project` is the name of our project.

```rust
use iced::{
    widget::{column, image, text, Image},
    Element,
    ContentFit,
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
            text("Construct from struct"),
            Image::new("ferris.png"),
            text("Construct from function"),
            image("ferris.png"),
            text("Different content fit"),
            image("ferris.png").content_fit(ContentFit::Cover)
        ).into()
    }
}
```

![Image](./pic/image.png)

:arrow_right:  Next: [Svg](./svg.md)

:blue_book: Back: [Table of contents](./../README.md)
