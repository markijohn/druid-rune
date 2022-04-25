#![windows_subsystem = "windows"]

use druid::{Application, commands, Event, EventCtx, Handled, TimerToken, UnitPoint, WidgetExt};

use druid::{AppLauncher, LocalizedString, Widget, WindowDesc, AppDelegate, DelegateCtx, Target, Command, Env};

use std::fs::{File,Metadata};
use std::path::PathBuf;
use druid::widget::*;
use rune::{Module, Sources};

use rune::termcolor::{ColorChoice, StandardStream};
use rune::{Diagnostics, FromValue, Vm, Source};
use std::sync::Arc;

#[derive(Clone,rune::Any, druid::Data, druid::Lens)]
pub struct AppState {
    name: String,
    lastmodified: u128
}

fn lastmodified() -> u128 {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("examples");
    path.push("purerune.rn");
    let file = std::fs::File::open(&path).unwrap();
    file.metadata().unwrap().modified().unwrap().duration_since( std::time::UNIX_EPOCH ).unwrap().as_millis()
}

impl AppState {
    fn new() -> Self {
        Self {
            name : "Hello".to_string(),
            lastmodified : lastmodified()
        }
    }

    fn just() -> Self {
        Self {
            name : "Hello".to_string(),
            lastmodified : 0
        }
    }
}

fn create_body() -> impl Widget<AppState> {
    let module = druid_qwidget::rune_module::druid::module::<AppState>().unwrap();
    let view_switcher = ViewSwitcher::new(
        |data: &AppState, _env| data.lastmodified,
        move |selector, _data, _env|  {
            println!("Start vm");
            let mut context = rune_modules::default_context().unwrap();
            context.install(
                &module
            );


            let mut sources = Sources::new();
            let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            path.push("examples");
            path.push("purerune.rn");

            let file = std::fs::File::open(&path).unwrap();
            let lastmodified = file.metadata().unwrap().modified().unwrap().duration_since( std::time::UNIX_EPOCH ).unwrap().as_millis();
            sources.insert( Source::new("main", std::fs::read_to_string(&path).unwrap() ) );

            let mut diagnostics = Diagnostics::new();

            //prepare is heavy weight
            let result = rune::prepare(&mut sources)
                .with_context(&context)
                .with_diagnostics(&mut diagnostics)
                .build();
            println!("Prepared rune script");

            if !diagnostics.is_empty() {
                let mut writer = StandardStream::stderr(ColorChoice::Always);
                diagnostics.emit(&mut writer, &sources).unwrap();
            }

            let unit = match result {
                Ok(unit) => unit,
                Err(e) => return Box::new(druid::widget::Label::new(e.to_string()).center())
            };

            let mut vm = Vm::new(Arc::new(context.runtime()), Arc::new(unit));

            match vm.call(&["main"], () ) {
                Ok(v) => {
                    let flex = druid_qwidget::rune_module::druid::Flex::<AppState>::from_value(v).unwrap();
                    let c = Container::new( flex.origin );
                    println!("Done");
                    Box::new( c )
                },
                Err(e) => {
                    println!("Done");
                    Box::new(druid::widget::Label::new(format!("Unknown : {}",e.to_string()) ).center())
                }
            }



        },
    );

    static TIMER_INTERVAL: std::time::Duration = std::time::Duration::from_millis(400);
    struct RefreshController {
        lastmodified : u128,
        timer_id : TimerToken
    }
    impl<W: Widget<AppState>> Controller<AppState, W> for RefreshController {
        fn event(&mut self, child: &mut W, ctx: &mut EventCtx, event: &Event, data: &mut AppState, env: &Env) {
            match event {
                Event::WindowConnected => {
                    // Start the timer when the application launches
                    self.timer_id = ctx.request_timer(TIMER_INTERVAL);
                }
                Event::Timer(id) => {
                    if *id == self.timer_id {
                        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
                        path.push("examples");
                        path.push("purerune.rn");
                        let lastmodified = std::fs::File::open(&path).unwrap()
                            .metadata().unwrap().modified().unwrap().duration_since( std::time::UNIX_EPOCH ).unwrap().as_millis();
                        if self.lastmodified != lastmodified {
                            self.lastmodified = lastmodified;
                            data.lastmodified = lastmodified;
                            println!("Changed rn");
                        }
                        self.timer_id = ctx.request_timer(TIMER_INTERVAL);
                    }
                }
                _ => child.event(ctx, event, data, env),
            }
        }
    }

    view_switcher.controller( RefreshController{lastmodified : lastmodified(), timer_id:TimerToken::INVALID} )
    // view_switcher
}



fn main() {

    let window = WindowDesc::new(create_body() )
        .window_size( (1000. , 800.) )
        .title(LocalizedString::new("split-demo-window-title").with_placeholder("Pike"));

    AppLauncher::with_window(window)
        .launch( AppState::just() )
        .expect("launch failed");
}
