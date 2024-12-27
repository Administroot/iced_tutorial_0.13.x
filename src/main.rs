/* A widget for searching and selecting a single value from a list of options.
This widget is composed by a TextInput that can be filled with the text to search
for corresponding values from the list of options that are displayed as a Menu. */

use std::default;

use iced::widget::{column, combo_box::State, combo_box, Column, ComboBox, Text};

fn main() {
    iced::run("ComboBox", MyApp::update, MyApp::view);
}

struct MyApp {
    my_words: combo_box::State<Words>,
    selected_word: Option<Words>, 
    text: String,
}

impl Default for MyApp {
    fn default() -> Self {
        MyApp::new();
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub enum Words {
    #[default]
    Aa,
    Bb,
    Cc,
}

impl Words {
    con
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

    }
}