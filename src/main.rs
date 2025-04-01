mod enums;
mod structs;

use enums::Rate;
use structs::Clicker;

use iced::widget::{button, column, row, text, text_input};
use iced::{Alignment, Element, Font, Sandbox, Settings, window};

fn main() -> iced::Result {
    // Include font bytes
    const CUSTOM_FONT: &[u8] = include_bytes!("../assets/fonts/0xProtoNerdFont-Regular.ttf");

    let window_settings = window::Settings {
        size: (400, 300),
        decorations: true,
        resizable: true,
        ..window::Settings::default()
    };

    let settings = Settings {
        default_font: Font::with_name("0xProto Nerd Font"),
        default_text_size: 16.0,
        window: window_settings,
        flags: (),
        ..Settings::default()
    };

    Clicker::run(settings)
}

impl Sandbox for Clicker {
    type Message = Rate;

    fn new() -> Self {
        Self {
            clicks: 0,
            cps: 0,
            cps_input: String::new(),
        }
    }

    fn title(&self) -> String {
        String::from("frenzy")
    }

    fn update(&mut self, rate: Rate) {
        match rate {
            Rate::Increment => {
                self.cps += 1;
            }
            Rate::Decrement => {
                self.cps -= 1;
            }
            Rate::CpsInputChanged(value) => {
                self.cps_input = value;
            }
            Rate::SetRate => {
                // Try to parse the input value as an integer
                if let Ok(value) = self.cps_input.parse::<i32>() {
                    self.cps = value;
                }
            }
        }
    }

    fn view(&self) -> Element<Rate> {
        column![
            // First row
            row![text(format!("Count: {}", self.cps)).size(50)]
                .padding(20)
                .align_items(Alignment::Center),
            // Second row
            row![
                button("Increment").on_press(Rate::Increment),
                button("Decrement").on_press(Rate::Decrement),
            ]
            .padding(20)
            .spacing(10) // Add spacing between elements in the second row
            .align_items(Alignment::Center),
            // Third row
            row![
                text_input("Enter a number", &self.cps_input)
                    .on_input(Rate::CpsInputChanged)
                    .padding(10),
                button("Set Rate").on_press(Rate::SetRate)
            ]
            .padding(20)
            .spacing(10) // Add spacing between elements in the second row
            .align_items(Alignment::Center),
        ]
        .spacing(10) // Add spacing between the rows
        .align_items(Alignment::Center)
        .into()
    }
}
