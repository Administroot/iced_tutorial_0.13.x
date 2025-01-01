use iced::{
    widget::{
        column, row,
        scrollable::{Direction, Scrollbar, Viewport},
        text, Scrollable,
    },
    Element,
};

fn main() -> iced::Result {
    iced::run("scrollable", MyApp::update, MyApp::view)
}

struct MyApp {
    offset4: String,
}

impl Default for MyApp {
    fn default() -> Self {
        MyApp::new()
    }
}

#[derive(Debug, Clone)]
enum Message {
    Scrolled4(Viewport),
}

impl MyApp {
    fn new() -> Self {
        Self {
            offset4: String::new(),
        }
    }

    fn update(&mut self, message: Message) {
        match message {
            // Display absolute offsets.
            Message::Scrolled4(v) => {
                self.offset4 = format!("{} {}", v.absolute_offset().x, v.absolute_offset().y);
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let long_vertical_texts =
            column((0..10).map(|i| text(format!("{} vertical scrollable", i + 1)).into()));
        let long_horizontal_texts =
            row((0..10).map(|i| text(format!("{} horizontal scrollable  ", i + 1)).into()));
        let long_both_texts = column(
            (0..10).map(|i| text(format!("{} vertical and horizontal scrollable", i + 1)).into()),
        );
        let long_both_texts_2 = column(
            (0..10).map(|i| text(format!("{} vertical and horizontal scrollable", i + 1)).into()),
        );
        column!(
            Scrollable::new(long_vertical_texts)
                .width(230)
                .height(105)
                .direction(Direction::Vertical(Scrollbar::new())),
            Scrollable::new(long_horizontal_texts)
                .width(500)
                .height(30)
                .direction(Direction::Horizontal(Scrollbar::new())),
            Scrollable::new(long_both_texts)
                .width(230)
                .height(105)
                .direction(Direction::Both {
                    vertical: Scrollbar::new(),
                    horizontal: Scrollbar::new()
                }),
            column![
                Scrollable::new(long_both_texts_2)
                    .width(230)
                    .height(105)
                    .direction(Direction::Both {
                        vertical: Scrollbar::new(),
                        horizontal: Scrollbar::new()
                    })
                    .on_scroll(Message::Scrolled4),
                text(&self.offset4),
            ],
        )
        .spacing(50)
        .into()
    }
}
