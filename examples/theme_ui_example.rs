// Example demonstrating the theme settings UI
// This shows how to create a complete theme customization interface

use cosmic::{
    iced::Application,
    iced::Command,
    iced::Element,
    iced::Settings,
    iced::Theme,
    iced::window,
    widget::text,
    widget::column,
    widget::container,
    widget::button,
    iced_core::Length,
    Apply,
};
use vortex_fm::views::theme_settings::{ThemeSettingsPage, ThemeMessage};

#[derive(Debug, Clone)]
enum Message {
    Theme(ThemeMessage),
    Close,
}

struct ThemeUIApp {
    theme_page: ThemeSettingsPage,
}

impl Application for ThemeUIApp {
    type Executor = cosmic::executor::Default;
    type Flags = ();
    type Message = Message;
    type Theme = Theme;

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            Self {
                theme_page: ThemeSettingsPage::new(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        "Vortex FM - Theme Settings".to_string()
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Theme(theme_message) => {
                self.theme_page.update(theme_message);
            }
            Message::Close => {
                return window::close(window::Id::MAIN);
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        column![
            text("Vortex FM Theme Settings")
                .size(24)
                .apply(container)
                .padding(20),
            
            ThemeSettingsPage::section()
                .map(Message::Theme),
            
            button("Close")
                .on_press(Message::Close)
                .apply(container)
                .padding(20)
        ]
        .spacing(20)
        .width(Length::Fill)
        .height(Length::Fill)
        .apply(container)
        .padding(20)
        .into()
    }

    fn theme(&self) -> Theme {
        Theme::default()
    }
}

fn main() -> cosmic::iced::Result {
    ThemeUIApp::run(Settings {
        window: window::Settings {
            size: cosmic::iced::Size::new(800.0, 600.0),
            ..Default::default()
        },
        ..Default::default()
    })
}
