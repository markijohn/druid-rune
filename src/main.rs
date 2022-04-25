#![windows_subsystem = "windows"]

use std::any::{Any,TypeId};
use druid::im::Vector;
use druid::{Data, Application, Color, commands, Event, EventCtx, Handled, KeyEvent, lens, LifeCycle, LifeCycleCtx, TimerToken, UnitPoint, UpdateCtx, WidgetExt};

use druid::{AppLauncher, LocalizedString, Widget, WindowDesc, AppDelegate, DelegateCtx, Target, Command, Env};

use std::fs::{File,Metadata};
use std::path::PathBuf;

use rune::{Module, Sources};

use rune::termcolor::{ColorChoice, StandardStream};
use rune::{Diagnostics, FromValue, Vm, Source};
use std::sync::Arc;
use std::sync::mpsc::{Receiver, Sender};
use std::time::Duration;
use druid::text::EditableText;
use druid_widget_nursery::{DropdownSelect};

mod druid_rune;
mod widget;

const DEFAULT_SOURCE:&str = r#"
//return builded widget
pub fn main() {
    flex()
    .with( label("HELLO") )
}
"#;

// We can't send any widget between threads.
// Use unsafe tricky
static mut BUILDED_WIDGET:Option< Box<dyn Widget<AppState>> > = None;

#[derive(Clone,rune::Any, druid::Data, druid::Lens)]
pub struct AppState {
    live : bool,
    ex_source : String,
    source : String,
    trigger : bool
}

impl AppState {
    fn new() -> Self {

        let live = if cfg!(debug_assertions) {
            false
        } else {
            true
        };
        Self {
            live,
            ex_source : String::new(),
            source : DEFAULT_SOURCE.to_string(),
            trigger : false
        }
    }
}

fn create_body( src_sx:Sender<String>, builded_tx:Receiver<()> ) -> impl Widget<AppState> {
    use druid::widget::*;


    //code edit control and compile
    pub struct SourceController;

    impl Controller<String, TextBox<String>> for SourceController {
        //simple rune code editor control
        fn event(&mut self, child: &mut TextBox<String>, ctx: &mut EventCtx, event: &Event, data: &mut String, env: &Env) {
            if let Event::KeyDown(ke) = event {
                if druid::HotKey::new(druid::SysMods::Cmd, druid::KbKey::Enter).matches(ke) {
                    ctx.set_handled();
                } else if ke.code == druid::Code::Tab {
                    child.text_mut().borrow_mut().insert_text(data,"\t");
                    ctx.set_handled(); //stop propagation
                } else if ke.code == druid::Code::Enter {
                    let curr_pos = child.text().borrow().selection().min();
                    let prev_lf = data.preceding_line_break( curr_pos );
                    let auto_lf_pos = (&data.as_str()[ prev_lf .. curr_pos ]).chars().enumerate().find( |(i,v)| !v.is_whitespace() ).unwrap_or( (0,'0') ).0;
                    let mut next_lf_prefix = String::from('\n');
                    next_lf_prefix.push_str(&data.as_str()[prev_lf .. prev_lf+auto_lf_pos]);
                    child.text_mut().borrow_mut().insert_text(data,&next_lf_prefix);
                    ctx.set_handled(); //stop propagation
                }
            }
            child.event(ctx, event, data, env);
        }

        // fn lifecycle(&mut self,child: &mut TextBox<String>,ctx: &mut LifeCycleCtx, event:&LifeCycle,data: &String,env: &Env) {
        //     if let LifeCycle::WidgetAdded = event {
        //         //init call
        //         self.0.send( data.clone() );
        //     }
        //     child.lifecycle(ctx, event, data, env)
        // }
        //
        // //send source to rune compiler
        // fn update(&mut self, child: &mut TextBox<String>, ctx: &mut UpdateCtx, data: &String, data2: &String, env: &Env) {
        //     use std::collections::hash_map::DefaultHasher;
        //     use std::hash::{Hash, Hasher};
        //     let mut s = DefaultHasher::new();
        //     data.hash( &mut s);
        //     let s1 = s.finish();
        //
        //     let mut s = DefaultHasher::new();
        //     data2.hash( &mut s);
        //     let s2 = s.finish();
        //
        //     if s1 != s2 {
        //         self.0.send( data2.clone() );
        //     }
        //
        //     child.update(ctx, data, data2, env)
        // }
    }

    let mut editor = TextBox::multiline();
    editor.handles_tab_notifications = false;


    let main =
    Container::new(
        Split::columns(
            editor
                .controller( SourceController )
                .padding( 2.)
                .border( Color::FUCHSIA, 0.2)
                .lens(AppState::source)
            ,
            ViewSwitcher::new(
                |data: &AppState, _env| data.trigger,
                move |trigger, _data, _env| {
                    //println!("Some builded..");
                    let widget:Box<dyn Widget<AppState>> = if let Ok( _) = builded_tx.try_recv() {
                        //TODO : sync bug
                        unsafe { BUILDED_WIDGET.take().unwrap_or( Box::new( Label::<AppState>::new("TODO") ) ) }
                    } else {
                        Box::new( Label::<AppState>::new("Building...") )
                    };
                    widget

                },
            ).padding(2.0)
        ).solid_bar(true)
        .split_point(0.4)
        .draggable(true)
        .bar_size(2.0)
        .env_scope(|env,_| {
            env.set(druid::theme::BORDER_LIGHT, Color::rgb8(0x1d, 0x1d, 0x1d));
        })
    );

    let mut rn_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    rn_dir.push("rn");
    let mut ex_list:Vector< (String,String) > = if let Ok(r) = std::fs::read_dir( rn_dir ) {
        r.filter( |re| {
            re.as_ref()
                .map( |r|
                    r.path().is_file()
                        && r.path().extension().map(|v| v.to_str().unwrap_or("") == "rn")
                        .unwrap_or(false) )
                .unwrap_or(false)
        }).map( |v| {
            let p = v.unwrap().path();
            (p.file_name().unwrap().to_str().unwrap().to_string(), std::fs::read_to_string(p).unwrap())
        }).collect()
    } else {
        Vector::new()
    };
    ex_list.insert(0, ("(select example)".to_string(), "".to_string()));

    struct ExDropdownSelected;
    impl<W: Widget<String>> Controller<String, W> for ExDropdownSelected {
        fn update(&mut self, child: &mut W, ctx: &mut UpdateCtx, old_data: &String, new_data: &String, env: &Env) {
            if old_data != new_data {
                let ndc = new_data.clone();
                ctx.get_external_handle().add_idle_callback( |data:&mut AppState| {
                    data.source = ndc;
                });
            }
            child.update(ctx, old_data, new_data, env);
        }
    }
    let head = Flex::row( )
        .main_axis_alignment(MainAxisAlignment::Start)
        .must_fill_main_axis(true)
        .with_child( Checkbox::new("Live").lens(AppState::live).padding(5.) )
        .with_default_spacer()
        .with_child(
            Button::new("Run(Ctrl+Enter)")
                .on_click( |ctx,state:&mut AppState,env|  state.trigger = !state.trigger )
                .padding(5.)
                .disabled_if( |state:&AppState,env| { state.live }) )
        .with_default_spacer()
        .with_child(
            DropdownSelect::new(ex_list)
                .align_left()
                .controller(ExDropdownSelected)
                .lens(AppState::ex_source)
        )
        .with_flex_spacer( 1.0 )
        .padding(10.)
        .background(Color::rgb8(0x5d,0x5d,0x5d) );

    struct MainController( Sender<String> );
    impl <W:Widget<AppState>> Controller<AppState, W> for MainController {
        fn event(&mut self, child: &mut W, ctx: &mut EventCtx, event: &Event, data: &mut AppState, env: &Env) {
            if let Event::KeyDown(key) = event {
                if druid::HotKey::new(druid::SysMods::Cmd, druid::KbKey::Enter).matches(key) {
                    self.0.send( data.source.clone() );
                }
            }
            child.event(ctx, event, data, env);
        }

        fn lifecycle(&mut self,child: &mut W,ctx: &mut LifeCycleCtx, event:&LifeCycle,data: &AppState,env: &Env) {
            if let LifeCycle::WidgetAdded = event {
                //init call
                self.0.send( data.source.clone() );
            }
            child.lifecycle(ctx, event, data, env)
        }

        //send source to rune compiler
        fn update(&mut self, child: &mut W, ctx: &mut UpdateCtx, data: &AppState, data2: &AppState, env: &Env) {
            //data2.live has default value (always true)
            if data.live {
                //use std::collections::hash_map::DefaultHasher;
                //use std::hash::{Hash, Hasher};
                // let mut s = DefaultHasher::new();
                // data.source.hash( &mut s);
                // let s1 = s.finish();
                //
                // let mut s = DefaultHasher::new();
                // data2.source.hash( &mut s);
                // let s2 = s.finish();
                //
                // if s1 != s2 {
                //     self.0.send( data2.source.clone() );
                // }
                if !data.source.same(&data2.source) {
                    self.0.send( data2.source.clone() );
                }
            }

            child.update(ctx, data, data2, env)
        }
    }
    Flex::column()
        .must_fill_main_axis(true)
        .with_child( head )
        .with_flex_child( main ,1.0 )
        .controller(MainController(src_sx))
}

fn widget_builder(src_tx:Receiver<String>, widget_sx:Sender<()>, event_sink: druid::ExtEventSink) {
    use druid_rune::druid::widget::*;
    std::thread::spawn( move || {
        let module = druid_rune::druid::module::<AppState>().unwrap();
        let mut context = rune_modules::default_context().unwrap();
        context.install(
            &module
        );

        std::panic::set_hook(Box::new(|_info| {
            eprintln!("{}", _info);
        }));

        loop {
            let mut builder_source = String::new();
            //take last source
            for n in src_tx.try_iter() {
                builder_source = n;
            }
            if builder_source.is_empty() {
                std::thread::sleep(Duration::from_millis(10));
                continue;
            }

            let mut sources = Sources::new();
            let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            path.push("examples");
            path.push("purerune.rn");

            sources.insert(Source::new("main", builder_source));

            let mut diagnostics = Diagnostics::new();

            //prepare is heavy weight
            let result = rune::prepare(&mut sources)
                .with_context(&context)
                .with_diagnostics(&mut diagnostics)
                .build();

            if !diagnostics.is_empty() {
                let mut writer = StandardStream::stderr(ColorChoice::Always);
                diagnostics.emit(&mut writer, &sources).unwrap();
            }

            let unit = match result {
                Ok(unit) => unit,
                Err(e) => {
                    println!("{:?}", e);
                    continue
                }
            };

            let mut vm = Vm::new(Arc::new(context.runtime()), Arc::new(unit));

            match vm.call(&["main"], ()) {
                Ok(v) => {
                    widget_type_resolve!(v, AppState, |resolved| {
                        unsafe { BUILDED_WIDGET = Some( Box::new( resolved.origin ) ) };
                        widget_sx.send( () );
                        event_sink.add_idle_callback(move |data: &mut AppState| {
                            //raise event to the to ViewSwitch (
                            data.trigger = !data.trigger;
                        });
                    });
                },
                Err(e) => {
                    eprintln!("{:?}", e);
                }
            }
        }

    } );
}

fn main() {
    let (src_sx, src_tx) = std::sync::mpsc::channel::<String>();
    let (widget_sx, widget_tx) = std::sync::mpsc::channel::<()>();

    let window = WindowDesc::new(create_body( src_sx, widget_tx ) )
        .window_size( (1200. , 800.) )
        .title(LocalizedString::new("split-demo-window-title").with_placeholder("Quick druid prototype"));

    let launcher = AppLauncher::with_window(window);
    let event_sink = launcher.get_external_handle();

    widget_builder(src_tx, widget_sx, event_sink);


    launcher.launch( AppState::new() )
    .expect("launch failed");
}
