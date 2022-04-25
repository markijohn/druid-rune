//! Linked wrapper widget for rune
use std::borrow::Cow;
use std::fmt::{Debug, Display, Formatter};
use std::rc::Rc;
use druid::{BoxConstraints, Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx, PaintCtx, Size, UpdateCtx, Widget};
use druid::widget::{Container, Flex};
use rune::Vm;
use simplecss::{Rule, StyleSheet};
use visdom::html::ParseOptions;
use visdom::Vis;
use rune::Any;
use rune::compile::Named;
use visdom::types::Elements;

pub enum Error {
    InvalidUISource(Box<dyn std::error::Error>),
    RequiredSingleBody
}
pub type Result<T> = std::result::Result<T,Error>;

#[derive(Debug)]
pub enum QWidgetTag {
    body, //container
    div, //container or flow
    button, //button
    label, //label
    input, //text
    canvas //Paint
}

impl Display for QWidgetTag {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            QWidgetTag::body => write!(f,"body"),
            QWidgetTag::div => write!(f,"div"),
            QWidgetTag::button => write!(f,"button"),
            QWidgetTag::label => write!(f,"label"),
            QWidgetTag::input => write!(f,"input"),
            QWidgetTag::canvas => write!(f,"canvas"),
        }
    }
}

pub struct QWidget<T:Any> {
    origin : Rc<QChildWidget<T>>,
    vm : Option<Box<Vm>>
}

impl <T:Any> Debug for QWidget<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", "\t".repeat(self.origin.depth), self.origin.tag)
    }
}

impl <T:Any> QWidget<T> {
    pub fn new(css:&str, ui:&str, rune_script:Option<&str>) -> Result< Self >{
        let mut global_css = StyleSheet::parse(css);

        let ui_root = Vis::load_options(ui, ParseOptions {
            case_sensitive_tagname: false,
            allow_self_closing: true,
            auto_fix_unclosed_tag: true,
            auto_fix_unexpected_endtag: true,
            auto_fix_unescaped_lt: true
        }).map_err( |e| {
            Error::InvalidUISource(e)
        })?;

        let style_elem = ui_root.find("style").into_iter().fold( String::new(), |mut sv, s| {
            sv.push_str( &s.html() );
            sv
        });
        global_css.parse_more( &style_elem );

        let body = ui_root.find("body");
        if body.length() != 1 ||  body.is_empty() {
            return Err(Error::RequiredSingleBody)
        }
        ///Container::new()

        for s in ui_root.find("script[type=\"text/runescript\"]") {

        }

        if let Some(script) = rune_script {

        }

        todo!()
    }
}

#[derive(Any)]
pub struct QChildWidget<T:'static + Named> {
    tag : QWidgetTag,
    id : String,
    classes : Vec<Rc<String>>,
    depth : usize,
    origin : Box<dyn Widget<T>>,
    parent : Option<Rc<QChildWidget<T>>>,
    side : Option< (Rc<QChildWidget<T>>, Rc<QChildWidget<T>>) >
}

impl <T:'static + Named> Debug for QChildWidget<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "QWidget")
    }
}

pub struct Queried<T:'static + Named> {
   queried : Vec< Rc<QChildWidget<T>> >
}

impl <T:'static + Named> Queried<T> {
    fn len(&self) -> usize {
        self.queried.len()
    }

    fn get_value(&self) -> Option< Vec<Option<&str>> > {
        if self.queried.len() == 0 {
            None
        } else {
            Some( self.queried.iter().map( |v| v.get_value() ).collect() )
        }
    }

    fn set_value(&self, value: &str) {
        if self.queried.len() == 0 {
            //None
        } else {
            self.queried.iter().for_each( |v| v.set_value( value ) )
        }
    }

    fn has_class(&self, class: &str) -> bool {
        todo!()
    }

    fn set_class(&mut self, class: &str) -> bool {
        todo!()
    }

    fn unset_class(&mut self, class: &str) -> bool {
        todo!()
    }
}

impl <T:'static + Named> QChildWidget<T> {
    pub fn create_child_widget(parent:Option<Rc<QChildWidget<T>>>, css_rules:&Vec<Rule>, elem:Elements ) -> QChildWidget<T> {

        todo!()
    }

    pub fn get_value(&self) -> Option<&str> {
        todo!()
    }

    pub fn set_value(&self, val:&str) {
        todo!()
    }

    pub fn query(str:&str) -> Option<String> {
        todo!()
    }
}

impl <T:'static + Named> Widget<T> for QChildWidget<T> {
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