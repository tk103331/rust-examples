use fltk::{prelude::*, *};
use fltk_theme::{WidgetScheme, SchemeType};

fn main() {
    let a = app::App::default();
    let widget_scheme = WidgetScheme::new(SchemeType::Clean);
    widget_scheme.apply();
    let mut win = window::Window::default().with_size(400, 300);
    let btn = button::Button::new(160, 200, 80, 30, "Hello");
    win.add(&btn);
    win.end();
    win.show();
    a.run().unwrap();
}