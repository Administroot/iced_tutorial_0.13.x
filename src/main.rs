use iced::{
    mouse::Cursor,
    widget::{
        canvas::{Cache, Geometry, Path, Program, Stroke},
        column, Canvas,
    },
    Color, Element, Length, Point, Rectangle, Renderer, Theme, Vector,
};

fn main() -> iced::Result {
    iced::application("drawing with caches", MyApp::update, MyApp::view).run()
}

struct MyApp {
    cache: Cache,
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
            cache: Cache::new(),
        }
    }

    fn update(&mut self, _message: Message) {
        todo!()
    }

    fn view(&self) -> Element<Message> {
        column!(
            "A Canvas",
            Canvas::new(self).width(Length::Fill).height(Length::Fill)
        )
        .into()
    }
}

impl<Message> Program<Message> for MyApp {
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
        let geometry = self.cache.draw(renderer, bounds.size(), |frame| {
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
        });
        vec![geometry]
    }
}
