use druid::{
    AppLauncher,Widget, widget::{Flex, RadioGroup}, Data, Lens, WindowDesc, WidgetExt};


#[derive(Data,Lens,Clone, Copy)]
struct AppState{
    image_type:ImageType
}

//单选 选项
#[derive(Clone, Copy, PartialEq, Data)]
enum ImageType {
    PNG,
    JPEG,
    ICON,
}

fn build_main() -> impl Widget<AppState> {

    //创建一组单选按钮
    let radio_group = RadioGroup::row(vec![
        ("png",ImageType::PNG),
        ("jpeg",ImageType::JPEG),
        ("icon",ImageType::ICON)
        ]).lens(AppState::image_type);

    Flex::row()
        .with_child(radio_group)
}

fn main() {
    //创建窗体
    let win = WindowDesc::new(build_main()).title("单选demo");

    let _ = AppLauncher::with_window(win).launch(AppState{image_type:ImageType::PNG});
}