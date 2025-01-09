use iced::{
    advanced::{
        layout,
        renderer::{self, Quad},
        widget, Layout, Widget,
    },
    alignment, mouse,
    widget::{checkbox, column, container, Checkbox},
    Border, Color, Element, Length, Rectangle, Shadow, Size, Theme,
};

fn main() -> iced::Result {
    iced::application("drawing widgets", MyApp::update, MyApp::view).run()
}

struct MyApp {
    highlight: bool,
}

impl Default for MyApp {
    fn default() -> Self {
        MyApp::new()
    }
}

#[derive(Debug, Clone)]
enum Message {
    Highlight(bool),
}

impl MyApp {
    fn new() -> Self {
        Self { highlight: false }
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Highlight(b) => {
                self.highlight = b;
            }
        }
    }

    fn view(&self) -> Element<Message> {
        container(
            column![
                MyWidget::new(self.highlight),
                checkbox("Highlight", self.highlight).on_toggle(Message::Highlight),
            ]
            .spacing(20),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .align_x(alignment::Horizontal::Center)
        .align_y(alignment::Vertical::Center)
        .into()
    }
}

struct MyWidget {
    highlight: bool,
}

impl MyWidget {
    fn new(highlight: bool) -> Self {
        Self { highlight }
    }
}

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
            // Use the `highlight` variable to change the color of our widget in the draw method.
            if self.highlight {
                Color::from_rgb(0.6, 0.8, 1.0)
            } else {
                Color::from_rgb(0.0, 0.2, 0.4)
            },
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
