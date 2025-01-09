use iced::{
    advanced::{Widget, widget, Layout, Renderer}, 
    Length, Theme, Size
};

fn main() -> iced::Result {
    iced::application("drawing with caches", MyApp::update, MyApp::view).run()
}

struct MyWidget;

impl<Message, Renderer> Widget<Message, Theme, Renderer> for MyWidget
where
    Renderer: Renderer,
{
    fn size(&self) -> Size<Length> {
        Size {
            width: Length::Shrink,
            height: Length::Shrink,
        }
    }

    fn layout(
        &self,
        tree: &mut widget::Tree,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        
    }

    fn draw(
        &self,
        tree: &widget::Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        todo!()
    }
}