use iced::Element;

fn main() -> iced::Result {
    iced::application("My App", MyApp::update, MyApp::view).run()
}

struct MyApp {
    page: Box<dyn Page>
}

impl Default for MyApp {
    fn default() -> Self {
        MyApp::new()
    }
}

struct PageB;

impl PageB {
    fn new() -> Self{
        Self
    }
}

impl Page for PageB {
    fn update(&mut self, message: Message) -> Option<Box<dyn Page>> {
        if let Message::PageB(msg) = message {
            match msg {
                PageBMessage::ButtonPressed => return Some(Box::new(PageA::new())),
            }
        }
        None
    }

    fn view(&self) -> iced::Element<Message> {
        column![
            text("Hello!"),
            button("Log out").on_press(Message::PageB(Mb::ButtonPressed)),
        ]
        .into()
    }
}

#[derive(Debug, Clone)]
enum Message {
    PageA(PageAMessage),
    PageB(PageBMessage),
}

trait Page {
    fn update(&mut self, message: Message) -> Option<Box<dyn Page>>;
    fn view(&self) -> iced::Element<'_, Message>;
}

impl MyApp {
    fn new() -> Self {
        Self {
            page: Box::new(PageA::new())
        }
    }

    fn update(&mut self, message: Message) {
        let page = self.page.update(message);
        if let Some(p) = page {
            self.page = p;
        }
    }

    fn view(&self) -> Element<Message> {
        self.page.view()
    }
}