use crate::widget::QChildWidget;
use rune::{ContextError, Module, Value};
use rune::runtime::Protocol;
use rune::{ToValue,FromValue};
use rune::Any;
use super::rune_ext::ValueExt;
use super::widget::*;

#[macro_export]
macro_rules! set_widget {
    ( $origin:ident , $module:ident, $T:ty , $($ext_fn:ident)* ) => {
        $module.ty::< $origin<$T> > ()?;
        $(
        set_widget_inst_fn!($ext_fn, $origin, $module, $T);
        )*
    };
//columns rows split_point min_size bar_size min_bar_area draggable solid_bar
    // ( padding            , $origin:ident, $module:ident, $T:ty ) => { $module.inst_fn("padding",             $origin::<$T>::padding)?; };
    // ( center             , $origin:ident, $module:ident, $T:ty ) => { $module.inst_fn("center",              $origin::<$T>::center)?; };
    // ( align_left         , $origin:ident, $module:ident, $T:ty ) => { $module.inst_fn("align_left",          $origin::<$T>::align_left)?; };
    // ( align_right        , $origin:ident, $module:ident, $T:ty ) => { $module.inst_fn("align_right",         $origin::<$T>::align_right)?; };
    // ( align_vertical     , $origin:ident, $module:ident, $T:ty ) => { $module.inst_fn("align_vertical",      $origin::<$T>::align_vertical)?; };
    // ( align_horizontal   , $origin:ident, $module:ident, $T:ty ) => { $module.inst_fn("align_horizontal",    $origin::<$T>::align_horizontal)?; };
    // ( fix_width          , $origin:ident, $module:ident, $T:ty ) => { $module.inst_fn("fix_width",           $origin::<$T>::fix_width)?; };
    // ( fix_height         , $origin:ident, $module:ident, $T:ty ) => { $module.inst_fn("fix_height",          $origin::<$T>::fix_height)?; };
    // ( fix_size           , $origin:ident, $module:ident, $T:ty ) => { $module.inst_fn("fix_size",            $origin::<$T>::fix_size)?; };
    // ( expand             , $origin:ident, $module:ident, $T:ty ) => { $module.inst_fn("expand",              $origin::<$T>::expand)?; };
    // ( expand_width       , $origin:ident, $module:ident, $T:ty ) => { $module.inst_fn("expand_width",        $origin::<$T>::expand_width)?; };
    // ( expand_height      , $origin:ident, $module:ident, $T:ty ) => { $module.inst_fn("expand_height",       $origin::<$T>::expand_height)?; };
    // ( background         , $origin:ident, $module:ident, $T:ty ) => { $module.inst_fn("background",          $origin::<$T>::background)?; };
    // ( border             , $origin:ident, $module:ident, $T:ty ) => { $module.inst_fn("border",              $origin::<$T>::border)?; };
    // ( env_scope          , $origin:ident, $module:ident, $T:ty ) => { $module.inst_fn("env_scope",           $origin::<$T>::env_scope)?; };
    // ( on_click           , $origin:ident, $module:ident, $T:ty ) => { $module.inst_fn("on_click",            $origin::<$T>::on_click)?; };
    // ( scroll             , $origin:ident, $module:ident, $T:ty ) => { $module.inst_fn("scroll",              $origin::<$T>::scroll)?; };
}

macro_rules! set_widget_inst_fn {
    ( $fnc:ident, $origin:ident, $module:ident, $T:ty ) => {
        { $module.inst_fn( stringify!($fnc),             $origin::<$T>::$fnc)?; };
    }
}

// fn tes_info<V:Into<Vec<u8>>>(v:V) {
//     println!("{:?}", v.into());
// }

fn tes_info(v:impl Into<Vec<u8>> ) {
    println!("{:?}", v.into());
}

pub fn module<T:'static + rune::compile::Named + druid::Data>() -> Result<Module,ContextError> {
    let mut module = Module::default();

    module.function( &["tuplecheck"], |v:Value| {
        // assert!( v.type_info().unwrap() != std::any::TypeId::of::< (f64,f64) >() )
        // also
        // assert!( std::any::TypeId::of::<rune::runtime::Tuple>()) != std::any::TypeId::of::< (f64,f64) >() )
        // this code raise the runtime error (already taken value)
        //let a = <(f64,f64)>::from_value(v.clone()).unwrap();

        //copy from reference
        let a = v.resolve_tuple2::<f64>().unwrap();
        println!("{:?}",a);
    });
    

    module.constant( &["TEXT_COLOR"],"org.linebender.druid.theme.label_color");
    module.constant( &["DISABLED_TEXT_COLOR"], "org.linebender.druid.theme.disabled_label_color");
    module.constant( &["PLACEHOLDER_COLOR"], "org.linebender.druid.theme.placeholder_color");
    module.constant( &["PRIMARY_LIGHT"], "org.linebender.druid.theme.primary_light");
    module.constant( &["PRIMARY_DARK"], "org.linebender.druid.theme.primary_dark");
    module.constant( &["PROGRESS_BAR_RADIUS"], "org.linebender.druid.theme.progress_bar_radius");
    module.constant( &["BACKGROUND_LIGHT"], "org.linebender.druid.theme.background_light");
    module.constant( &["BACKGROUND_DARK"], "org.linebender.druid.theme.background_dark");
    module.constant( &["FOREGROUND_LIGHT"], "org.linebender.druid.theme.foreground_light");
    module.constant( &["FOREGROUND_DARK"], "org.linebender.druid.theme.foreground_dark");
    module.constant( &["DISABLED_FOREGROUND_LIGHT"], "org.linebender.druid.theme.disabled_foreground_light");
    module.constant( &["DISABLED_FOREGROUND_DARK"], "org.linebender.druid.theme.disabled_foreground_dark");
    module.constant( &["BUTTON_DARK"], "org.linebender.druid.theme.button_dark");
    module.constant( &["BUTTON_LIGHT"], "org.linebender.druid.theme.button_light");
    module.constant( &["DISABLED_BUTTON_DARK"], "org.linebender.druid.theme.disabled_button_dark");
    module.constant( &["DISABLED_BUTTON_LIGHT"], "org.linebender.druid.theme.disabled_button_light");
    module.constant( &["BUTTON_BORDER_RADIUS"], "org.linebender.druid.theme.button_radius");
    module.constant( &["BUTTON_BORDER_WIDTH"], "org.linebender.druid.theme.button_border_width");
    module.constant( &["BORDER_DARK"], "org.linebender.druid.theme.border_dark");
    module.constant( &["BORDER_LIGHT"], "org.linebender.druid.theme.border_light");
    module.constant( &["SELECTED_TEXT_BACKGROUND_COLOR"], "org.linebender.druid.theme.selection_color");
    module.constant( &["SELECTED_TEXT_INACTIVE_BACKGROUND_COLOR"], "org.linebender.druid.theme.selection_color_inactive");
    module.constant( &["SELECTION_TEXT_COLOR"], "org.linebender.druid.theme.selection_text_color");
    module.constant( &["CURSOR_COLOR"], "org.linebender.druid.theme.cursor_color");
    module.constant( &["TEXT_SIZE_NORMAL"], "org.linebender.druid.theme.text_size_normal");
    module.constant( &["TEXT_SIZE_LARGE"], "org.linebender.druid.theme.text_size_large");
    module.constant( &["BASIC_WIDGET_HEIGHT"], "org.linebender.druid.theme.basic_widget_height");
    module.constant( &["WIDE_WIDGET_WIDTH"], "org.linebender.druid.theme.long-widget-width");
    module.constant( &["BORDERED_WIDGET_HEIGHT"], "org.linebender.druid.theme.bordered_widget_height");
    module.constant( &["TEXTBOX_BORDER_RADIUS"], "org.linebender.druid.theme.textbox_border_radius");
    module.constant( &["TEXTBOX_BORDER_WIDTH"], "org.linebender.druid.theme.textbox_border_width");
    module.constant( &["TEXTBOX_INSETS"], "org.linebender.druid.theme.textbox_insets");
    module.constant( &["WIDGET_PADDING_HORIZONTAL"], "org.linebender.druid.theme.widget-padding-h");
    module.constant( &["WIDGET_PADDING_VERTICAL"], "org.linebender.druid.theme.widget-padding-v");
    module.constant( &["WIDGET_CONTROL_COMPONENT_PADDING"], "org.linebender.druid.theme.widget-padding-control-label");
    module.constant( &["SCROLLBAR_COLOR"], "org.linebender.druid.theme.scrollbar_color");
    module.constant( &["SCROLLBAR_BORDER_COLOR"], "org.linebender.druid.theme.scrollbar_border_color");
    module.constant( &["SCROLLBAR_MAX_OPACITY"], "org.linebender.druid.theme.scrollbar_max_opacity");
    module.constant( &["SCROLLBAR_FADE_DELAY"], "org.linebender.druid.theme.scrollbar_fade_time");
    module.constant( &["SCROLLBAR_WIDTH"], "org.linebender.druid.theme.scrollbar_width");
    module.constant( &["SCROLLBAR_PAD"], "org.linebender.druid.theme.scrollbar_pad");
    module.constant( &["SCROLLBAR_RADIUS"], "org.linebender.druid.theme.scrollbar_radius");
    module.constant( &["SCROLLBAR_EDGE_WIDTH"], "org.linebender.druid.theme.scrollbar_edge_width");
    module.constant( &["SCROLLBAR_MIN_SIZE"], "org.linebender.theme.scrollbar_min_size");
    
    
    
    
    module.constant(&["UnitPoint", "TOP_LEFT"], (0.0, 0.0) );
    module.constant(&["UnitPoint", "TOP"], (0.5, 0.0) );
    module.constant(&["UnitPoint", "TOP_RIGHT"], (1.0, 0.0) );
    module.constant(&["UnitPoint", "LEFT"], (0.0, 0.5) );
    module.constant(&["UnitPoint", "CENTER"], (0.5, 0.5) );
    module.constant(&["UnitPoint", "RIGHT"], (1.0, 0.5) );
    module.constant(&["UnitPoint", "BOTTOM_LEFT"], (0.0, 1.0) );
    module.constant(&["UnitPoint", "BOTTOM"], (0.5, 1.0) );
    module.constant(&["UnitPoint", "BOTTOM_RIGHT"], (1.0, 1.0) );


    // module.constant( &["flex","Start"],  CrossAxisAlignment::Start() )?;
    // module.constant( &["flex","End"],  CrossAxisAlignment::End() )?;
    // module.constant( &["flex","Center"],  CrossAxisAlignment::Center() )?;
    // module.constant( &["flex","Baseline"],  CrossAxisAlignment::Baseline() )?;
    module.function( &["flex"], Flex::<T>::new )?;
    module.function( &["flex","row"], Flex::<T>::row)?;
    module.function( &["flex","column"], Flex::<T>::column)?;

    module.function( &["label"], Label::<T>::new)?;
    module.function( &["button"], Button::<T>::new)?;
    module.function( &["checkbox"], Checkbox::<T>::new)?;
    module.function( &["either"], Either::<T>::new)?;
    module.function( &["progressbar"], ProgressBar::<T>::new)?;
    module.function( &["painter"], Painter::<T>::new)?;
    module.function( &["list"], List::<T>::new)?;

    module.function( &["radiogroup"], RadioGroup::<T>::new)?;
    //module.function( &["radiogroup","row"], RadioGroup::<T>::row)?;
    //module.function( &["radiogroup","column"], RadioGroup::<T>::new)?;
    module.function( &["slider"], Slider::<T>::new)?;
    module.function( &["stepper"], Stepper::<T>::new)?;
    module.function( &["textbox"], TextBox::<T>::new)?;
    module.function( &["switch"], Switch::<T>::new)?;
    module.function( &["spinner"], Spinner::<T>::new)?;
    module.function( &["image"], Image::<T>::new)?;
    module.function( &["svg"], Svg::<T>::new)?;

    module.function( &["call_rfn"], |f:Value| {
        let f = rune::runtime::Function::from_value(f).unwrap();
        let result:u32 = f.call( (2,3) ).unwrap();
        println!("{:?}", result);
    });


    super::types::install_types::<T>(&mut module)?;
    super::render::install_render::<T>(&mut module)?;

    //Env
    module.ty::<Env>();
    module.inst_fn( "set", Env::set)?;

    //Flex
    module.ty::<FlexParams>();
    module.function( &["FlexParams","new"], FlexParams::new)?;
    module.ty::<CrossAxisAlignment>()?;
    module.ty::<MainAxisAlignment>()?;

    //Text
    module.ty::<LineBreaking>()?;
    module.ty::<TextAlignment>()?;

    //Image
    module.ty::<FillStrat>()?;

    set_widget!(Padding, module, T,          padding center align_left align_right align_vertical align_horizontal fix_width fix_height fix_size expand expand_width expand_height background border env_scope on_click scroll);
    set_widget!(Align, module, T,            padding center align_left align_right align_vertical align_horizontal fix_width fix_height fix_size expand expand_width expand_height background border env_scope on_click scroll);
    set_widget!(SizedBox, module, T,         padding center align_left align_right align_vertical align_horizontal fix_width fix_height fix_size background border env_scope on_click scroll);
    module.inst_fn("expand", SizedBox::<T>::_expand )?;
    module.inst_fn("expand_width", SizedBox::<T>::_expand_width )?;
    module.inst_fn("expand_height", SizedBox::<T>::_expand_height )?;

    set_widget!(Container, module, T,        padding center align_left align_right align_vertical align_horizontal fix_width fix_height fix_size expand expand_width expand_height env_scope on_click scroll);
    module.inst_fn("background", Container::<T>::_background )?;
    module.inst_fn("border", Container::<T>::_border )?;

    set_widget!(Flex, module, T,             padding center align_left align_right align_vertical align_horizontal fix_width fix_height fix_size expand expand_width expand_height background border env_scope on_click scroll cross_axis_alignment main_axis_alignment must_fill_main_axis with_child with_flex_child with_default_spacer with_spacer with_flex_spacer set_cross_axis_alignment set_main_axis_alignment set_must_fill_main_axis add_child add_flex_child);

    //TODO : with_font set_font draw_at
    set_widget!(Label, module, T,           padding center align_left align_right align_vertical align_horizontal fix_width fix_height fix_size expand expand_width expand_height background border env_scope on_click scroll with_text_color with_text_size with_line_break_mode with_text_alignment set_text_color set_text_size set_line_break_mode set_text_alignment baseline_offset);
    set_widget!(Button, module, T,          padding center align_left align_right align_vertical align_horizontal fix_width fix_height fix_size expand expand_width expand_height background border env_scope on_click scroll);
    set_widget!(Checkbox, module, T,        padding center align_left align_right align_vertical align_horizontal fix_width fix_height fix_size expand expand_width expand_height background border env_scope on_click scroll);
    set_widget!(Either, module, T,          padding center align_left align_right align_vertical align_horizontal fix_width fix_height fix_size expand expand_width expand_height background border env_scope on_click scroll);
    set_widget!(ProgressBar, module, T,     padding center align_left align_right align_vertical align_horizontal fix_width fix_height fix_size expand expand_width expand_height background border env_scope on_click scroll);
    set_widget!(Painter, module, T,         padding center align_left align_right align_vertical align_horizontal fix_width fix_height fix_size expand expand_width expand_height background border env_scope on_click scroll);
    set_widget!(RadioGroup, module, T,      padding center align_left align_right align_vertical align_horizontal fix_width fix_height fix_size expand expand_width expand_height background border env_scope on_click scroll);
    set_widget!(Slider, module, T,          padding center align_left align_right align_vertical align_horizontal fix_width fix_height fix_size expand expand_width expand_height background border env_scope on_click scroll);
    set_widget!(Stepper, module, T,         padding center align_left align_right align_vertical align_horizontal fix_width fix_height fix_size expand expand_width expand_height background border env_scope on_click scroll);
    set_widget!(TextBox, module, T,         padding center align_left align_right align_vertical align_horizontal fix_width fix_height fix_size expand expand_width expand_height background border env_scope on_click scroll);
    set_widget!(Switch, module, T,          padding center align_left align_right align_vertical align_horizontal fix_width fix_height fix_size expand expand_width expand_height background border env_scope on_click scroll);
    set_widget!(Spinner, module, T,         padding center align_left align_right align_vertical align_horizontal fix_width fix_height fix_size expand expand_width expand_height background border env_scope on_click scroll);
    set_widget!(Image, module, T,           padding center align_left align_right align_vertical align_horizontal fix_width fix_height fix_size expand expand_width expand_height border env_scope on_click scroll fill_mode set_fill_mode interpolation_mode set_interpolation_mode clip_area set_clip_area);
    set_widget!(Svg, module, T,             padding center align_left align_right align_vertical align_horizontal fix_width fix_height fix_size expand expand_width expand_height border env_scope on_click scroll fill_mode set_fill_mode);
    set_widget!(EnvScope, module, T,        padding center align_left align_right align_vertical align_horizontal fix_width fix_height fix_size expand expand_width expand_height background border on_click scroll);
    set_widget!(ControllerHost, module, T,  padding center align_left align_right align_vertical align_horizontal fix_width fix_height fix_size expand expand_width expand_height background border env_scope on_click scroll);
    set_widget!(Split, module, T,           padding center align_left align_right align_vertical align_horizontal fix_width fix_height fix_size expand expand_width expand_height background border env_scope on_click scroll split_point min_size bar_size min_bar_area draggable solid_bar);// columns rows );


    module.ty::<QChildWidget<T>>()?;
    module.field_fn(Protocol::GET, "value", QChildWidget::<T>::get_value)?;
    module.field_fn(Protocol::SET, "value", QChildWidget::<T>::set_value)?;

    Ok(module)
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {

        #[derive(rune::Any)]
        struct Data;
        let module = super::module::<Data>().unwrap();
    }
}