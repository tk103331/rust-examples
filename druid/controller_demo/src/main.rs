use druid::widget::{Controller,Flex,TextBox};
use druid::{Env,Event,EventCtx,Widget, WidgetExt, Data, Lens, WindowDesc, AppLauncher};

struct TakeFocus;

impl <T, W: Widget<T>> Controller<T, W> for TakeFocus {
    //处理事件
    /*
    child:小部件
    EventCtx:事件etc
    Event：事件
    data：数据
    */
    fn event(&mut self, child: &mut W, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
        //当窗口第一次实例化的时候接收到WindowConnectioned
        if let Event::WindowConnected = event {
            //让小部件获得焦点
            ctx.request_focus();
        }
        child.event(ctx, event, data, env)
    }
}

#[derive(Data,Lens,Clone)]
struct AppState{
    text:String
}

fn builder_ui() -> impl Widget<AppState>{
    Flex::row()
        //给输入框添加controller控制器
        .with_child(TextBox::multiline().controller(TakeFocus).lens(AppState::text))
}


fn main() {
    let win = WindowDesc::new(builder_ui()).window_size((200.,200.));

    let _app = AppLauncher::with_window(win).launch(AppState{text:"".to_string()});
}