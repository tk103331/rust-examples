use druid::widget::{Button, Flex, Label};
use druid::{AppLauncher, LocalizedString, PlatformError, Widget, WidgetExt, WindowDesc};


fn ui_builder() -> impl Widget<u32> {
    //创建一个本地处理话的数据
    let text = LocalizedString::new("hello-counter")
        .with_arg("count", |data:&u32,_evt|{(*data).into()});
    let label = Label::new(text).padding(5.0).center();
    let button = Button::new("+ 1")
        .on_click(|_ctx,data:&mut u32,_ent|{
            *data += 1;
        });
    Flex::column().with_child(label).with_child(button)
}


fn main() -> Result<(), PlatformError>{
    let win = WindowDesc::new(ui_builder());
    let data = 0_u32;
    let _app = AppLauncher::with_window(win)
        .log_to_console()
        .launch(data);
    Ok(())
}
