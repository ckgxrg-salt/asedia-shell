use iced::widget::{button, column, row, text};
use iced::{Element, Event, Subscription, Task};
use iced_layershell::to_layer_message;
use std::fmt::Display;

pub struct LogoutWindow {
    text: String,
    focused: LogoutAction,
}

#[derive(PartialEq, Clone, Debug)]
pub enum LogoutAction {
    None,
    Poweroff,
    Reboot,
    Suspend,
    Logout,
    Lock,
}

impl Display for LogoutAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::None => write!(f, "Asedia"),
            Self::Poweroff => write!(f, "Power off"),
            Self::Reboot => write!(f, "Reboot"),
            Self::Suspend => write!(f, "Suspend"),
            Self::Logout => write!(f, "Log out"),
            Self::Lock => write!(f, "Lock Screen"),
        }
    }
}

#[to_layer_message]
#[derive(Clone, Debug)]
pub enum Message {
    SelectAction(LogoutAction),
    IcedEvent(Event),
}

impl Default for LogoutWindow {
    fn default() -> Self {
        Self::new()
    }
}

impl LogoutWindow {
    #[must_use]
    pub fn new() -> Self {
        Self {
            text: LogoutAction::None.to_string(),
            focused: LogoutAction::None,
        }
    }

    #[must_use]
    pub fn namespace(&self) -> String {
        String::from("asedia-logout")
    }

    pub fn subscription(&self) -> Subscription<Message> {
        iced::event::listen().map(Message::IcedEvent)
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::SelectAction(action) => {
                if action != LogoutAction::None && self.focused == action {
                    execute(&action);
                } else {
                    self.text = action.to_string();
                    self.focused = action;
                }
                Task::none()
            }
            Message::IcedEvent(event) => {
                println!("{event:?}");
                Task::none()
            }
            _ => unreachable!(),
        }
    }

    #[must_use]
    pub fn view(&self) -> Element<'_, Message> {
        column![
            row![
                button("poweroff").on_press(Message::SelectAction(LogoutAction::Poweroff)),
                button("reboot").on_press(Message::SelectAction(LogoutAction::Reboot)),
            ],
            text(&self.text),
            row![
                button("suspend").on_press(Message::SelectAction(LogoutAction::Suspend)),
                button("logout").on_press(Message::SelectAction(LogoutAction::Logout)),
                button("lock").on_press(Message::SelectAction(LogoutAction::Lock)),
            ]
        ]
        .into()
    }
}

// Executes the given logout action.
fn execute(action: &LogoutAction) {
    println!("{action}");
    panic!("Exited");
}
