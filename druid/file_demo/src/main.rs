
use std::sync::Arc;
use druid::WidgetExt;
use druid::{Data,Selector,AppDelegate,Env,FileInfo,FileDialogOptions,FileSpec, Lens,AppLauncher, Widget,WindowDesc, widget::{Flex,Label,Scroll,Button,List,CrossAxisAlignment}};

fn main() {
    let win = WindowDesc::new(builder_ui()).window_size((500.,500.));

    let data = AppState { folders: Arc::new(vec![]) };

    let _app = AppLauncher::with_window(win)
        .delegate(Delegate{})
        .launch(data);
}

#[derive(Clone,Data,Lens)]
struct AppState{
    //目录
    folders:Arc<Vec<FolderProperties>>
}


#[derive(Clone,Data,Lens)]
struct FolderProperties{
    //目录名称
    file_name:String
}

//打开文件
const OPEN_FILE_MI:Selector<FileInfo> = Selector::new("open_file_mi");
//打开文件夹
const OPEN_DIR_MI:Selector<FileInfo> = Selector::new("open_dir_mi");
//修改图片
const UPDATE_IMAGE:Selector<String> = Selector::new("update_image");

fn builder_ui() -> impl Widget<AppState> {

    //创建过滤类型
    let file_spec = vec![FileSpec::JPG,FileSpec::PNG];

    //创建文件对话框 选择图片
    let file_dialog_image = FileDialogOptions::new()
        .allowed_types(file_spec)
        //发送选择器
        .accept_command(OPEN_FILE_MI);

    let file_dialog_folder = FileDialogOptions::new()
        //设置选择文件夹而不是文件
        .select_directories()
        //发送选择器
        .accept_command(OPEN_DIR_MI);


    //创建两个按钮
    let open_image = Button::new("打开图片").on_click(move |ctx,_data,_env|{
        ctx.submit_command(druid::commands::SHOW_OPEN_PANEL.with(file_dialog_image.clone()))
    });
    let open_folder = Button::new("打开文件夹").on_click(move |ctx,_data,_env|{
        ctx.submit_command(druid::commands::SHOW_OPEN_PANEL.with(file_dialog_folder.clone()))
    });
    

    //创建布局
    Flex::row()
        //位置
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .with_child(
            Scroll::new(List::new(list).lens(AppState::folders)).vertical().fix_height(480.).fix_width(150.)
        )
        .with_child(
            Flex::row()
                .with_child(open_folder)
                .with_child(open_image)
        )
}


struct  Delegate{}

impl AppDelegate<AppState> for Delegate{

    fn command(
            &mut self,
            _ctx: &mut druid::DelegateCtx,
            _target: druid::Target,
            cmd: &druid::Command,
            data: &mut AppState,
            _env: &Env,
        ) -> druid::Handled {

            println!("{:?}",cmd);
            if let Some(file_info) = cmd.get(OPEN_FILE_MI) {
                _ctx.submit_command(UPDATE_IMAGE.with(file_info.path.as_os_str().to_str().unwrap().to_string()));
                return druid::Handled::Yes;
            }
            if let Some(file_info) = cmd.get(OPEN_DIR_MI) {
                let path = file_info.path.as_os_str().to_str().unwrap().to_string();
                let mut file = std::fs::read_dir(path).unwrap();
                let mut list = vec![];
                while let Some(next) = file.next(){ 
                    let n = next.unwrap();
                    let file_name = n.file_name().as_os_str().to_str().unwrap().to_string();
                    list.push(FolderProperties{file_name:file_name});
                }
                let list = Arc::new(list);
                data.folders = list;
                
                return druid::Handled::Yes;
            }

            druid::Handled::No
        
    }
}


fn list() -> impl Widget<FolderProperties> {
    Flex::row()
        .with_child(Label::new(|dir_list:&FolderProperties,_env:&Env|{format!("{}",dir_list.file_name)}).fix_width(400.))
}