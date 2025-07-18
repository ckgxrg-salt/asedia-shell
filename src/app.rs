//! Application definition

use iced::widget::button;
use iced::{Color, Event, Subscription, Task, Theme};
use iced_layershell::{Appearance, Application, to_layer_message};

pub struct AsediaShell {}

#[to_layer_message]
#[derive(Debug, Clone)]
pub enum Message {
    TextInput(String),
    IcedEvent(Event),
}

impl Application for AsediaShell {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = ();
    type Theme = Theme;

    fn new(_flags: Self::Flags) -> (Self, Task<Message>) {
        (Self {}, Task::none())
    }

    fn namespace(&self) -> String {
        String::from("asedia-shell")
    }

    fn style(&self, theme: &Self::Theme) -> iced_layershell::Appearance {
        Appearance {
            background_color: Color::TRANSPARENT,
            text_color: theme.palette().text,
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        iced::event::listen().map(Message::IcedEvent)
    }

    fn update(&mut self, message: Message) -> Task<Message> {
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
