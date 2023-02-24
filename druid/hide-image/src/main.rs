use std::env;

use druid::widget::{Flex, Label, Image, Either, Checkbox, Button};
use druid::{AppLauncher, PlatformError, Widget, WidgetExt, WindowDesc, ImageBuf, Data, Lens};


#[derive(Clone, Data, Lens)]
struct AppState {
    pub show_image: bool
}

fn ui_builder() -> impl Widget<AppState> {
    let label = Label::new("隐藏图片");
    let mut image_path = env::current_dir().unwrap();
    image_path.push("logo.png");
 
    let image_buf = ImageBuf::from_file(image_path).unwrap();
    let image = Image::new(image_buf).fix_size(200.0, 200.0);
    let either = Either::new(|data: &AppState,_env| {data.show_image}, image, label);
    let checkbox = Checkbox:: new("是否显示图片").lens(AppState::show_image);
    let button = Button::new("切换显示")
        .on_click(|_ctx, data: &mut AppState, _env|{
            data.show_image = !data.show_image;
        });
    Flex::column().with_child(checkbox).with_child(button).with_child(either)
}


fn main() -> Result<(), PlatformError>{
    let win = WindowDesc::new(ui_builder());
    let data = AppState{show_image: false};
    let _app = AppLauncher::with_window(win)
        .log_to_console()
        .launch(data);
    Ok(())
}
