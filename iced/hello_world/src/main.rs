use iced::widget::{column, text};
use iced::{Alignment, Element, Sandbox, Settings};

pub fn main() -> iced::Result {
    Hello::run(Settings::default())
}

struct Hello {

} 

impl Sandbox for Hello {
    type Message = ();

    fn new() -> Self {
        Self{}
    }

    fn title(&self) -> String {
        String::from("HelloWorld - Iced")
    }

    fn update(&mut self, _: ()) {}

    fn view(&self) -> Element<()> {
        column![
            text("HelloWorld").size(50)
        ]        
        .padding(20)
        .align_items(Alignment::Center)
        .into()
    }
}