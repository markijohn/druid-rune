use std::collections::HashMap;
use std::ops::DerefMut;
use druid::Widget;
use once_cell::sync::Lazy;
use simplecss::{AttributeOperator, PseudoClass, StyleSheet};
use rune::Any;


static mut SMAP:Lazy< HashMap<String,(String,StyleSheet<'_>)> > = Lazy::new( || HashMap::new() );

// {Padding<$TT> Align<$TT> SizedBox<$TT> Container<$TT> Flex<$TT> Label<$TT> Button<$TT> Checkbox<$TT>
// Either<$TT> ProgressBar<$TT> List<$TT> RadioGroup<$TT> Slider<$TT> Stepper<$TT> TextBox<$TT>
// Switch<$TT> Spinner<$TT> Image<$TT> Svg<$TT>

#[derive(Clone)]
pub(crate) enum WidgetTag {
    Container,
    Flex,
    Label,
    Button,
    Checkbox,
    Either,
    ProgressBar,
    List,
    RadioGroup,
    Slider,
    Stepper,
    TextBox,
    Switch,
    Spinner,
    Image,
    Svg
}

impl WidgetTag {
    pub fn name(&self) -> &str {
        match self {
            WidgetTag::Container => "container",
            WidgetTag::Flex => "flex",
            WidgetTag::Label => "label",
            WidgetTag::Button => "button",
            WidgetTag::Checkbox => "checkbox",
            WidgetTag::Either => "either",
            WidgetTag::ProgressBar => "progressbar",
            WidgetTag::List => "list",
            WidgetTag::RadioGroup => "radioGroup",
            WidgetTag::Slider => "slider",
            WidgetTag::Stepper => "stepper",
            WidgetTag::TextBox => "textBox",
            WidgetTag::Switch => "switch",
            WidgetTag::Spinner => "spinner",
            WidgetTag::Image => "image",
            WidgetTag::Svg => "svg",
        }
    }
}



#[derive(Clone)]
pub(crate) struct Elem {
    tag : WidgetTag,
    id : String,
    class : Vec<String>,
    parent : Option< Box<Elem> >
}

impl simplecss::Element for Elem {
    fn parent_element(&self) -> Option<Self> {
        self.parent.as_ref().map( |v| *v.clone() )
    }

    fn prev_sibling_element(&self) -> Option<Self> {
        None
    }

    fn has_local_name(&self, name: &str) -> bool {
        name == self.tag.name()
    }

    fn attribute_matches(&self, local_name: &str, operator: AttributeOperator) -> bool {
        if local_name == "id" && operator.matches( &self.id ) {
            true
        } else {
            //panic!("attribute select not support")
            false
        }
    }

    fn pseudo_class_matches(&self, class: PseudoClass) -> bool {
        //not support yet
        false
    }
}

#[derive(Any)]
pub(crate) struct Mapper {
    key : String,
    css : String,
    unsafe_ptr : *mut (),
}

impl Drop for Mapper {
    fn drop(&mut self) {
        if !self.unsafe_ptr.is_null() {
            unsafe { Box::from_raw(  self.unsafe_ptr as *mut simplecss::StyleSheet ) };
            self.unsafe_ptr = std::ptr::null_mut();
        }
    }
}

impl Mapper {
    pub fn new(key:String) -> Self {
        Self { key, css:String::new(), unsafe_ptr : std::ptr::null_mut() }
    }

    fn de_ss(&self) -> Option<&simplecss::StyleSheet> {
        if self.unsafe_ptr.is_null() {
            None
        } else {
            Some( unsafe { &*(self.unsafe_ptr as *const simplecss::StyleSheet) } )
        }
    }

    pub fn css(&mut self, css:String) {
        //rune script not support lifetime specific struct.
        //just save global

        //.......
        let parsed = StyleSheet::parse( &css[..] );
        if !self.unsafe_ptr.is_null() {
            unsafe { Box::from_raw(  self.unsafe_ptr as *mut simplecss::StyleSheet ) };
            self.unsafe_ptr = std::ptr::null_mut()
        }
        let u_ptr = Box::into_raw( Box::new( parsed ) );
        self.unsafe_ptr = u_ptr as _;
    }

    pub fn filter<F:FnMut( (&simplecss::Rule) )->bool>(&self, elem:&Elem, mut f:F) {
        if let Some( (_,ss) ) = unsafe { SMAP.get(&self.key) } {
            ss.rules.iter().filter( |v| {
                v.selector.matches( elem )
            } ).for_each( |v| {
                f( v );
            } );
        }
    }
}