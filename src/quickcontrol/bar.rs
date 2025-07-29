//! Top bar for the quickcontrol interface

use iced::Subscription;
use iced::widget::text;
use std::time::{Duration, Instant};

struct Bar {}

#[derive(Clone)]
enum Message {
    UpdateClock(Instant),
    UpdateBattery(Instant),
}

impl Bar {
    fn subscription(&self) -> Subscription<Message> {
        Subscription::batch([
            iced::time::every(Duration::from_secs(1)).map(Message::UpdateClock),
            iced::time::every(Duration::from_secs(60)).map(Message::UpdateBattery),
        ])
    }

    fn view(&self) -> iced::Element<Message> {
        self.clock();
        self.battery();
    }

    fn clock(&self) {
        text()
    }

    fn battery(&self) {}
}
