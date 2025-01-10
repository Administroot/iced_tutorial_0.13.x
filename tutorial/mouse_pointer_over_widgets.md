# Mouse Pointer Over Widgets

To change the mouse pointer based on the requirement of our widgets, we can use the [mouse_interaction](https://docs.rs/iced/0.12.1/iced/advanced/widget/trait.Widget.html#method.mouse_interaction) method of [Widget](https://docs.rs/iced/0.12.1/iced/advanced/widget/trait.Widget.html).

```rust
fn mouse_interaction(
    &self,
    _state: &Tree,
    layout: Layout<'_>,
    cursor: mouse::Cursor,
    _viewport: &Rectangle,
    _renderer: &Renderer,
) -> mouse::Interaction {
    if cursor.is_over(layout.bounds()) {
        mouse::Interaction::Pointer
    } else {
        mouse::Interaction::Idle
    }
}
```

The method returns [Interaction](https://docs.rs/iced/0.12.1/iced/mouse/enum.Interaction.html), which specifies the type of the mouse pointer.
In our example, we specify [Interaction::Pointer](https://docs.rs/iced/0.12.1/iced/mouse/enum.Interaction.html#variant.Pointer) when the mouse is over the widget.

The full code is as follows:

```rust
use iced::{
    advanced::{
        graphics::core::event,
        layout,
        renderer::{self, Quad},
        widget, Layout, Widget,
    },
    alignment, mouse,
    widget::{column, container, text},
    Border, Color, Element, Event, Length, Rectangle, Shadow, Size, Theme,
};
  
fn main() -> iced::Result {
    iced::application("mouse pointer over widgets", MyApp::update, MyApp::view).run()
}
  
struct MyApp {
    count: u32,
}

impl Default for MyApp {
    fn default() -> Self {
        MyApp::new()
    }
}

#[derive(Debug, Clone)]
enum Message {
    MyWidgetPressed,
}

impl MyApp {
    fn new() -> Self {
        Self { count: 0 }
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::MyWidgetPressed => {
                self.count += 1;
            }
        }
    }

    fn view(&self) -> Element<Message> {
        container(column![MyWidget::new(Message::MyWidgetPressed), text(self.count)].spacing(20))
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(alignment::Horizontal::Center)
            .align_y(alignment::Vertical::Center)
            .into()
    }
}

struct MyWidget<Message> {
    pressed_message: Message,
}

impl<Message> MyWidget<Message> {
    fn new(pressed_message: Message) -> Self {
        Self { pressed_message }
    }
}
  
impl<Message: Clone, Renderer> Widget<Message, Theme, Renderer> for MyWidget<Message>
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
  
    fn on_event(
        &mut self,
        _state: &mut widget::Tree,
        event: iced::Event,
        layout: Layout<'_>,
        cursor: iced::advanced::mouse::Cursor,
        _renderer: &Renderer,
        _clipboard: &mut dyn iced::advanced::Clipboard,
        shell: &mut iced::advanced::Shell<'_, Message>,
        _viewport: &Rectangle,
    ) -> iced::advanced::graphics::core::event::Status {
        // Check the mouse position
        if cursor.is_over(layout.bounds()) {
            match event {
                // Check the mouse button state
                Event::Mouse(mouse::Event::ButtonPressed(_)) => {
                    // After pressing the button, shell will produce a message
                    shell.publish(self.pressed_message.clone());
                    event::Status::Captured
                }
                _ => event::Status::Ignored,
            }
        } else {
            event::Status::Ignored
        }
    }
  
    fn mouse_interaction(
            &self,
            _state: &widget::Tree,
            layout: Layout<'_>,
            cursor: iced::advanced::mouse::Cursor,
            _viewport: &Rectangle,
            _renderer: &Renderer,
        ) -> iced::advanced::mouse::Interaction {
        if cursor.is_over(layout.bounds()){
            mouse::Interaction::Pointer
        } else {
            mouse::Interaction::Idle
        }
    }
}
  
impl<'a, Message, Renderer> From<MyWidget<Message>> for Element<'a, Message, Theme, Renderer>
where
    Message: 'a + Clone,
    Renderer: iced::advanced::Renderer,
{
    fn from(widget: MyWidget<Message>) -> Self {
        Self::new(widget)
    }
}
```

![Mouse Pointer Over Widgets](./pic/mouse_pointer_over_widgets.png)

:arrow_right:  Next: [Texts In Widgets](./texts_in_widgets.md)

:blue_book: Back: [Table of contents](./../README.md)
