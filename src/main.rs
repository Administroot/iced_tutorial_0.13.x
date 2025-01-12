use iced::{
    image::Handle, widget::{button, column, text}, Element
};

fn main() -> iced::Result {
    iced::application("My First App", MyApp::update, MyApp::view).run()
}

struct MyApp {
    image_handle: Option<Handle>,
    show_container: bool,
}

impl Default for MyApp {
    fn default() -> Self {
        MyApp::new()
    }
}

#[derive(Debug, Clone)]
enum Message {
    Load,
}

impl MyApp {
    fn new() -> Self {
        Self {
            image_handle: None,
            show_container: false,
        }
    }

    fn update(&mut self, _message: Message) {
        todo!()
    }

    fn view(&self) -> Element<Message> {
        column![button("Load").on_press(Message::Load),]
    }
}