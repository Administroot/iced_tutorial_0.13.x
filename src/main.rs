/* A widget for searching and selecting a single value from a list of options.
This widget is composed by a TextInput that can be filled with the text to search
for corresponding values from the list of options that are displayed as a Menu. */

use iced::{widget::{column, combo_box, combo_box::State, ComboBox, scrollable}, Element};

fn main() -> iced::Result {
    iced::run("ComboBox", MyApp::update, MyApp::view)
}

struct MyApp {
    state1: State<u32>,
    state2: State<u32>,
    state3: State<Words>,
    my_words: State<Words>,
    state5: State<Words>,
    selected_word: Option<Words>, 
}

impl Default for MyApp {
    fn default() -> Self {
        MyApp::new()
    }
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Selected(Words),
    DoNothing,
}

#[derive(Debug, Clone, Copy, Default)]
pub enum Words {
    #[default]
    Aa,
    Bb,
    Cc,
}

impl Words {
    const ALL: [Words; 3] = [Words::Aa, Words::Bb, Words::Cc];
}

impl std::fmt::Display for Words {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Words::Aa => "Aa",
                Words::Bb => "Bb",
                Words::Cc => "Cc",
            }
        )
    }
}

impl MyApp {
    fn new() -> Self {
        Self {
            my_words: State::new(Words::ALL.to_vec()),
            state1: State::new(vec![]),
            state2: State::new(vec![]),
            state3: State::new(Words::ALL.to_vec()),
            state5: State::new(Words::ALL.to_vec()),
            selected_word: None,
        }
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Selected(w) => {
                self.selected_word = Some(w);
            },
            Message::DoNothing => {},
        }
    }

    fn view(&self) -> Element<Message> {
        let combo_box_1: ComboBox<'_, u32, Message, _, _> = ComboBox::new(
            &self.state1,
            "Construct from struct",
            None,
            |_| {Message::DoNothing},
        );
        let combo_box_2 = combo_box(
            &self.state2, 
            "Construct from function", 
            None,
            |_| {Message::DoNothing},
        );
        let combo_box_3 = combo_box(
            &self.state3,
            "With list of items",
            None,
            |_| {Message::DoNothing},
        );
        let combo_box_4 = combo_box(
            &self.my_words, 
            "Functional combobox (Press Enter or click an option)", 
            self.selected_word.as_ref(), 
            |s| Message::Selected(s),
        );
        let combo_box_5 = combo_box(
            &self.state5,
            "Shorter parameter (Press Enter or click an option)",
            self.selected_word.as_ref(),
            Message::Selected,
        );
        let content = column![
            combo_box_1,
            combo_box_2,
            combo_box_3,
            combo_box_4,
            combo_box_5,
        ];
        scrollable(content).into()
    }
}