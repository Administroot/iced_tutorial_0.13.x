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
    Loaded<Vec<u8>>,
}

impl MyApp {
    fn new() -> Self {
        Self {
            image_handle: None,
            show_container: false,
        }
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Load => {
                self.show_container = true;
                return Task::perform(
                    async {
                        let mut file = File::open("ferris.png")
                    }
                )
            }
        }
    }

    fn view(&self) -> Element<Message> {
        column![button("Load").on_press(Message::Load),
        if self.show_container {
            match &self.image_handle{
                Some(h) => container(image(h.clone())),
                None => container("Loading"),
            }
        } else {
            container("")
        }
        ]
        .padding(20)
        .into()
    }
}