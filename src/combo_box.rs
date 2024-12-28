/* A widget for searching and selecting a single value from a list of options.
This widget is composed by a TextInput that can be filled with the text to search
for corresponding values from the list of options that are displayed as a Menu. */

use std::vec;

use iced::{
    font::Family,
    widget::{
        column, combo_box,
        combo_box::State,
        scrollable, text,
        text_input::{Icon, Side},
        ComboBox,
    },
    Element, Font,
};

fn main() -> iced::Result {
    iced::run("ComboBox", MyApp::update, MyApp::view)
}

struct MyApp {
    state1: State<u32>,
    state2: State<u32>,
    state3: State<Words>,
    my_words: State<Words>,
    state5: State<Words>,
    state6: State<Words>,
    state7: State<Words>,
    state8: State<Words>,
    state9: State<u32>,
    state10: State<u32>,
    state11: State<u32>,
    state12: State<u32>,
    input6: String,

    selected_word: Option<Words>,
    selected5: Option<Words>,
    selected6: Option<Words>,
    selected7: Option<Words>,
    selected8: Option<Words>,
    info8: String,
}

impl Default for MyApp {
    fn default() -> Self {
        MyApp::new()
    }
}

#[derive(Debug, Clone)]
enum Message {
    Selected(Words),
    DoNothing,
    Selected5(Words),
    Select6(Words),
    Select7(Words),
    Select8(Words),

    Hover7(Words),
    Hover8(Words),
    Close8,
    // Input6(String),
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
            state6: State::new(Words::ALL.to_vec()),
            state7: State::new(Words::ALL.to_vec()),
            state8: State::new(Words::ALL.to_vec()),
            state9: State::new(vec![]),
            state10: State::new(vec![]),
            state11: State::new(vec![]),
            state12: State::new(vec![]),

            input6: String::new(),
            selected_word: None,
            selected5: None,
            selected6: None,
            selected7: None,
            selected8: None,
            info8: String::new(),
        }
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Selected(w) => {
                self.selected_word = Some(w);
            }
            Message::DoNothing => {}
            Message::Selected5(w) => {
                self.selected5 = Some(w);
            }
            Message::Select6(w) => {
                self.selected6 = Some(w);
                self.input6 = w.to_string();
            }
            // Message::Input6(s) => {
            //     self.input6 = s;
            // },
            Message::Select7(w) => {
                self.selected7 = Some(w);
            }
            Message::Select8(w) => {
                self.selected8 = Some(w);
            }
            Message::Hover7(w) => {
                self.selected7 = Some(w);
            }
            Message::Hover8(w) => {
                self.selected8 = Some(w);
            }
            Message::Close8 => {
                self.info8 = String::from("Done").into();
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let combo_box_1: ComboBox<'_, u32, Message, _, _> =
            ComboBox::new(&self.state1, "Construct from struct", None, |_| {
                Message::DoNothing
            });
        let combo_box_2 = combo_box(&self.state2, "Construct from function", None, |_| {
            Message::DoNothing
        });
        let combo_box_3 = combo_box(&self.state3, "With list of items", None, |_| {
            Message::DoNothing
        });
        let combo_box_4 = combo_box(
            &self.my_words,
            "Functional combobox (Press Enter or click an option)",
            self.selected_word.as_ref(),
            |s| Message::Selected(s),
        );
        let combo_box_5 = combo_box(
            &self.state5,
            "Shorter parameter (Press Enter or click an option)",
            self.selected5.as_ref(),
            Message::Selected5,
        );
        let combo_box_6 = combo_box(
            &self.state6,
            "Respond to input",
            self.selected6.as_ref(),
            Message::Select6,
        );
        // .on_input(Message::Input6);
        let combo_box_7 = combo_box(
            &self.state7,
            "Respond to option hover",
            self.selected7.as_ref(),
            Message::Select7,
        )
        .on_option_hovered(Message::Hover7);
        let combo_box_8 = combo_box(
            &self.state8,
            "Respond to closing menu",
            self.selected8.as_ref(),
            Message::Select8,
        )
        .on_option_hovered(Message::Hover8)
        .on_close(Message::Close8);
        // Personalization
        let combo_box_9 = combo_box(&self.state9, "Different font", None, |_| Message::DoNothing)
            .font(Font {
                family: Family::Fantasy,
                ..Font::DEFAULT
            });
        let combo_box_10 =
            combo_box(&self.state10, "Larger text", None, |_| Message::DoNothing).size(24.);
        let combo_box_11 =
            combo_box(&self.state11, "With padding", None, |_| Message::DoNothing).padding(20);
        let combo_box_12 =
            combo_box(&self.state12, "Icon", None, |_| Message::DoNothing).icon(Icon {
                font: Font::DEFAULT,
                code_point: '\u{2705}',
                size: None,
                spacing: 10.,
                side: Side::Left,
            });
        let content = column![
            combo_box_1,
            combo_box_2,
            combo_box_3,
            combo_box_4,
            combo_box_5,
            text(&self.input6),
            combo_box_6,
            combo_box_7,
            text(&self.info8),
            combo_box_8,
            combo_box_9,
            combo_box_10,
            combo_box_11,
            combo_box_12,
        ];
        scrollable(content).into()
    }
}
