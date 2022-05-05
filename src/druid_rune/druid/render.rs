use std::any::{TypeId};
use std::marker::PhantomData;
use std::ops::Deref;
use super::types::{Affine, Point, Circle, Color, InterpolationMode, ImageFormat, Line, Rect, RoundedRect, Size, Region, StrokeStyle};

use druid::piet;
use druid::piet::{RenderContext};
use rune::runtime::Function;
use rune::{Any,Value,FromValue};
use crate::{register_impl, shape_type_resolve, shape_type_resolve_impl, shape_type_ref_resolve, shape_type_ref_resolve_impl, type_resolve_impl, widget_type_resolve};

// macro_rules! type_resolve {
//     ($_self:ident, $v:ident, Shape, |$__self:ident, $resolved:ident| $f:block ) => {
//         type_resolve_impl!($_self, $v, {Circle,Line,Rect,RoundedRect}, |$__self,$resolved| $f )
//     };
// }
//
// macro_rules! type_resolve_impl {
//     ($_self:ident, $v:ident, {$($typs:ty),*}, |$__self:ident, $resolved:ident| $f:block ) => {
//         let v_hash = $v.type_hash().unwrap();
//         $( if v_hash == rune::Hash::from_type_id( TypeId::of::< $typs >() ) {
//             let cb = |$__self:&mut PaintCtx, $resolved:$typs | $f;
//             cb($_self, <$typs>::from_value($v).unwrap());
//         } ) else *
//         else {
//             panic!("Unknown type : {}", $v.type_info().unwrap().to_string())
//         }
//     };
// }
//
// macro_rules! register_impl {
//     ( $module:ident, $owner:ident, [$($consts:ident),*], [$($fields:ident),*], [$($const_fncs:ident),*], [$($inst_fncs:ident),*] ) => {
//         $(
//             //TODO : Any object constant not support
//             $module.constant( &[ stringify!($owner), stringify!($consts) ], $owner::$consts)?;
//         )*
//         $(
//             $module.field_fn( Protocol::GET, stringify!($fields), $owner::$fields)?;
//             //TODO : SETTER?
//         )*
//
//         $(
//             $module.function( &[ stringify!($owner), stringify!($const_fncs) ], $owner::$const_fncs)?;
//         )*
//         $module.ty::<$owner>();
//         $(
//             $module.inst_fn( stringify!($inst_fncs), $owner::$inst_fncs)?;
//         )*
//     }
// }

fn tes<'a>(a:&'a Vec<String>) {

}

pub fn install_render<T:'static + rune::compile::Named + druid::Data>(module:&mut rune::Module) -> Result<(), rune::ContextError> {
    module.ty::<Text>()?;
    module.ty::<Image>()?;
    module.ty::<TextLayout>()?;
    module.ty::<Brush>()?;
    module.ty::<Error>()?;
    module.ty::<PaintCtx>()?;

    //module.function(&["tes"], tes)?;

    register_impl!(module, PaintCtx, [],[]
    ,[]
    ,[size,is_hot,is_active,is_focused,has_focus,depth,region,with_child_ctx,with_save,paint_with_z_index
            ,status,clear,solid_brush,fill,fill_even_odd,stroke,stroke_styled,clip,draw_text,save,restore
        ,finish,transform,current_transform,make_image,draw_image,draw_image_area,capture_image_area,blurred_rect]);

    Ok(())
}

type Render<'a> = druid::piet::Piet<'a>; //D2DRenderContext

#[derive(Any,Debug)]
pub struct Text(piet::PietText);
impl Into<piet::PietText> for Text {
    fn into(self) -> piet::PietText {
        self.0
    }
}

#[derive(Any)]
pub struct Image(piet::PietImage);
impl Into<piet::PietImage> for Image {
    fn into(self) -> piet::PietImage {
        self.0
    }
}

impl AsRef<piet::PietImage> for Image {
    fn as_ref(&self) -> &piet::PietImage {
        &self.0
    }
}

#[derive(Any,Debug)]
pub struct TextLayout(piet::PietTextLayout);
impl Into<piet::PietTextLayout> for TextLayout {
    fn into(self) -> piet::PietTextLayout {
        self.0
    }
}

impl AsRef<piet::PietTextLayout> for TextLayout {
    fn as_ref(&self) -> &piet::PietTextLayout {
        &self.0
    }
}

#[derive(Any)]
pub struct Brush(piet::Brush); //D2DBrush

impl Into<piet::Brush> for Brush {
    fn into(self) -> piet::Brush {
        self.0
    }
}

#[derive(Any,Debug)]
pub struct Error(piet::Error);

impl From<piet::Error> for Error {
    fn from(e: piet::Error) -> Self {
        Self(e)
    }
}

//Context and RenderContext(deref)
#[derive(rune::Any,Debug)]
pub struct PaintCtx {
    unsafe_ptr : *mut (),
    region : Region
}

impl PaintCtx {
    // type Brush = Brush;
    //
    // type Text = D2DText;
    //
    // type TextLayout = D2DTextLayout;
    //
    //type Image = piet::Image;

    //////////////////////////////////////////////////////////////////
    //remove lifetime lexical wrapper
    pub fn new(ctx:&mut druid::PaintCtx) -> Self{
        Self {
            unsafe_ptr : ctx as *mut _ as _,
            region : Region( ctx.region().clone() )
        }
    }

    fn de(&self) -> &mut druid::PaintCtx {
        unsafe { std::mem::transmute::<_, &mut druid::PaintCtx>( self.unsafe_ptr ) }
    }

    //////////////////////////////////////////////////////////////////
    // PaintCtx wrapper
    // pub fn widget_id(&self) -> WidgetId {
    //     self.widget_state.id
    // }
    //
    // pub fn window(&self) -> &WindowHandle {
    //     &self.state.window
    // }
    //
    // pub fn window_id(&self) -> WindowId {
    //     self.state.window_id
    // }
    //
    // pub fn text(&mut self) -> &mut PietText {
    //     &mut self.state.text
    // }

    pub fn size(&self) -> Size {
        Size( self.de().size() )
    }

    pub fn is_hot(&self) -> bool {
        self.de().is_hot()
    }

    pub fn is_active(&self) -> bool {
        self.de().is_active()
    }

    pub fn is_focused(&self) -> bool {
        self.de().is_focused()
    }

    pub fn has_focus(&self) -> bool {
        self.de().has_focus()
    }

    // pub fn set_cursor(&mut self, cursor: &Cursor) {
    //     self.widget_state.cursor_change = CursorChange::Set(cursor.clone());
    // }

    // pub fn override_cursor(&mut self, cursor: &Cursor) {
    //     self.widget_state.cursor_change = CursorChange::Override(cursor.clone());
    // }

    // pub fn clear_cursor(&mut self) {
    //     self.de().clear_cursor();
    // }

    // pub fn submit_command(&mut self, cmd: impl Into<Command>) {
    //     self.state.submit_command(cmd.into())
    // }

    // pub fn get_external_handle(&self) -> ExtEventSink {
    //     self.state.ext_handle.clone()
    // }

    // pub fn request_timer(&mut self, deadline: Duration) -> TimerToken {
    //     self.state.request_timer(&mut self.widget_state, deadline)
    // }

    #[inline]
    pub fn depth(&self) -> u32 {
        self.de().depth()
    }

    #[inline]
    //pub fn region(&self) -> &Region {
    pub fn region(&self) -> Region {
        self.region.clone()
    }

    //pub fn with_child_ctx(&mut self, region: impl Into<Region>, rf: Function) {
    pub fn with_child_ctx(&mut self, region: Region, rf: Function) {
        let f = | mut_ctx:&mut druid::PaintCtx | {
            let wrapped_ctx = PaintCtx::new( mut_ctx );
            let _:() = rf.call( (wrapped_ctx,) ).unwrap();
        };
        self.de().with_child_ctx( Into::<druid::Region>::into(region), f );
    }

    pub fn with_save(&mut self, rf: Function) {
        let f = | mut_ctx:&mut druid::PaintCtx | {
            let wrapped_ctx = PaintCtx::new( mut_ctx );
            let _:() = rf.call( (wrapped_ctx,) ).unwrap();
        };
        self.de().with_save( f );
    }

    /// Allows to specify order for paint operations.
    ///
    /// Larger `z_index` indicate that an operation will be executed later.
    pub fn paint_with_z_index(
        &mut self,
        z_index: u32,
        rf: Function,
    ) {
        // let f = | mut_ctx:&mut druid::PaintCtx | {
        //     let wrapped_ctx = PaintCtx::new( mut_ctx  );
        //     let _:() = rf.call( (wrapped_ctx,) ).unwrap();
        // };
        // self.de().paint_with_z_index(z_index, f);
        unimplemented!()
    }

    //////////////////////////////////////////////////////////////////
    // RenderContext wrapper (D2DRenderContext)
    fn status(&mut self) -> Result<(), Error> {
        self.de().status().map_err(|e| Error(e))
    }

    //fn clear(&mut self, region: impl Into<Option<Rect>>, color: Color) {
    fn clear(&mut self, region: Option<Rect>, color: Value) {
        self.de().clear(region.map(|v| v.into()), Color::from(color).to_native() );
    }

    fn solid_brush(&mut self, color: Value) -> Brush {
        Brush( self.de().solid_brush(Color::from(color).to_native() ) )
    }

    // fn gradient(&mut self, gradient: impl Into<FixedGradient>) -> Result<Brush, Error> {
    //     self.unsafe_deref().gradient( gradient )
    // }

    // Shape impl is :
    // [X] PathSeg(Line,QuadBez,CubicBez)
    // [X] Arc
    // [X] BezPath
    // [O] Circle
    // [X] CircleSegment
    // [X] CubicBez
    // [X] Ellipse
    // [O] Line
    // [X] QuadBez
    // [O] RoundedRect
    // [O] Rect
    //
    // IntoBrush impl is :
    // [O] Color,
    // [X] FixedGradient(FixedLinearGradient,FixedRadialGradient),
    // [X] PaintBrush(Color,LinearGradient,RadialGradient,FixedGradient),
    // [X] LinearGradient,
    // [X] RadialGradient
    //
    // TODO : full set (curretly only Line,Rect,RoundedRect,Circle and Color are supported)



    //fn fill(&mut self, shape: impl Shape, brush: &impl IntoBrush<Self>) {
    //fn fill(&mut self, shape: rune::runtime::Shared<Value>, brush: &Color) {
    fn fill(&mut self, shape: Value, brush: Value) {
        shape_type_ref_resolve!(self, shape, |_self, resolved| {
            _self.de().fill(resolved.to_native(), &Color::from(brush).to_native() );
        });
    }

    //fn fill_even_odd(&mut self, shape: impl Shape, brush: &impl IntoBrush<Self>) {
    fn fill_even_odd(&mut self, shape: Value, brush: Value) {
        shape_type_ref_resolve!(self, shape, |_self, resolved| {
            _self.de().fill_even_odd(resolved.to_native(), &Color::from(brush).to_native() );
        });
    }

    //fn stroke(&mut self, shape: impl Shape, brush: &impl IntoBrush<Self>, width: f64) {
    fn stroke(&mut self, shape: Value, brush: Value, width: f64) {
        shape_type_ref_resolve!(self, shape, |_self, resolved| {
            _self.de().stroke(resolved.to_native(), &Color::from(brush).to_native(), width);
        });
    }

    //fn stroke_styled(&mut self, shape: impl Shape, brush: &impl IntoBrush<Self>, width: f64, style: &StrokeStyle, ) {
    fn stroke_styled(&mut self, shape: Value, brush: Value, width: f64, style: &StrokeStyle, ) {
        shape_type_ref_resolve!(self, shape, |_self, resolved| {
            _self.de().stroke_styled(resolved.to_native(), &Color::from(brush).to_native(), width, style.as_ref());
        });
    }

    //fn clip(&mut self, shape: impl Shape) {
    fn clip(&mut self, shape: Value) {
        shape_type_ref_resolve!(self, shape, |_self, resolved| {
            _self.de().clip(resolved.to_native());
        });
    }

    // fn text(&mut self) -> &mut Text {
    //     self.de().text()
    // }

    //fn draw_text(&mut self, layout: &TextLayout, pos: impl Into<Point>) {
    fn draw_text(&mut self, layout: &TextLayout, pos: Value) {
        self.de().draw_text(layout.as_ref(), Point::from(pos) );
    }

    fn save(&mut self) -> Result<(), Error> {
        self.de().save().map_err( |e| Error::from(e) )
    }

    fn restore(&mut self) -> Result<(), Error> {
        self.de().restore().map_err( |e| Error::from(e) )
    }

    // Discussion question: should this subsume EndDraw, with BeginDraw on
    // D2DRenderContext creation? I'm thinking not, as the shell might want
    // to do other stuff, possibly related to incremental paint.
    fn finish(&mut self) -> Result<(), Error> {
        self.de().finish().map_err( |e| Error::from(e) )
    }

    fn transform(&mut self, transform: Affine) {
        self.de().transform( transform.into() )
    }

    fn current_transform(&self) -> Affine {
        Affine(self.de().current_transform())
    }

    fn make_image(
        &mut self,
        width: usize,
        height: usize,
        buf: &[u8],
        format: ImageFormat,
    ) -> Result<Image, Error> {
        self.de().make_image(width,height,buf,format.into())
            .map(|v| Image(v))
            .map_err( |e| Error::from(e) )
    }

    #[inline]
    //fn draw_image(&mut self,image: &Image,dst_rect: impl Into<Rect>,interp: InterpolationMode,) {
    fn draw_image(&mut self,image: &Image,dst_rect: Rect,interp: InterpolationMode,) {
        self.de().draw_image(image.as_ref(), Into::<druid::Rect>::into(dst_rect), interp.into());
    }

    #[inline]
    //fn draw_image_area(&mut self,image: &Image,src_rect: impl Into<Rect>,dst_rect: impl Into<Rect>,interp: InterpolationMode,) {
    fn draw_image_area(&mut self,image: &Image,src_rect: Rect,dst_rect: Rect,interp: InterpolationMode,) {
        self.de().draw_image_area(image.as_ref(), Into::<druid::Rect>::into(src_rect), Into::<druid::Rect>::into(dst_rect), interp.into());
    }

    //fn capture_image_area(&mut self, rect: impl Into<Rect>) -> Result<Image, Error> {
    fn capture_image_area(&mut self, rect: Rect) -> Result<Image, Error> {
        self.de().capture_image_area(Into::<druid::Rect>::into(rect)).map(|v| Image(v)).map_err( |e| Error::from(e) )
    }

    //fn blurred_rect(&mut self, rect: Rect, blur_radius: f64, brush: &impl IntoBrush<Self>) {
    fn blurred_rect(&mut self, rect: Rect, blur_radius: f64, brush: Color) {
        let c:druid::Color = brush.into();
        self.de().blurred_rect(rect.into(), blur_radius, &c);
    }
}
