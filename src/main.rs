use iced::widget::Column;
use iced::{Element, Sandbox};
use iced_native::widget::button;
use iced_native::Theme;

mod levels;

struct MicroArchLevel {
    cpu_features: Vec<String>,
}

#[derive(Debug, Clone)]
enum Message {
    ClickedScan,
}

impl Sandbox for MicroArchLevel {
    type Message = Message;

    fn new() -> Self {
        MicroArchLevel {
            cpu_features: vec![],
        }
    }

    fn title(&self) -> String {
        String::from("Easy Microarchitecture Level")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::ClickedScan => {}
        }
    }

    fn view(&self) -> Element<Self::Message> {
        let scan_button = button("Scan").on_press(Message::ClickedScan);
        let local_column = Column::new();

        local_column.push(scan_button).into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}

fn main() {
    println!("Easy Microarchitecture Level");
}
