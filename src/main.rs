use iced::{
    advanced::{
        layout,
        renderer::{self, Quad},
        widget::Tree,
        Layout, Widget,
    },
    alignment, mouse,
    widget::container,
    Alignment, Border, Color, Element, Length, Rectangle, Shadow, Size, Theme,
};

fn main() -> iced::Result {
    iced::application("widgets_with_children", MyApp::update, MyApp::view).run()
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
        container(MyWidgetOuter::new())
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(alignment::Horizontal::Center)
            .align_y(alignment::Vertical::Center)
            .into()
    }
}

struct MyWidgetOuter<'a, Message, Renderer> {
    inner_widget: Element<'a, Message, Theme, Renderer>
}

impl<'a, Message, Renderer> MyWidgetOuter<'a, Message, Renderer> 
where Renderer: iced::advanced::Renderer,
{
    fn new(inner_widget: Element<'a, Message, Theme, Renderer>) -> Self {
        Self {
            inner_widget
        }
    }
}

impl<Message, Renderer> Widget<Message, Theme, Renderer> for MyWidgetOuter
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
        tree: &mut Tree,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        /* Yet, the inner_widget is of type Element.
        So, we have to cast it as Widget by the as_widget method.*/
        let mut child_node = self.inner_widget.as_widget().layout(&mut tree.children[0], renderer, limits);
        let size_of_this_node = child_node.size().expand(Size::new(50., 50.));
        child_node = child_node.align(Alignment::Center, Alignment::Center, size_of_this_node);
        layout::Node::with_children(size_of_this_node, vec![child_node])
    }

    fn draw(
        &self,
        state: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: iced::advanced::mouse::Cursor,
        viewport: &Rectangle,
    ) {
        renderer.fill_quad(
            Quad {
                bounds: layout.bounds(),
                border: Border {
                    color: Color::from_rgb(0.6, 0.93, 1.0),
                    width: 1.0,
                    radius: 10.0.into(),
                },
                shadow: Shadow::default(),
            },
            Color::from_rgb(0.0, 0.33, 0.4),
        );

        self.inner_widget.as_widget().draw(&state.children[0], renderer, theme, style, layout.children().next().unwrap(), cursor, viewport);
    }

    fn on_event(
            &mut self,
            state: &mut Tree,
            event: iced::Event,
            layout: Layout<'_>,
            cursor: iced::advanced::mouse::Cursor,
            renderer: &Renderer,
            clipboard: &mut dyn iced::advanced::Clipboard,
            shell: &mut iced::advanced::Shell<'_, Message>,
            viewport: &Rectangle,
        ) -> iced::advanced::graphics::core::event::Status {
        let inner_widget = &mut self.inner_widget as &mut dyn Widget<Message, Theme, Renderer>;
        inner_widget.on_event(state, event, layout, cursor, renderer, clipboard, shell, viewport)
    }

    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(self.inner_widget.as_widget())]
    }

    fn 
}

impl<'a, Message, Renderer> From<MyWidgetOuter> for Element<'a, Message, Theme, Renderer>
where
    Renderer: iced::advanced::Renderer,
{
    fn from(widget: MyWidgetOuter) -> Self {
        Self::new(widget)
    }
}
