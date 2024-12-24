use iced::widget::{button, column, Column, TextInput, text_input};

pub fn main() -> iced::Result {
    iced::application("My app", update, view).run()
}

#[derive(Debug, Clone)]

enum Message {
    DoSomething,
}

fn update(_value: &mut u64, _message: Message) {}

fn view(_value: &u64) -> Column<Message> {
    column![
        text_input("Construct from function", ""),
        TextInput::new("Construct from struct", ""),
        // text_input("Enabled text input", self.text3.as_str())
        //     .on_input(|s| MyAppMessage::Update3(s)),
        // text_input("Shorter on_input", self.text4.as_str()).on_input(MyAppMessage::Update4),
        // text_input("Press Ctrl/Cmd + V", self.text5.as_str())
        //     .on_input(MyAppMessage::Update5)
        //     .on_paste(MyAppMessage::Paste5),
        // text(self.info5.as_str()),
        // text_input("Press enter", self.text6.as_str())
        //     .on_input(MyAppMessage::Update6)
        //     .on_submit(MyAppMessage::Submit6),
        // text(self.info6.as_str()),
        // text_input("Password", self.text7.as_str())
        //     .secure(true)
        //     .on_input(MyAppMessage::Update7),
        // text_input("Different font", "").font(Font {
        //     family: Family::Fantasy,
        //     ..Font::DEFAULT
        // }),
        // text_input("Larger text", "").size(24),
        // text_input("With padding", "").padding(20),
        // text_input("Icon", self.text11.as_str())
        //     .icon(Icon {
        //         font: Font::DEFAULT,
        //         code_point: '\u{2705}',
        //         size: None,
        //         spacing: 10.,
        //         side: Side::Left,
        //     })
        //     .on_input(MyAppMessage::Update11),
    ]
    .into()
}
