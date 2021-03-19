//! Application used to find passages and related laws from riksdagen.se

use iced::{
    button, scrollable, text_input, Align, Button, Column, Element, Length, Row, Sandbox,
    Scrollable, Settings, Text, TextInput,
};

/// This is the app itself, currently it contains the states for the search
/// button and search input, as well as information about the search text itself
/// and the result from the law query.
#[derive(Default)]
struct PillarCandy {
    search_confirmed: button::State,
    search_input: text_input::State,
    result_state: scrollable::State,
    search_text: String,
    result_text: String,
}

/// Message enum notifying the app if the search changes or if the search has been confirmed
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

    /// When the search text changes the search text will be updated,
    /// a request will only be sent once the search button has been pressed.
    fn update(&mut self, message: Message) {
        match message {
            Message::SearchChanged(s) => {
                self.search_text = s;
            }
            Message::ConfirmSearch => {
                // Bottom is just playing around, will be moved and refactored into lib.rs
                // Works for now but non existend tags need to be handled
                self.result_text = String::from("Tbd");
            }
        }
    }

    /// The GUI layout
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

        // TODO: Scrollable seems to be extremly laggy, but Text doesn't display the full text. Workaround?
        let result = Scrollable::new(&mut self.result_state)
            .padding(10)
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
