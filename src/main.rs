use iced::{
    advanced::{
        layout,
        renderer::{self, Quad},
        widget::{self, Tree},
        Layout, Text, Widget,
    },
    alignment,
    widget::{
        container, svg::Handle, text::{LineHeight, Shaping, Wrapping}
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
        container(MyWidgetWithImage::new())
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(alignment::Horizontal::Center)
            .align_y(alignment::Vertical::Center)
            .into()
    }
}

struct MyWidgetWithImage {
    handle: Handle,
};

impl MyWidgetWithImage {
    const CONTENT: &'static str = "  My Widget  ";

    fn new() -> Self {
        Self {
            handle: Handle::from_path("ferris.png")
        }
    }
}

impl<Message, Renderer> Widget<Message, Theme, Renderer> for MyWidgetWithImage
where
    Renderer: iced::advanced::Renderer + iced::advanced::image::Renderer<Handle = Handle>,
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
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        iced::widget::image::layout(renderer, limits, &self.handle, Length::Fixed(200.), Length::Fixed(200.), iced::ContentFit::Contain, iced::Rotation::default())
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
                    color: Color::from_rgb(1.0, 0.66, 0.6),
                    width: 1.0,
                    radius: 10.0.into(),
                },
                shadow: Shadow::default(),
            },
            Color::BLACK,
        );

        iced::widget::image::draw(renderer, layout, &self.handle, iced::ContentFit::Contain, iced::, rotation, opacity);
    }
}

impl<Message, Renderer> From<MyWidgetWithImage> for Element<Message, Theme, Renderer>
where
    Renderer: iced::advanced::Renderer + iced::advanced::image::Renderer,
{
    fn from(widget: MyWidgetWithImage) -> Self {
        Self::new(widget)
    }
}
