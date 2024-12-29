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
    value6: u32,
    value7: u32,
    value8: u32,
    released_value_7: u32,
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
    Update5(u32),
    Update6(u32),
    Update7(u32),
    
    Release7,
}

impl MyApp {
    fn new() -> Self {
        Self {
            value3: 50,
            value4: 50,
            value5: 50,
            value6: 50,
            value7: 50,
            released_value_7: 50,
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
            },
            MyAppMessage::Update5(v) => {
                self.value5 = v;
            },
            MyAppMessage::Update6(v) => {
                self.value6 = v;
            },
            MyAppMessage::Update7(v) => {
                self.value7 = v;
            },
            MyAppMessage::Release7 => {
                self.released_value_7 = self.value7;
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
            slider(0..=100, self.value5, MyAppMessage::Update5).step(10u32),
            text("Different step when a shift key is pressed"),
            slider(0..=100, self.value6, MyAppMessage::Update6).shift_step(10u32),
            text(format!("React to mouse release: {}", self.released_value_7)),
            slider(0..=100, self.value7, MyAppMessage::Update7).on_release(MyAppMessage::Release7),
            text("Press Ctrl (or Command) and click to return to the default value"),
            slider(0..=100, self.value8, MyAppMessage::Update8).default(30u32),
        ).into()
    }
}
