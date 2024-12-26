/* A widget for searching and selecting a single value from a list of options.
This widget is composed by a TextInput that can be filled with the text to search
for corresponding values from the list of options that are displayed as a Menu. */

use iced::{widget::{column, combo_box::{self, State}, Column, ComboBox}, Application, Settings};

pub fn main() -> iced::Result {
    // iced::application("My app", update, view).run()
    MyApp::run(Settings::default())
}

#[derive(Debug, Clone)]

enum MyAppMessage {
    DoNothing,
    Select4(String),
    Select5(String),
    Select6(String),
    Input6(String),
}

#[derive(Default)]
struct MyApp {
    state1: State<u32>,
    state2: State<u32>,
    state3: State<String>,
    state4: State<String>,
    state5: State<String>,
    state6: State<String>,
    select4: Option<String>,
    select5: Option<String>,
    select6: Option<String>,
    input6: String,
}


impl Application for MyApp {
    type Message = MyAppMessage;

    fn new() -> Self {
        Self {
            state4: State::new(["Aa", "Ab", "Ba", "Bb"].map(|s| s.to_string()).to_vec()),
        }
    }
}

fn update(state: &mut MyApp, message: MyAppMessage) {
    match message {
        MyAppMessage::DoNothing => {},
        MyAppMessage::Select4(s) => {
            state.select4 = Some(s);
        },
        MyAppMessage::Select5(s) => {
            state.select5 = Some(s);
        },
        MyAppMessage::Select6(s) => {
            state.select6 = Some(s);
        }
        MyAppMessage::Input6(s) => {
            state.input6 = s;
        }
    }
}

fn view(myapp: &MyApp) -> Column<MyAppMessage> {
    column![
        ComboBox::new(&myapp.state1, "Construct from struct", None, |_| {
            MyAppMessage::DoNothing
        }),
        combo_box(&myapp.state2, "Construct from function", None, |_| {
            MyAppMessage::DoNothing
        }),
        combo_box(&myapp.state3, "With list of items", None, |_| {
            MyAppMessage::DoNothing
        }),
        combo_box(
            &myapp.state4,
            "Functional combobox (Press Enter or click an option)",
            myapp.select4.as_ref(),
            |s| MyAppMessage::Select4(s)
        ),
        combo_box(
            &myapp.state5,
            "Shorter parameter (Press Enter or click an option)",
            myapp.select5.as_ref(),
            MyAppMessage::Select5
        ),
        combo_box(
            &myapp.state6,
            "Respond to input",
            myapp.select6.as_ref(),
            MyAppMessage::Select6
        )
        .on_input(MyAppMessage::Input6),
    ]
    .into()
}
