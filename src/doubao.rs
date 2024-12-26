use iced::{
    widget::{column, combo_box, combo_box::State, Column, ComboBox},
    executor,
    theme,
    window,
    Application,
    Command,
    Element,
    Settings,
    Theme,
};

#[derive(Debug, Clone)]
enum Message {
    Selected(Option<String>),
}

struct MyApp {
    state: State<String>,
    selected: Option<String>,
}

impl Application for MyApp {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Self::Message>) {
        let options = vec![
            "Option 1".to_string(),
            "Option 2".to_string(),
            "Option 3".to_string(),
        ];

        let initial_selection = Some("Option 2".to_string());

        let state = State::with_selection(options, initial_selection.clone());

        (
            Self {
                state,
                selected: initial_selection,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("ComboBox Example")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::Selected(selected) => {
                self.selected = selected;
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        let combo_box = ComboBox::new(
            &self.state,
            "Select an option",
            self.selected.clone(),
            Message::Selected,
        );

        column![combo_box].into()
    }
}

pub fn main() -> iced::Result {
    MyApp::run(Settings {
        window: window::Settings {
            size: (300, 100),
            ..window::Settings::default()
        },
        antialiasing: true,
        ..Settings::default()
    })
}
