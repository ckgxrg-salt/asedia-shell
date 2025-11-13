//! Application definition

use iced::widget::button;
use iced::{Color, Event, Subscription, Task, Theme};
use iced_layershell::{Appearance, to_layer_message};

pub struct DaywatchShell {}

#[to_layer_message]
#[derive(Debug, Clone)]
pub enum Message {
    TextInput(String),
    IcedEvent(Event),
}

impl DaywatchShell {
    pub fn new(_flags: ()) -> (Self, Task<Message>) {
        (Self {}, Task::none())
    }

    pub fn namespace(&self) -> String {
        String::from("daywatch-shell")
    }

    pub fn style(&self, theme: &Theme) -> iced_layershell::Appearance {
        Appearance {
            background_color: Color::TRANSPARENT,
            text_color: theme.palette().text,
        }
    }

    pub fn subscription(&self) -> Subscription<Message> {
        iced::event::listen().map(Message::IcedEvent)
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::IcedEvent(_event) => {
                println!("event");
                Task::none()
            }
            _ => unreachable!(),
        }
    }

    fn view(&self) -> iced::Element<Message> {
        button("Nope").into()
    }
}
