use druid::{Point, Rect};
use druid::widget::prelude::*;
use druid::{
    theme,
    widget::{Click, ControllerHost, Flex, Label, LabelText},
    Affine, AppLauncher, Color, Data, Env, Insets, Lens, LinearGradient, RoundedRectRadii, Size,
    UnitPoint, WindowDesc,
};


struct MyButton<T>{
    //按钮上面的文字使用Label
    label:Label<T>,
    //尺寸
    label_size:Size
}

impl<T: Data> MyButton<T> {

    pub fn new(text: impl Into<LabelText<T>>) -> MyButton<T>{
        //根据传入文字参数，创建按钮
        MyButton::form_label(Label::new(text))
    }

    pub fn form_label(label:Label<T>) -> MyButton<T>{
        //根据传入label部件创建按钮
        MyButton 
        { 
            label,
            label_size: Size::ZERO,
        }
    }

    //按钮点击事件（也可以实现其余的事件，鼠标悬浮，鼠标移除等）
    pub fn on_click(
        self,
        f: impl Fn(&mut EventCtx,&mut T,&Env)+ 'static,
    ) -> ControllerHost<Self,Click<T>> {
        //controller事件处理，后面会详细讲解，Click是druid里面的部件可以直接使用，点击事件
        ControllerHost::new(self, Click::new(f))
    }

}

const LABEL_INSETS: Insets = Insets::new(30.0, 20.0, 30.0, 20.0);

impl<T:Data> Widget<T> for MyButton<T> {

    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
        match event {
            Event::MouseDown(_) => {
                if !ctx.is_disabled() {
                    ctx.set_active(true);
                    //请求重画
                    ctx.request_paint();
                }
            },
            Event::MouseUp(_) => {
                if ctx.is_active() && ctx.is_disabled() {
                    //请求重画
                    ctx.request_paint();
                    //trace!("Button {:?} released",ctx.widget_id());
                }
                ctx.set_active(false);
            },
            _ => {}
        }
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
        if let LifeCycle::HotChanged(_) | LifeCycle::DisabledChanged(_) = event {
            ctx.request_paint();
        }
        self.label.lifecycle(ctx, event, data, env)
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &T, data: &T, env: &Env) {
        //调用label的update方法
        self.label.update(ctx, old_data, data, env)
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &T, env: &Env) -> Size {
        //设置padding的大小
        let padding = Size::new(LABEL_INSETS.x_value(), LABEL_INSETS.y_value());
        //shrink 按照传入的padding设置label的边距，（给定的大小也是取整数，四舍五入）
        //loosen 复制一份相同大小的尺寸
        let label_bc = bc.shrink(padding).loosen();
        //调用label的layout方法
        self.label_size = self.label.layout(ctx, &label_bc, data, env);
        //获取部件的最小宽度
        let min_height = env.get(theme::BORDERED_WIDGET_HEIGHT);
        //返回偏移量
        let baseline = self.label.baseline_offset();
        //设置偏移量
        ctx.set_baseline_offset(baseline + LABEL_INSETS.y1);
        //按钮的大小
        let button_size = bc.constrain(Size::new(
            self.label_size.width + padding.width,
            (self.label_size.height + padding.height).max(min_height),
        ));
        println!("{:?}",button_size);
        button_size
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, env: &Env) {

        //active 表示是否鼠标按下
        //disabled 表示是否禁用
        let is_active = ctx.is_active() && !ctx.is_disabled();
        //hot 鼠标是否悬停在按钮上面
        let is_hot = ctx.is_hot();
        //size 布局大小
        let size = ctx.size();
        //获取按钮边框的宽度
        let stroke_width = env.get(theme::BUTTON_BORDER_WIDTH);

        let rounded_rect = size
            //转换成原点格式 Rect::new(0., 0., self.width, self.height)
            .to_rect()
            //计算尺寸
            .inset(-stroke_width / 2.0)
            //拐角的半径 env.get(theme::BUTTON_BORDER_RADIUS)
            .to_rounded_rect(RoundedRectRadii {
                top_left:size.width/2.0,
                top_right:size.width/2.0,
                bottom_right:size.width/2.0,
                bottom_left:size.width/2.0,
            });

        //
        let bg_gradient = if ctx.is_disabled() {
            //禁用的效果
            LinearGradient::new(
                UnitPoint::TOP,
                UnitPoint::BOTTOM,
                (
                    env.get(theme::DISABLED_BUTTON_LIGHT),
                    env.get(theme::DISABLED_BUTTON_DARK),
                ),
            )
            //鼠标按下的效果
        } else if is_active {
            LinearGradient::new(
                UnitPoint::TOP,
                UnitPoint::BOTTOM,
                (Color::PURPLE,Color::WHITE)
                //(env.get(theme::BUTTON_DARK), env.get(theme::BUTTON_LIGHT)),
            )
        } else {
            //从某个位置到某个位置，可以渐变
            LinearGradient::new(
                //从顶部到底部  由红色渐变到白色
                UnitPoint::TOP,
                UnitPoint::BOTTOM,
                (Color::BLUE,Color::WHITE)
                //(env.get(theme::BUTTON_LIGHT), env.get(theme::BUTTON_DARK)),
            )
        };

        //边框的颜色
        let border_color = if is_hot && !ctx.is_disabled() {
            //env.get(theme::BORDER_LIGHT)
            Color::RED
        } else {
            //env.get(theme::BORDER_DARK)
            Color::WHITE
        };

        //
        ctx.stroke(rounded_rect, &border_color, stroke_width);

        ctx.fill(rounded_rect, &bg_gradient);

        let label_offset = (size.to_vec2() - self.label_size.to_vec2()) / 2.0;

        ctx.with_save(|ctx| {
            ctx.transform(Affine::translate(label_offset));
            //调用label的paint
            self.label.paint(ctx, data, env);
        });
    }
}

#[derive(Clone,Data,Lens)]
struct State{
    //label显示的文字
    text:String,
}


fn builder_ui() -> impl Widget<State> {

    //MyButton创建，这里的Label使用了Data模型，MyButton添加了点击事件
    let my_button = MyButton::form_label(Label::new(|state: &State,_env: &Env|{ format!("{}",state.text)}))
        .on_click(|_etc,state,_env|{
            println!("点击了按钮");
            state.text = "你点击了".to_string();
        });

    //Flex布局，这个后面也会讲解一下
    Flex::row()
        //把我们的组件添加到布局
        .with_child(my_button)
}

fn main() {
    let win = WindowDesc::new(builder_ui())
        .title("自定义按钮")
        .window_size((300.0, 300.0));

    let _app = AppLauncher::with_window(win)
        .log_to_console()
        .launch(State {
            text: "按钮".to_string(),
        });
}
