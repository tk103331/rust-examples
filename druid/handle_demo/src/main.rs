
use std::thread;

use druid::{Selector, Widget, Data, Lens,ExtEventSink, WindowDesc, AppLauncher,AppDelegate, Target, Env, WidgetExt};
use druid::widget::{Flex,Label,Button, Spinner, Either};
use std::time::Duration;

//创建选择器
const CHANGE_Text: Selector<u32> = Selector::new("change_text");

#[derive(Clone,Data,Lens,Default)]
struct AppState{
    num: u32,
    processing:bool,
}

fn builder_ui() -> impl Widget<AppState> {
    
    let label = Label::new(|state: &AppState,_env: &Env|{ format!("{}",state.num)});
    
    let button = Button::new("改变文字")
        .on_click(|ctx,data:&mut AppState,env|{
            println!("点击");
            data.processing = false;
            
            wrapped_slow_function(ctx.get_external_handle(),data.num);
            println!("结束");
        });

    let spinner = Spinner::new();

    let either = Either::new(|data:&AppState,_env: &Env|{data.processing}, button, spinner);


    Flex::column()
        .with_child(label)
        .with_child(either)
}

fn wrapped_slow_function(sink: ExtEventSink,num: u32){
    thread::spawn(move||{

        thread::sleep(Duration::from_millis(1000));
        sink.submit_command(CHANGE_Text, num, Target::Auto)
            .expect("command to submit");
    });
}

struct Delegate;

impl AppDelegate<AppState> for Delegate {
    
    fn command(
            &mut self,
            _ctx: &mut druid::DelegateCtx,
            _target: druid::Target,
            cmd: &druid::Command,
            data: &mut AppState,
            _env: &druid::Env,
        ) -> druid::Handled {
            println!("接收命令");
            if let Some(text) = cmd.get(CHANGE_Text) {
                println!("{:?}",data.num);
                data.num = 10;
                println!("进入了");
                data.processing = true;
                druid::Handled::Yes
            }else{
                druid::Handled::No
            }   
    }
}


fn main() {
    let win = WindowDesc::new(builder_ui());

    let _app = AppLauncher::with_window(win)
        .delegate(Delegate{})
        .launch(AppState{num:1,processing:true});
}