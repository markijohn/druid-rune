//! Linked wrapper widget for rune
use std::fmt::{Debug, Formatter};
use std::rc::Rc;
use druid::{BoxConstraints, Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx, PaintCtx, Size, UpdateCtx, Widget};
use druid::widget::Flex;
use rune::Vm;
use simplecss::StyleSheet;
use visdom::html::ParseOptions;
use visdom::Vis;
use rune::Any;

#[derive(Debug)]
pub enum QTag {
    BODY,
    DIV,
    BUTTON,
    LABEL,
    INPUT,
    CANVAS
}

#[derive(Any)]
pub struct QWidget<T:Any> {
    tag : QTag,
    id : String,
    classes : Vec<Rc<String>>,
    origin : Box<dyn Widget<T>>,
    vm : Option<Vm>,
}

impl <T:Any> Debug for QWidget<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "QWidget")
    }
}

impl <T:Any> QWidget<T> {
    pub fn new(css:&str, ui:&str, rune_scripts:Option<&str>) {
        let mut css = StyleSheet::parse(css);

        let elem = Vis::load_options_catch(ui, ParseOptions {
            case_sensitive_tagname: false,
            allow_self_closing: true,
            auto_fix_unclosed_tag: true,
            auto_fix_unexpected_endtag: true,
            auto_fix_unescaped_lt: true
        }, Box::new(|e| {
            todo!()
        }) );

        let style_elem = elem.find("style").into_iter().fold( String::new(), |mut sv, s| {
            sv.push_str( &s.html() );
            sv
        });
        css.parse_more( &style_elem );

        if let Some(script) = rune_scripts {

        }
    }

    pub fn call(func:&str) {

    }

    pub fn get_text(&self) -> Option<&str> {
        todo!()
    }

    pub fn query(str:&str) -> Option<String> {
        todo!()
    }
}

impl <T:Any> Widget<T> for QWidget<T> {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
        self.origin.event(ctx,event,data,env);
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
        self.origin.lifecycle(ctx, event, data, env);
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &T, data: &T, env: &Env) {
        self.origin.update(ctx, old_data, data, env);
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &T, env: &Env) -> Size {
        self.origin.layout(ctx, bc, data, env)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, env: &Env) {
        self.origin.paint(ctx, data, env);
    }
}