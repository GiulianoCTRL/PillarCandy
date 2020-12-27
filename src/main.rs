//! Application used to find passages and related laws from riksdagen.se

use iced::{
    Sandbox, Element, Settings,
    Column, Row,
    Align, Length,
    Text,
    TextInput, text_input,
    Scrollable, scrollable,
    Button, button,
};

#[derive(Default)]
struct PillarCandy {
    search_confirmed: button::State,
    search_input: text_input::State,
    search_text: String,
    result: scrollable::State,
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
                self.result_text = self.search_text.to_string();
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        let search_field = TextInput::new(
            &mut self.search_input, 
            "Search for law", 
            &mut self.search_text, 
            Message::SearchChanged
        );

        let search_button = Button::new(
            &mut self.search_confirmed, 
            Text::new("Search")
        );

        let search_row = Row::new()
            .spacing(20)
            .align_items(Align::Start)
            .push(search_field)
            .push(search_button.on_press(Message::ConfirmSearch));

        let result = Scrollable::new(&mut self.result)
            .width(Length::Fill)
            .height(Length::Fill)
            .push(Text::new(&self.result_text));

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