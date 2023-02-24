
use std::sync::Arc;

use druid::{widget::{Flex,Label,CrossAxisAlignment,MainAxisAlignment,Button, RadioGroup}, Widget, WindowDesc, Env, AppLauncher, Lens, Data, AppDelegate, Selector, Command, Target};

#[derive(Clone,Lens,Data)]
struct AppState{
    label_text:String,
    main_axis_alignment: MainAxisAlignment,
    cross_axis_alignment: CrossAxisAlignment,
}
 
const CHANGE_MainAxisAlignment: Selector<MainAxisAlignment> = Selector::new("main_axis_alignment");
const CHANGE_CrossAxisAlignment: Selector<CrossAxisAlignment> = Selector::new("cross_axis_alignment");

struct  Delegate{}
impl AppDelegate<AppState> for Delegate {
    
    fn command(
            &mut self,
            _ctx: &mut druid::DelegateCtx,
            _target: druid::Target,
            cmd: &druid::Command,
            data: &mut AppState,
            _env: &druid::Env,
        ) -> druid::Handled {
            if let Some(main_axis_alignment) = cmd.get(CHANGE_MainAxisAlignment) {
                data.main_axis_alignment = *main_axis_alignment;
                druid::Handled::Yes
            } else if let Some(cross_axis_alignment) = cmd.get(CHANGE_CrossAxisAlignment) {
                data.cross_axis_alignment = *cross_axis_alignment;
                druid::Handled::Yes
            } else{
                druid::Handled::No
            }   
        }
}

struct MainWidget {
    app_state: AppState
}

impl Widget<AppState> for MainWidget {
    fn event(&mut self, ctx: &mut druid::EventCtx, event: &druid::Event, data: &mut AppState, env: &Env) {
        
    }
    fn lifecycle(&mut self, ctx: &mut druid::LifeCycleCtx, event: &druid::LifeCycle, data: &AppState, env: &Env) {
        
    }
    fn update(&mut self, ctx: &mut druid::UpdateCtx, old_data: &AppState, data: &AppState, env: &Env) {
        
    }
    fn layout(&mut self, ctx: &mut druid::LayoutCtx, bc: &druid::BoxConstraints, data: &AppState, env: &Env) -> druid::Size {
        bc.max()
    }
    fn paint(&mut self, ctx: &mut druid::PaintCtx, data: &AppState, env: &Env) {
        let mut ui =build_ui(self.app_state.main_axis_alignment, self.app_state.cross_axis_alignment);
        ui.paint(ctx, data, env);
    }
}

fn build_flex(main_axis_alignment: MainAxisAlignment,cross_axis_alignment: CrossAxisAlignment) -> Flex<AppState>{

    let button = Button::new("点击");
    let button1 = Button::new("点击1");
    let button2 = Button::new("点击2");
    let button3 = Button::new("点击3");
    let button4 = Button::new("点击4");
    Flex::column()
        .main_axis_alignment(main_axis_alignment)
        .cross_axis_alignment(cross_axis_alignment)
        .with_child(button)
        .with_child(button1)
        .with_child(button2)
        .with_child(button3)
        .with_child(button4)
}

fn build_ui(main_axis_alignment: MainAxisAlignment,cross_axis_alignment: CrossAxisAlignment) -> impl Widget<AppState> {
    let flex = build_flex(main_axis_alignment, cross_axis_alignment);

    let btns_main = Flex::row()
    .with_child(new_main_axis_btn(MainAxisAlignment::Start))
    .with_child(new_main_axis_btn(MainAxisAlignment::Center))
    .with_child(new_main_axis_btn(MainAxisAlignment::End))
    .with_child(new_main_axis_btn(MainAxisAlignment::SpaceAround))
    .with_child(new_main_axis_btn(MainAxisAlignment::SpaceBetween))
    .with_child(new_main_axis_btn(MainAxisAlignment::SpaceEvenly));

    let btns_cross = Flex::row()
    .with_child(new_cross_axis_btn(CrossAxisAlignment::Start))
    .with_child(new_cross_axis_btn(CrossAxisAlignment::Center))
    .with_child(new_cross_axis_btn(CrossAxisAlignment::End))
    .with_child(new_cross_axis_btn(CrossAxisAlignment::Fill))
    .with_child(new_cross_axis_btn(CrossAxisAlignment::Baseline));

    Flex::column()
    .with_child(flex)
    .with_child(btns_main)
    .with_child(btns_cross)
}

fn new_main_axis_btn(align: MainAxisAlignment) -> impl Widget<AppState> {
    Button::new(format!("Main{:?}", align)).on_click(move |ctx,_data,_env|{
        ctx.submit_command(Command::new(CHANGE_MainAxisAlignment, align, Target::Auto));
    })
}

fn new_cross_axis_btn(align: CrossAxisAlignment) -> impl Widget<AppState> {
    Button::new(format!("Cross{:?}", align)).on_click(move |ctx,_data,_env|{
        ctx.submit_command(Command::new(CHANGE_CrossAxisAlignment, align, Target::Auto));
    })
}


fn main() {
    let app_state = AppState{
        label_text:"1".to_string(),
        main_axis_alignment: MainAxisAlignment::Start,
        cross_axis_alignment: CrossAxisAlignment::Start,
    };
    let mut main = MainWidget{app_state};
    let win = WindowDesc::new(main).window_size((400.,400.));

    let _app = AppLauncher::with_window(win).delegate(Delegate{})
        .launch(main.app_state.clone());
}