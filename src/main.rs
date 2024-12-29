use iced::{
    widget::{
        column, slider, text, Slider
    },
    Element
};

fn main() -> iced::Result {
    iced::run("My First App", MyApp::update, MyApp::view)
}

struct MyApp {
    value3: u32,
    value4: u32,
    value5: u32,
}

impl Default for MyApp {
    fn default() -> Self {
        MyApp::new()
    }
}

#[derive(Debug, Clone)]
enum MyAppMessage {
    DoNothing,
    Update3(u32),
    Update4(u32),
}

impl MyApp {
    fn new() -> Self {
        Self {
            value3: 50,
            value4: 50,
        }
    }

    fn update(&mut self, message: MyAppMessage) {
        match message {
            MyAppMessage::DoNothing => {},
            MyAppMessage::Update3(v) => {
                self.value3 = v;
            },
            MyAppMessage::Update4(v) => {
                self.value4 = v;
            }
        }
    }

    fn view(&self) -> Element<MyAppMessage> {
        column!(
            text("Construct from struct"),
            Slider::new(0..=100, 50, |_| MyAppMessage::DoNothing),
            text("Construct from function"),
            slider(0..=100, 50, |_| MyAppMessage::DoNothing),
            text("Functional slider"),
            slider(0..=100, self.value3, |v| MyAppMessage::Update3(v)),
            text("Shorter parameter"),
            slider(0..=100, self.value4, MyAppMessage::Update4),
            text("Different step"),
            slider(0..=100, , on_change)
        ).into()
    }
}
