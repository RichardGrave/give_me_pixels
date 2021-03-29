use iced::window;
use iced::{executor, Application, Command, Element, Settings, Text};

// pub fn main() -> iced::Result {
//     _init_app_window()
// }

// TODO:RG remove '_' when we want to use the app window
pub fn _init_app_window() -> iced::Result {
    GiveMePixels::run(Settings {
        window: window::Settings {
            // TODO:RG get and set max window size
            size: (1440, 900),
            ..window::Settings::default()
        },
        ..Settings::default()
    })
}

struct GiveMePixels;

impl Application for GiveMePixels {
    type Executor = executor::Default;
    type Message = ();
    type Flags = ();

    fn new(_flags: ()) -> (GiveMePixels, Command<Self::Message>) {
        (GiveMePixels, Command::none())
    }

    fn title(&self) -> String {
        String::from("Give me Pixels")
    }

    fn update(&mut self, _message: Self::Message) -> Command<Self::Message> {
        Command::none()
    }

    fn view(&mut self) -> Element<Self::Message> {
        Text::new("And here we are").into()
    }
}
