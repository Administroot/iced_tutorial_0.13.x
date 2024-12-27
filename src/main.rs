/* A widget for searching and selecting a single value from a list of options.
This widget is composed by a TextInput that can be filled with the text to search
for corresponding values from the list of options that are displayed as a Menu. */

use iced::{widget::{column, combo_box, scrollable}, Element};

fn main() -> iced::Result {
    iced::run("ComboBox", MyApp::update, MyApp::view)
}

struct MyApp {
    my_words: combo_box::State<Words>,
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
            my_words: combo_box::State::new(Words::ALL.to_vec()),
            selected_word: None,
        }
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Selected(w) => {
                self.selected_word = Some(w);
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let combo_box = combo_box(
            &self.my_words, 
            "Functional combobox (Press Enter or click an option)", 
            self.selected_word.as_ref(), 
            Message::Selected,
        );
        let content = column![
            combo_box,
        ];
        scrollable(content).into()
    }
}