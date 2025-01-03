use iced::{
    widget::{button, column, text},
    Element,
};

fn main() -> iced::Result {
    iced::application("My First App", MyApp::update, MyApp::view).run()
}

enum Page {
    A, 
    B
}

struct MyApp {
    page: Page
}

impl Default for MyApp {
    fn default() -> Self {
        MyApp::new()
    }
}

#[derive(Debug, Clone)]
enum Message {
    GoToBButtonPressed(Page),
    GoToAButtonPressed(Page)
}

impl MyApp {
    fn new() -> Self {
        Self {
            page: Page::A,
        }
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::GoToAButtonPressed => Page::A,
            Message::GoToBButtonPressed => Page::B,
        }
    }

    fn view(&self) -> Element<Message> {
        match self.page {
            Page::A => {
                column![
                    text("Page A"),
                    button("Go to B").on_press(Message::GoToBButtonPressed),
                ]
            },
            Page::B => {}
        }
    }
}