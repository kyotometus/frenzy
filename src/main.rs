mod structs;
mod enums;

use structs::Counter;
use enums::Message;

use iced::widget::{button, column, text};
use iced::{window, Alignment, Element, Sandbox, Settings};

fn main() -> iced::Result {
    let window_settings = window::Settings {
        size: (400, 300),
        decorations: true,
        resizable: true,
        ..window::Settings::default()
    };

    let settings = Settings {
        window: window_settings,
        flags: (),
        ..Settings::default()
    };

    Counter::run(settings)
}

impl Sandbox for Counter {
    type Message = Message;

    fn new() -> Self {
        Self { value: 0 }
    }

    fn title(&self) -> String {
        String::from("Frenzy")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Increment => {
                self.value += 1;
            }
            Message::Decrement => {
                self.value -= 1;
            }
        }
    }

    fn view(&self) -> Element<Message> {
        column![
            text(format!("Count: {}", self.value)).size(50),
            button("Increment").on_press(Message::Increment),
            button("Decrement").on_press(Message::Decrement),
        ]
        .padding(20)
        .align_items(Alignment::Center)
        .into()
    }
}