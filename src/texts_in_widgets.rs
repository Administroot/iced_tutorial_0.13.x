use iced::{
    advanced::{
        layout,
        renderer::{self, Quad},
        widget::{self, Tree},
        Layout, Text, Widget,
    },
    alignment,
    widget::{
        container,
        text::{LineHeight, Shaping, Wrapping},
    },
    Border, Color, Element, Length, Rectangle, Shadow, Size, Theme,
};

fn main() -> iced::Result {
    iced::application("texts in widgets", MyApp::update, MyApp::view).run()
}

struct MyApp {}

impl Default for MyApp {
    fn default() -> Self {
        MyApp::new()
    }
}

#[derive(Debug, Clone)]
enum Message {
    _MyWidgetPressed,
}

impl MyApp {
    fn new() -> Self {
        Self {}
    }

    fn update(&mut self, _message: Message) {}

    fn view(&self) -> Element<Message> {
        container(MyWidgetWithText::new())
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(alignment::Horizontal::Center)
            .align_y(alignment::Vertical::Center)
            .into()
    }
}

struct MyWidgetWithText;

impl MyWidgetWithText {
    const CONTENT: &'static str = "  My Widget  ";

    fn new() -> Self {
        Self
    }
}

impl<Message, Renderer> Widget<Message, Theme, Renderer> for MyWidgetWithText
where
    Renderer: iced::advanced::Renderer + iced::advanced::text::Renderer,
{
    fn size(&self) -> Size<Length> {
        Size {
            width: Length::Shrink,
            height: Length::Shrink,
        }
    }

    fn layout(
        &self,
        _tree: &mut Tree,
        _renderer: &Renderer,
        _limits: &layout::Limits,
    ) -> layout::Node {
        layout::Node::new(Size {
            width: 200.0,
            height: 100.0,
        })
    }

    fn draw(
        &self,
        _tree: &widget::Tree,
        renderer: &mut Renderer,
        _theme: &Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        _cursor: iced::advanced::mouse::Cursor,
        viewport: &Rectangle,
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

        let bounds = layout.bounds();
        renderer.fill_text(
            Text {
                content: Self::CONTENT.to_owned(),
                bounds: bounds.size(),
                size: renderer.default_size(),
                line_height: LineHeight::default(),
                font: renderer.default_font(),
                horizontal_alignment: alignment::Horizontal::Center,
                vertical_alignment: alignment::Vertical::Center,
                shaping: Shaping::default(),
                wrapping: Wrapping::default(),
            },
            bounds.center(),
            Color::from_rgb(0.6, 0.8, 1.0),
            *viewport,
        );
    }
}

impl<'a, Message, Renderer> From<MyWidgetWithText> for Element<'a, Message, Theme, Renderer>
where
    Renderer: iced::advanced::Renderer + iced::advanced::text::Renderer,
{
    fn from(widget: MyWidgetWithText) -> Self {
        Self::new(widget)
    }
}
