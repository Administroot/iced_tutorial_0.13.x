use iced::{
    widget::{button, column, container, image, image::Handle},
    Element, Task,
};
use tokio::{fs::File, io::AsyncReadExt};

fn main() -> iced::Result {
    iced::application("loading images asynchronously", MyApp::update, MyApp::view).run()
}

struct MyApp {
    image_handle: Option<Handle>,
    show_container: bool,
}

impl Default for MyApp {
    fn default() -> Self {
        MyApp::new().0
    }
}

#[derive(Debug, Clone)]
enum Message {
    Load,
    Loaded(Vec<u8>),
}

impl MyApp {
    fn new() -> (Self, Task<Message>) {
        (
            Self {
                image_handle: None,
                show_container: false,
            },
            Task::none(),
        )
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Load => {
                self.show_container = true;
                return Task::perform(
                    async {
                        let mut file = File::open("ferris.png").await.unwrap();
                        let mut buffer = Vec::new();
                        file.read_to_end(&mut buffer).await.unwrap();
                        buffer
                    },
                    Message::Loaded,
                );
            }
            Message::Loaded(data) => {
                self.image_handle = Some(Handle::from_bytes(data));
            }
        }
        Task::none()
    }

    fn view(&self) -> Element<Message> {
        column![
            button("Load").on_press(Message::Load),
            if self.show_container {
                match &self.image_handle {
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
