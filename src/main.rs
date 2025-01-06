use iced::{
    widget::{button, column, text, text_input},
    Element,
};

fn main() -> iced::Result {
    iced::application("My App", MyApp::update, MyApp::view).run()
}

impl Default for MyApp {
    fn default() -> Self {
        MyApp::new()
    }
}

// Page B
#[derive(Debug, Clone)]
enum PageBMessage {
    ButtonPressed,
}

type Mb = PageBMessage;

struct PageB {
    name: String
}

impl PageB {
    fn new(name: String) -> Self {
        Self { name }
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
            text(format!("hello, {}!", self.name)),
            button("Log out").on_press(Message::PageB(Mb::ButtonPressed)),
        ]
        .into()
    }
}

// Page A
#[derive(Debug, Clone)]
enum PageAMessage {
    ButtonPressed,
}

type Ma = PageAMessage;

struct PageA {}

impl PageA {
    fn new() -> Self {
        Self {}
    }
}

impl Page for PageA {
    fn update(&mut self, message: Message) -> Navigation {
        if let Message::PageA(msg) = message {
            match msg {
                PageAMessage::ButtonPressed => {
                    return Navigation::GoTo(Box::new(PageB::new(1)));
                }
            }
        }
        Navigation::None
    }

    fn view(&self) -> iced::Element<Message> {
        column![
            text("Start"),
            button("Next").on_press(Message::PageA(Ma::ButtonPressed))
        ]
        .into()
    }
}

#[derive(Debug, Clone)]
enum Message {
    PageA(PageAMessage),
    PageB(PageBMessage),
}

enum Navigation {
    GoTo(Box<dyn Page>),
    Back,
    None,
}

trait Page {
    fn update(&mut self, message: Message) -> Navigation;
    fn view(&self) -> iced::Element<'_, Message>;
}

// MyApp
struct MyApp {
    pages: Vec<Box<dyn Page>>,
}

impl MyApp {
    fn new() -> Self {
        Self {
            pages: vec![Box::new(PageA::new())],
        }
    }

    fn update(&mut self, message: Message) {
        let navigation = self.pages.last_mut().unwrap().update(message);
        match navigation {
            Navigation::GoTo(p) => self.pages.push(p),
            Navigation::Back => {
                if self.pages.len() > 1 {
                    self.pages.pop();
                }
            }
            Navigation::None => {}
        }
    }

    fn view(&self) -> Element<Message> {
        self.pages.last().unwrap().view()
    }
}
