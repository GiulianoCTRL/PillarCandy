//! Application used to find passages and related laws from riksdagen.se

use iced::{
    button, text_input, Align, Button, Column, Element, Length, Row, Sandbox, Settings, Text,
    TextInput,
};

use pillar_candy::Law;

#[derive(Default)]
struct PillarCandy {
    search_confirmed: button::State,
    search_input: text_input::State,
    search_text: String,
    result_text: String,
}

#[derive(Debug, Clone)]
enum Message {
    SearchChanged(String),
    ConfirmSearch,
}

impl Sandbox for PillarCandy {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("PillarCandy")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::SearchChanged(s) => {
                self.search_text = s;
            }
            Message::ConfirmSearch => {
                self.result_text = Law::from_string(&self.search_text).text();
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        let search_field = TextInput::new(
            &mut self.search_input,
            "Search for law",
            &self.search_text,
            Message::SearchChanged,
        );

        let search_button = Button::new(&mut self.search_confirmed, Text::new("Search"));

        let search_row = Row::new()
            .spacing(20)
            .align_items(Align::Start)
            .push(search_field)
            .push(search_button.on_press(Message::ConfirmSearch));

        // TODO: Scrollable seems to be extremly laggy, but Text doesn't display the full text.
        let result = Text::new(&self.result_text)
            .width(Length::Fill)
            .height(Length::Fill);

        Column::new()
            .padding(20)
            .spacing(20)
            .align_items(Align::Start)
            .push(search_row)
            .push(result)
            .into()
    }
}

pub fn main() -> iced::Result {
    PillarCandy::run(Settings::default())
}
