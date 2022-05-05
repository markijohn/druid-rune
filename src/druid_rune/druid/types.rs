// rune AnyObj enum not support(support only rust primitive). so some types are wrap as struct
// u8 -> u32, because rune script u8 type is b'a'. not support integer
// & -> rune script can't return reference
// 'self' -> '&self' : Rune script can't pre-detect 'self' consumed fn.
//  It's raised NoRef rune error.

use crate::{type_resolve_impl, register_impl};
use std::any::TypeId;
use std::fmt::Formatter;
use rune::{Any, FromValue, runtime::Protocol, Value};
use druid::piet;
use druid::piet::kurbo;
use rune::compile::Named;
use rune::runtime::Function;
use super::rune_ext::ValueExt;


pub fn install_types<T:'static + rune::compile::Named + druid::Data>(module:&mut rune::Module) -> Result<(), rune::ContextError> {
    //not support Any constant type
    //module.constant(&["Color","AQUA"],         Color::rgb8(0, 255, 255) );
    module.constant(&["Color","AQUA"],         (0, 255, 255) );
    module.constant(&["Color","BLACK"],        (0, 0, 0) );
    module.constant(&["Color","BLUE"],         (0, 0, 255) );
    module.constant(&["Color","FUCHSIA"],      (255, 0, 255) );
    module.constant(&["Color","GRAY"],         (128, 128, 128) );
    module.constant(&["Color","GREEN"],        (0, 128, 0) );
    module.constant(&["Color","LIME"],         (0, 255, 0) );
    module.constant(&["Color","MAROON"],       (128, 0, 0) );
    module.constant(&["Color","NAVY"],         (0, 0, 128) );
    module.constant(&["Color","OLIVE"],        (128, 128, 0) );
    module.constant(&["Color","PURPLE"],       (128, 0, 128) );
    module.constant(&["Color","RED"],          (255, 0, 0) );
    module.constant(&["Color","SILVER"],       (192, 192, 192) );
    module.constant(&["Color","TEAL"],         (0, 128, 128) );
    module.constant(&["Color","TRANSPARENT"],  (0, 0, 0, 0) );
    module.constant(&["Color","WHITE"],        (255, 255, 255) );
    module.constant(&["Color","YELLOW"],       (255, 255, 0) );

    module.ty::<LineCap>();
    module.ty::<LineJoin>();
    register_impl!(module, StrokeStyle, [], [], [new], [line_join,line_cap,dash_offset,dash_pattern,set_line_join,set_line_cap,set_dash_offset,set_dash_pattern,miter_limit]);

    //hold : AQUA,BLACK,BLUE,FUCHSIA,GRAY,GREEN,LIME,MAROON,NAVY,OLIVE,PURPLE,RED,SILVER,TEAL,TRANSPARENT,WHITE,YELLOW
    register_impl!(module, Color, [],[]
        , [rgb8,rgba8,from_rgba32_u32,from_hex_str,grey8,grey,rgba,rgb,hlc,hlca]
        , [with_alpha,as_rgba_u32,as_rgba8,as_rgba]);

    register_impl!(module, Size, [],[]
    ,[new]
    ,[max_side,min_side,area,is_empty,clamp,to_vec2,round,ceil,floor,expand,aspect_ratio,to_rect,to_rounded_rect,is_finite,is_nan]);

    //ZERO,ORIGIN
    register_impl!(module, Point, [],[]
    ,[new]
    ,[to_vec2,lerp,midpoint,distance,round]);

    //IDENTITY,FLIP_Y,FLIP_X
    register_impl!(module, Affine, [],[]
    ,[new,scale,scale_non_uniform,rotate,translate,map_unit_square]
    ,[as_coeffs,determinant,inverse,transform_rect_bbox,is_finite,is_nan]);

    //ZERO
    register_impl!(module, Vec2, [],[x,y]
    ,[new]
    ,[to_point,to_size,dot,cross,hypot,hypot2,atan2,from_angle,lerp,normalize,round,ceil,floor,expand,trunc,is_finite,is_nan]);

    register_impl!(module, Line, [],[]
    ,[new]
    ,[length,crossing_point,is_finite,is_nan]);

    //ZERO
    register_impl!(module, Insets, [],[]
    ,[uniform,uniform_xy,new]
    ,[x_value,y_value,size,are_nonnegative,nonnegative,is_finite,is_nan]);

    register_impl!(module, PathEl, [],[]
    ,[MoveTo,LineTo,QuadTo,CurveTo,ClosePath]
    ,[]);

    register_impl!(module, BezPath, [],[]
    ,[new,from_vec]
    ,[push,move_to,line_to,quad_to,curve_to,close_path,elements,flatten,get_seg,is_empty,apply_affine,is_finite,is_nan]);

    //EMPTY
    register_impl!(module, Region, [],[]
    ,[]
    ,[rects,add_rect,set_rect,clear,bounding_box,intersects,is_empty,to_bez_path,union_with,intersect_with]);

    register_impl!(module, Circle, [],[]
    ,[new]
    ,[segment,is_finite,is_nan]);

    register_impl!(module, CircleSegment, [],[]
    ,[new]
    ,[is_finite,is_nan]);

    register_impl!(module, Ellipse, [],[]
    ,[new,from_rect,from_affine]
    ,[with_center,with_radii,with_rotation,center,radii,rotation,is_finite,is_nan]);

    register_impl!(module, Rect, [],[]
    ,[new,from_points,from_origin_size,from_center_size]
    ,[with_origin,with_size,inset,width,height,min_x,max_x,min_y,max_y,origin,size,area,is_empty,center,contains,abs,union,union_pt,intersect,inflate,round,ceil,floor,expand,trunc,scale_from_origin,to_ellipse,to_rounded_rect,aspect_ratio,contained_rect_with_aspect_ratio,is_finite,is_nan]);

    register_impl!(module, RoundedRectRadii, [],[]
    ,[new,from_single_radius]
    ,[abs,clamp,is_finite,is_nan,as_single_radius]);

    register_impl!(module, RoundedRect, [],[]
    ,[new,from_rect,from_points,from_origin_size]
    ,[width,height,radii,rect,origin,center,is_finite,is_nan]);

    module.ty::<InterpolationMode>()?;

    module.ty::<ImageFormat>();

    register_impl!(module, QuadSpline, [],[]
    ,[new]
    ,[points]);

    register_impl!(module, CubicBez, [],[]
    ,[new]
    ,[to_quads,approx_spline,is_finite,is_nan]);

    register_impl!(module, QuadBez, [],[]
    ,[new]
    ,[raise,is_finite,is_nan]);

    register_impl!(module, PathSeg, [],[]
    ,[Line,Quad,Cubic]
    ,[]);

    module.function( &["BackgroundBrush","Color"], BackgroundBrush::<T>::Color)?;
    Ok(())
}

#[derive(Any,Debug)]
pub enum LineCap {
    #[rune(constructor)]Butt,
    #[rune(constructor)]Round,
    #[rune(constructor)]Square
}

impl Into<piet::LineCap> for LineCap {
    fn into(self) -> piet::LineCap {
        match self {
            LineCap::Butt => piet::LineCap::Butt,
            LineCap::Round => piet::LineCap::Round,
            LineCap::Square => piet::LineCap::Square,
        }
    }
}

#[derive(Any,Clone,Debug)]
pub enum LineJoin {
    #[rune(constructor)]Miter{#[rune(get,set)] limit:f64},
    #[rune(constructor)]Round,
    #[rune(constructor)]Bevel
}

impl Into<piet::LineJoin> for LineJoin {
    fn into(self) -> piet::LineJoin {
        match self {
            LineJoin::Miter{limit} => piet::LineJoin::Miter{limit},
            LineJoin::Round => piet::LineJoin::Round,
            LineJoin::Bevel => piet::LineJoin::Bevel
        }
    }
}

impl LineJoin {
    pub const DEFAULT_MITER_LIMIT: f64 = piet::LineJoin::DEFAULT_MITER_LIMIT;
}

#[derive(Any,Clone,Debug)]
pub struct StrokeStyle(pub piet::StrokeStyle);

impl Into<piet::StrokeStyle> for StrokeStyle {
    fn into(self) -> piet::StrokeStyle {
        self.0
    }
}

impl AsRef<piet::StrokeStyle> for StrokeStyle {
    fn as_ref(&self) -> &piet::StrokeStyle {
        &self.0
    }
}

impl StrokeStyle {
    pub const fn new() -> StrokeStyle {
        StrokeStyle(piet::StrokeStyle::new())
    }

    //pub fn line_join(mut self, line_join: LineJoin) -> Self {
    pub fn line_join(&mut self, line_join: LineJoin) -> Self {
        let mut n = self.clone();
        n.set_line_join(line_join);
        n
    }

    //pub fn line_cap(mut self, line_cap: LineCap) -> Self {
    pub fn line_cap(&mut self, line_cap: LineCap) -> Self {
        let mut n = self.clone();
        n.set_line_cap(line_cap);
        n
    }

    //pub fn dash_offset(mut self, offset: f64) -> Self {
    pub fn dash_offset(&mut self, offset: f64) -> Self {
        let mut n = self.clone();
        n.set_dash_offset(offset);
        n
    }

    //pub fn dash_pattern(mut self, lengths: &'static [f64]) -> Self {
    pub fn dash_pattern(&mut self, lengths: Vec<f64>) -> Self {
        //Self(self.0.dash_pattern(lengths))
        unimplemented!()
    }

    pub fn set_line_join(&mut self, line_join: LineJoin) {
        self.0.set_line_join(line_join.into())
    }

    pub fn set_line_cap(&mut self, line_cap: LineCap) {
        self.0.set_line_cap(line_cap.into())
    }

    pub fn set_dash_offset(&mut self, offset: f64) {
        self.0.set_dash_offset(offset)
    }

    //pub fn set_dash_pattern(&mut self, lengths: impl Into<std::rc::Rc<[f64]>>) {
    pub fn set_dash_pattern(&mut self, lengths: Vec<f64>) {
        //self.dash_pattern.alloc = Some(lengths.into());
        unimplemented!()
    }

    pub fn miter_limit(&self) -> Option<f64> {
        self.0.miter_limit()
    }
}

#[derive(Debug,Clone,Copy,Any)]
pub enum Color {
    #[rune(constructor)]
    Rgba32(#[rune(get,set)] u32)
}

impl Color {
    //pub const fn rgb8(r: u8, g: u8, b: u8) -> Color {
    pub const fn rgb8(r: u32, g: u32, b: u32) -> Color {
        Color::from_rgba32_u32(((r as u32) << 24) | ((g as u32) << 16) | ((b as u32) << 8) | 0xff)
    }

    //pub const fn rgba8(r: u8, g: u8, b: u8, a: u8) -> Color {
    pub const fn rgba8(r: u32, g: u32, b: u32, a: u32) -> Color {
        Color::from_rgba32_u32(
            ((r as u32) << 24) | ((g as u32) << 16) | ((b as u32) << 8) | (a as u32),
        )
    }

    pub const fn from_rgba32_u32(rgba: u32) -> Color {
        Color::Rgba32(rgba)
    }

    //pub const fn from_hex_str(hex: &str) -> Color {
    pub fn from_hex_str(hex: &str) -> Color {
        Self::from_rgba32_u32( druid::Color::from_hex_str(hex).unwrap().as_rgba_u32() )
    }

    //pub const fn grey8(grey: u8) -> Color {
    pub const fn grey8(grey: u32) -> Color {
        Color::rgb8(grey, grey, grey)
    }

    pub fn grey(grey: f64) -> Color {
        Color::rgb(grey, grey, grey)
    }

    pub fn rgba(r: f64, g: f64, b: f64, a: f64) -> Color {
        Self::from_rgba32_u32( druid::Color::rgba(r,g,b,a).as_rgba_u32() )
    }

    pub fn rgb(r: f64, g: f64, b: f64) -> Color {
        Self::from_rgba32_u32( druid::Color::rgb(r,g,b).as_rgba_u32() )
    }

    pub fn hlc(h: f64, L: f64, c: f64) -> Color {
        Self::from_rgba32_u32( druid::Color::hlc(h,L,c).as_rgba_u32() )
    }

    pub fn hlca(h: f64, l: f64, c: f64, a: f64) -> Color {
        Color::hlc(h, c, l).with_alpha(a)
    }

    //pub fn with_alpha(self, a: f64) -> Color {
    pub fn with_alpha(&self, a: f64) -> Color {
        let a = (a.max(0.0).min(1.0) * 255.0).round() as u32;
        Color::from_rgba32_u32((self.as_rgba_u32() & !0xff) | a)
    }

    pub fn as_rgba_u32(&self) -> u32 {
        match *self {
            Color::Rgba32(rgba) => rgba,
        }
    }

    pub fn as_rgba8(&self) -> (u8, u8, u8, u8) {
        let rgba = self.as_rgba_u32();
        (
            (rgba >> 24 & 255) as u8,
            ((rgba >> 16) & 255) as u8,
            ((rgba >> 8) & 255) as u8,
            (rgba & 255) as u8,
        )
    }

    pub fn as_rgba(&self) -> (f64, f64, f64, f64) {
        let rgba = self.as_rgba_u32();
        (
            (rgba >> 24) as f64 / 255.0,
            ((rgba >> 16) & 255) as f64 / 255.0,
            ((rgba >> 8) & 255) as f64 / 255.0,
            (rgba & 255) as f64 / 255.0,
        )
    }
    pub fn to_native(&self) -> druid::Color {
        self.clone().into()
    }

}

impl Into<druid::Color> for Color {
    fn into(self) -> druid::Color {
        match self {
            Color::Rgba32(c) => druid::Color::from_rgba32_u32(c)
        }
    }
}

impl From<Value> for Color {
    fn from(v:Value) -> Self {
        if let Some( (r,g,b) ) = v.resolve_tuple3::<u32>() {
            Color::rgb8(r,g,b)
        } else if let Some( (r,g,b,a) ) = v.resolve_tuple4::<u32>() {
            Color::rgba8( r,g,b,a )
        } else {
            v.resolve_as_copy::<Self>().unwrap()
        }
        //
        // match v {
        //     Value::Tuple(v) => {
        //         let v = Vec::from( v.take().unwrap().into_inner() ); //.into_iter()
        //         let len = v.len();
        //         let mut into_iner = v.into_iter();
        //         match len {
        //             3 => Color::rgb8( u32::from_value(into_iner.next().unwrap()).unwrap(), u32::from_value(into_iner.next().unwrap()).unwrap(), u32::from_value(into_iner.next().unwrap()).unwrap() ),
        //             4 => Color::rgba8( u32::from_value(into_iner.next().unwrap()).unwrap(), u32::from_value(into_iner.next().unwrap()).unwrap(), u32::from_value(into_iner.next().unwrap()).unwrap(), u32::from_value(into_iner.next().unwrap()).unwrap() ),
        //             _ => panic!("invalid Color value")
        //         }
        //     },
        //     Value::Any(_) => Self::from_value(v).unwrap(),
        //     _ => panic!("invalid Color value")
        // }
    }
}

#[derive(Any, Debug, Clone)]
pub struct FixedGradient(piet::FixedGradient);

//Can't make as enum : [E0658]: associated type bounds are unstable
#[derive(Any)]
pub struct BackgroundBrush<T:'static + Named + druid::Data> {
    origin : druid::widget::BackgroundBrush<T>
    // Color( Color),
    // Linear(LinearGradient),
    // Radial(RadialGradient),
    // Fixed(FixedGradient),
    // Painter(Painter<T>),
}

impl <T:'static + Named + druid::Data> std::fmt::Debug for BackgroundBrush<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "BackgroundBrush")
    }
}

impl <T:'static + Named + druid::Data> BackgroundBrush<T> {
    pub fn Color(c:Color) -> Self {
        BackgroundBrush{ origin : druid::widget::BackgroundBrush::Color(c.into()) }
    }

    pub fn to_native(self) -> druid::widget::BackgroundBrush<T> {
        self.origin
    }
}

impl <T:'static + Named + druid::Data> From<Color> for BackgroundBrush<T> {
    fn from(c: Color) -> Self {
        Self::Color(c)
    }
}

impl <T:'static + Named + druid::Data> From<Value> for BackgroundBrush<T> {
    fn from(v: Value) -> Self {
        if let Value::Tuple(_) = v {
            return BackgroundBrush::Color( Color::from(v) )
        }
        type_resolve_impl!(v, {BackgroundBrush<T>,Color}, |resolved| {
            BackgroundBrush::from(resolved)
        })
    }
}


#[derive(Any,Clone,Copy)]
pub struct Size(pub kurbo::Size);

impl Into<kurbo::Size> for Size {
    fn into(self) -> kurbo::Size {
        self.0
    }
}

impl Size {
    pub const fn new(w:f64, h:f64) -> Self {
        Self(kurbo::Size::new(w,h))
    }

    //pub fn max_side(self) -> f64 {
    pub fn max_side(&self) -> f64 {
        self.0.max_side()
    }

    //pub fn min_side(self) -> f64 {
    pub fn min_side(&self) -> f64 {
        self.0.min_side()
    }

    #[inline]
    //pub fn area(self) -> f64 {
    pub fn area(&self) -> f64 {
        self.0.area()
    }

    #[inline]
    //pub fn is_empty(self) -> bool {
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    //pub fn clamp(self, min: Size, max: Size) -> Self {
    pub fn clamp(&self, min: Size, max: Size) -> Self {
        Self ( self.0.clamp(min.to_native(), max.to_native() ) )
    }

    #[inline]
    //pub const fn to_vec2(self) -> Vec2 {
    pub const fn to_vec2(&self) -> Vec2 {
        Vec2(self.0.to_vec2())
    }

    #[inline]
    //pub fn round(self) -> Size {
    pub fn round(&self) -> Size {
        Size(self.0.round())
    }

    #[inline]
    //pub fn ceil(self) -> Size {
    pub fn ceil(&self) -> Size {
        Size(self.0.ceil())
    }

    #[inline]
    //pub fn floor(self) -> Size {
    pub fn floor(&self) -> Size {
        Size(self.0.floor())
    }

    #[inline]
    //pub fn expand(self) -> Size {
    pub fn expand(&self) -> Size {
        Size(self.0.expand())
    }

    #[inline]
    //pub fn trunc(self) -> Size {
    pub fn trunc(&self) -> Size {
        Size(self.0.trunc())
    }

    //pub fn aspect_ratio(self) -> f64 {
    pub fn aspect_ratio(&self) -> f64 {
        self.0.aspect_ratio()
    }

    #[inline]
    //pub const fn to_rect(self) -> Rect {
    pub const fn to_rect(&self) -> Rect {
        Rect(self.0.to_rect())
    }

    #[inline]
    //pub fn to_rounded_rect(self, radii: impl Into<RoundedRectRadii>) -> RoundedRect {
    pub fn to_rounded_rect(&self, radii: RoundedRectRadii) -> RoundedRect {
        RoundedRect( self.0.to_rounded_rect( radii.to_native() ) )
    }

    #[inline]
    //pub fn is_finite(self) -> bool {
    pub fn is_finite(&self) -> bool {
        self.0.is_finite()
    }

    #[inline]
    //pub fn is_nan(self) -> bool {
    pub fn is_nan(&self) -> bool {
        self.0.is_nan()
    }

    #[inline]
    pub fn to_native(&self) -> kurbo::Size {
        self.0.clone()
    }
}

impl From<Value> for Size {
    fn from(v:Value) -> Self {
        if let Some( (w,h) ) = v.resolve_tuple2::<f64>() {
            Size::new(w,h)
        } else {
            v.resolve_as_copy::<Self>().unwrap()
        }
    }
}


#[derive(Any,Clone,Copy)]
pub struct Point(pub kurbo::Point);

impl Into<kurbo::Point> for Point {
    fn into(self) -> druid::Point {
        self.0
    }
}

impl Point {
    pub const ZERO: Point = Point::new(0. ,0.);
    pub const ORIGIN: Point = Point::new(0. ,0.);

    pub const fn new(x: f64, y: f64) -> Point {
        Self(kurbo::Point::new(x,y))
    }

    pub const fn to_vec2(self) -> Vec2 {
        Vec2(self.0.to_vec2())
    }

    pub fn lerp(self, other: Point, t: f64) -> Point {
        Point(self.0.lerp(other.into(),t))
    }

    pub fn midpoint(self, other: Point) -> Point {
        Point(self.0.midpoint(other.into()))
    }

    pub fn distance(self, other: Point) -> f64 {
        self.0.distance(other.into())
    }

    pub fn round(self) -> Point {
        Point(self.0.round())
    }

    pub fn to_native(self) -> kurbo::Point {
        self.into()
    }
}

impl From<Value> for Point {
    fn from(v:Value) -> Self {
        if let Some( (x,y) ) = v.resolve_tuple2::<f64>() {
            Point::new(x,y)
        } else {
            v.resolve_as_copy::<Self>().unwrap()
        }
    }
}

#[derive(Any, Clone, Copy, Debug, PartialEq)]
pub struct Affine(pub kurbo::Affine);

impl Into<kurbo::Affine> for Affine {
    fn into(self) -> druid::Affine {
        self.0
    }
}

impl Affine {
    pub const IDENTITY: Affine = Affine(kurbo::Affine::IDENTITY);

    pub const FLIP_Y: Affine = Affine(kurbo::Affine::FLIP_Y);

    pub const FLIP_X: Affine = Affine(kurbo::Affine::FLIP_X);

    pub fn to_native(&self) -> druid::Affine {
        self.0
    }

    #[inline]
    //pub const fn new(c: [f64; 6]) -> Affine {
    pub fn new(c: Vec<f64>) -> Affine {
        let mut a = [0.; 6];
        a.copy_from_slice( c.as_slice() );
        Affine( kurbo::Affine::new(a) )
    }

    #[inline]
    pub const fn scale(s: f64) -> Affine {
        Affine( kurbo::Affine::scale(s) )
    }

    #[inline]
    pub const fn scale_non_uniform(s_x: f64, s_y: f64) -> Affine {
        Affine( kurbo::Affine::scale_non_uniform(s_x,s_y) )
    }

    #[inline]
    pub fn rotate(th: f64) -> Affine {
        Affine( kurbo::Affine::rotate(th) )
    }

    #[inline]
    //pub fn translate<V: Into<Vec2>>(p: V) -> Affine {
    pub fn translate(p: Vec2) -> Affine {
        let v:druid::Vec2 = p.into();
        Affine( kurbo::Affine::translate(v) )
    }

    pub fn map_unit_square(rect: Rect) -> Affine {
        Affine( kurbo::Affine::map_unit_square(rect.into()) )
    }

    #[inline]
    //pub fn as_coeffs(self) -> [f64; 6] {
    pub fn as_coeffs(&self) -> Vec<f64> {
        self.0.clone().as_coeffs().to_vec()
    }

    //pub fn determinant(self) -> f64 {
    pub fn determinant(&self) -> f64 {
        self.0.clone().determinant()
    }

    //pub fn inverse(self) -> Affine {
    pub fn inverse(&self) -> Affine {
        Affine( self.0.clone().inverse() )
    }

    //pub fn transform_rect_bbox(self, rect: Rect) -> Rect {
    pub fn transform_rect_bbox(&self, rect: Rect) -> Rect {
        Rect( self.0.transform_rect_bbox(rect.to_native()) )
    }

    #[inline]
    pub fn is_finite(&self) -> bool {
        self.0.is_finite()
    }

    /// Is this map NaN?
    #[inline]
    pub fn is_nan(&self) -> bool {
        self.0.is_nan()
    }
}

#[derive(Any,Clone,Copy)]
pub struct Vec2(kurbo::Vec2);

impl Into<kurbo::Vec2> for Vec2 {
    fn into(self) -> druid::Vec2 {
        self.0
    }
}

impl Vec2 {
    pub const ZERO: Vec2 = Vec2::new(0. ,0.);

    pub const fn new(x: f64, y: f64) -> Vec2 {
        Self(druid::piet::kurbo::Vec2::new(x,y))
    }

    #[inline]
    //pub fn x(self) -> f64 {
    pub fn x(&self) -> f64 {
        self.0.x
    }

    #[inline]
    //pub fn y(self) -> f64 {
    pub fn y(&self) -> f64 {
        self.0.y
    }

    #[inline]
    //pub const fn to_point(self) -> Point {
    pub const fn to_point(&self) -> Point {
        Point(self.0.to_point())
    }

    #[inline]
    //pub const fn to_size(self) -> Size {
    pub const fn to_size(&self) -> Size {
        Size(self.0.to_size())
    }

    #[inline]
    //pub fn dot(self, other: Vec2) -> f64 {
    pub fn dot(&self, other: Vec2) -> f64 {
        self.0.dot(other.into())
    }

    #[inline]
    //pub fn cross(self, other: Vec2) -> f64 {
    pub fn cross(&self, other: Vec2) -> f64 {
        self.0.cross( other.into() )
    }

    #[inline]
    //pub fn hypot(self) -> f64 {
    pub fn hypot(&self) -> f64 {
        self.0.hypot()
    }

    #[inline]
    //pub fn hypot2(self) -> f64 {
    pub fn hypot2(&self) -> f64 {
        self.0.hypot2()
    }

    #[inline]
    //pub fn atan2(self) -> f64 {
    pub fn atan2(&self) -> f64 {
        self.0.atan2()
    }

    #[inline]
    pub fn from_angle(th: f64) -> Vec2 {
        Vec2(kurbo::Vec2::from_angle(th))
    }

    #[inline]
    //pub fn lerp(self, other: Vec2, t: f64) -> Vec2 {
    pub fn lerp(&self, other: Vec2, t: f64) -> Vec2 {
        Vec2(self.0.lerp(other.into(),t))
    }

    #[inline]
    //pub fn normalize(self) -> Vec2 {
    pub fn normalize(&self) -> Vec2 {
        Vec2(self.0.normalize())
    }

    #[inline]
    //pub fn round(self) -> Vec2 {
    pub fn round(&self) -> Vec2 {
        Vec2(self.0.round())
    }

    #[inline]
    //pub fn ceil(self) -> Vec2 {
    pub fn ceil(&self) -> Vec2 {
        Vec2(self.0.ceil())
    }

    #[inline]
    //pub fn floor(self) -> Vec2 {
    pub fn floor(&self) -> Vec2 {
        Vec2(self.0.floor())
    }

    #[inline]
    //pub fn expand(self) -> Vec2 {
    pub fn expand(&self) -> Vec2 {
        Vec2(self.0.expand())
    }

    #[inline]
    //pub fn trunc(self) -> Vec2 {
    pub fn trunc(&self) -> Vec2 {
        Vec2(self.0.trunc())
    }

    #[inline]
    //pub fn is_finite(self) -> bool {
    pub fn is_finite(&self) -> bool {
        self.0.is_finite()
    }

    #[inline]
    //pub fn is_nan(self) -> bool {
    pub fn is_nan(&self) -> bool {
        self.0.is_nan()
    }

    #[inline]
    pub fn to_native(&self) -> kurbo::Vec2 {
        self.0
    }
}

impl From<Value> for Vec2 {
    fn from(v:Value) -> Self {
        if let Some( (x,y) ) = v.resolve_tuple2::<f64>() {
            Vec2::new(x,y)
        } else {
            v.resolve_as_copy::<Self>().unwrap()
        }
    }
}


#[derive(Any,Clone,Copy)]
pub struct Line( kurbo::Line );

impl Into<kurbo::Line> for Line {
    fn into(self) -> kurbo::Line {
        self.0
    }
}

impl Line {
    pub fn to_native(&self) -> kurbo::Line {
        self.0
    }

    //pub fn new(p0: impl Into<Point>, p1: impl Into<Point>) -> Line {
    pub fn new(p0: Value, p1: Value) -> Line {
        let p0:kurbo::Point = Point::from(p0).into();
        let p1:kurbo::Point = Point::from(p1).into();
        Self( kurbo::Line::new(p0, p1) )
    }

    #[inline]
    //pub fn length(self) -> f64 {
    pub fn length(&self) -> f64 {
        self.0.length()
    }

    //pub fn crossing_point(self, other: Line) -> Option<Point> {
    pub fn crossing_point(&self, other: Line) -> Option<Point> {
        self.0.crossing_point( other.into() ).map( |v| Point(v) )
    }

    #[inline]
    //pub fn is_finite(self) -> bool {
    pub fn is_finite(&self) -> bool {
        self.0.is_finite()
    }

    #[inline]
    //pub fn is_nan(self) -> bool {
    pub fn is_nan(&self) -> bool {
        self.0.is_nan()
    }
}


#[derive(Any,Clone, Copy, Default, Debug, PartialEq)]
pub struct Insets(kurbo::Insets);

impl Into<kurbo::Insets> for Insets {
    fn into(self) -> druid::Insets {
        self.0
    }
}

impl Insets {
    pub const ZERO: Insets = Insets::uniform(0.);

    #[inline]
    pub const fn uniform(d: f64) -> Insets {
        Insets(kurbo::Insets::uniform(d))
    }

    #[inline]
    pub const fn uniform_xy(x: f64, y: f64) -> Insets {
        Insets(kurbo::Insets::uniform_xy(x,y))
    }

    #[inline]
    pub const fn new(x0: f64, y0: f64, x1: f64, y1: f64) -> Insets {
        Insets(kurbo::Insets::new(x0,y0,x1,y1))
    }

    #[inline]
    //pub fn x_value(self) -> f64 {
    pub fn x_value(&self) -> f64 {
        self.0.x_value()
    }

    #[inline]
    //pub fn y_value(self) -> f64 {
    pub fn y_value(&self) -> f64 {
        self.0.y_value()
    }

    //pub fn size(self) -> Size {
    pub fn size(&self) -> Size {
        Size(self.0.size())
    }

    //pub fn are_nonnegative(self) -> bool {
    pub fn are_nonnegative(&self) -> bool {
        self.0.are_nonnegative()
    }

    //pub fn nonnegative(self) -> Insets {
    pub fn nonnegative(&self) -> Insets {
        Insets(self.0.nonnegative())
    }

    #[inline]
    pub fn is_finite(&self) -> bool {
        self.0.is_finite()
    }

    #[inline]
    pub fn is_nan(&self) -> bool {
        self.0.is_nan()
    }

    #[inline]
    pub fn to_native(&self) -> druid::Insets {
        self.0
    }
}

impl From<Value> for Insets {
    fn from(v: Value) -> Self {
        if let Value::Float(v) = v {
            Insets::uniform(v)
        } else if let Some( (x,y) ) = v.resolve_tuple2::<f64>() {
            Insets::uniform_xy( x, y )
        } else if let Some( (x0,y0,x1,y1) ) = v.resolve_tuple4::<f64>() {
            Insets::new( x0, y0, x1, y1 )
        } else {
            v.resolve_as_copy::<Self>().unwrap()
        }
    }
}

impl From<f64> for Insets {
    fn from(src: f64) -> Insets {
        Insets::uniform(src)
    }
}

impl From<(f64, f64)> for Insets {
    fn from(src: (f64, f64)) -> Insets {
        Insets::uniform_xy(src.0, src.1)
    }
}

impl From<(f64, f64, f64, f64)> for Insets {
    fn from(src: (f64, f64, f64, f64)) -> Insets {
        Insets::new(src.0, src.1, src.2, src.3)
    }
}


#[derive(Any,Clone,Copy,Debug)]
pub struct PathEl(pub kurbo::PathEl);

impl Into<kurbo::PathEl> for PathEl {
    fn into(self) -> kurbo::PathEl {
        self.0
    }
}

impl PathEl {
    pub fn MoveTo(p:&Point) -> Self { Self(kurbo::PathEl::MoveTo(p.to_native())) }
    pub fn LineTo(p:&Point) -> Self { Self(kurbo::PathEl::LineTo(p.to_native())) }
    pub fn QuadTo(p1:&Point, p2:&Point) -> Self { Self(kurbo::PathEl::QuadTo(p1.to_native(),p2.to_native())) }
    pub fn CurveTo(p1:&Point, p2:&Point, p3:&Point) -> Self { Self(kurbo::PathEl::CurveTo(p1.to_native(),p2.to_native(),p3.to_native())) }
    pub fn ClosePath() -> Self { Self(kurbo::PathEl::ClosePath) }
}

#[derive(Any,Clone, Default, Debug, PartialEq)]
pub struct BezPath(pub kurbo::BezPath);

impl Into<kurbo::BezPath> for BezPath {
    fn into(self) -> kurbo::BezPath {
        self.0
    }
}

impl BezPath {
    pub fn new() -> BezPath {
        Default::default()
    }
    pub fn from_vec(v: Vec<PathEl>) -> BezPath {
        BezPath( kurbo::BezPath::from_vec(v.into_iter().map(|v| v.into()).collect()) )
    }

    pub fn push(&mut self, el: &PathEl) {
        self.0.push(el.0 )
    }

    //pub fn move_to<P: Into<Point>>(&mut self, p: P) {
    pub fn move_to(&mut self, p: &Point) {
        self.push(&PathEl::MoveTo(p) );
    }

    //pub fn line_to<P: Into<Point>>(&mut self, p: P) {
    pub fn line_to(&mut self, p: &Point) {
        self.push( &PathEl::LineTo(p) );
    }

    //pub fn quad_to<P: Into<Point>>(&mut self, p1: P, p2: P) {
    pub fn quad_to(&mut self, p1: &Point, p2: &Point) {
        self.push(&PathEl::QuadTo(p1, p2));
    }

    //pub fn curve_to<P: Into<Point>>(&mut self, p1: P, p2: P, p3: P) {
    pub fn curve_to(&mut self, p1: &Point, p2: &Point, p3: &Point) {
        self.push(&PathEl::CurveTo(p1, p2, p3) );
    }

    pub fn close_path(&mut self) {
        self.push(&PathEl::ClosePath() );
    }

    //pub fn elements(&self) -> &[PathEl] {
    pub fn elements(&self) -> Vec<PathEl> {
        self.0.elements().iter().map(|v| PathEl( v.clone() )).collect()
    }

    // pub fn iter(&self) -> impl Iterator<Item = PathEl> + '_ {
    //     self.0.iter().copied()
    // }

    // pub fn segments(&self) -> impl Iterator<Item = PathSeg> + '_ {
    //     self.0.segments()
    // }

    //pub fn flatten(&self, tolerance: f64, callback: impl FnMut(PathEl)) {
    pub fn flatten(&self, tolerance: f64, callback: Function) {
        self.0.flatten(tolerance, |p:kurbo::PathEl| {
            let _:() = callback.call( (PathEl(p),) ).unwrap();
        });
    }

    pub fn get_seg(&self, ix: usize) -> Option<PathSeg> {
        self.0.get_seg(ix).map(|v| PathSeg(v))
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn apply_affine(&mut self, affine: Affine) {
        self.0.apply_affine( affine.into() );
    }

    #[inline]
    pub fn is_finite(&self) -> bool {
        self.0.is_finite()
    }

    #[inline]
    pub fn is_nan(&self) -> bool {
        self.0.is_nan()
    }
}


#[derive(Any,Clone, Debug)]
pub struct Region(pub druid::Region);

impl Into<druid::Region> for Region {
    fn into(self) -> druid::Region {
        self.0
    }
}

impl Region {
    pub const EMPTY: Region = Region(druid::Region::EMPTY);

    #[inline]
    //pub fn rects(&self) -> &[Rect] {
    pub fn rects(&self) -> Vec<Rect> {
        self.0.rects().iter().map(|v| Rect(v.clone()) ).collect()
    }

    pub fn add_rect(&mut self, rect: Rect) {
        self.0.add_rect( rect.into() );
    }

    pub fn set_rect(&mut self, rect: Rect) {
        self.0.set_rect( rect.into() );
    }

    pub fn clear(&mut self) {
        self.0.clear();
    }

    pub fn bounding_box(&self) -> Rect {
        Rect(self.0.bounding_box())
    }

    pub fn intersects(&self, rect: Rect) -> bool {
        self.0.intersects( rect.into() )
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn to_bez_path(&self) -> BezPath {
        BezPath( self.0.to_bez_path() )
    }

    pub fn union_with(&mut self, other: &Region) {
        self.0.union_with( &other.0 );
    }

    pub fn intersect_with(&mut self, rect: Rect) {
        self.0.intersect_with(rect.into());
    }
}

impl From<Rect> for Region {
    fn from(rect: Rect) -> Region {
        Region(druid::Region::from(rect.to_native()))
    }
}

#[derive(Any, Clone, Copy, Default, PartialEq)]
pub struct Circle(pub kurbo::Circle);

impl Into<kurbo::Circle> for Circle {
    fn into(self) -> kurbo::Circle {
        self.0
    }
}

impl Circle {
    pub fn to_native(&self) -> kurbo::Circle {
        self.0.clone()
    }

    #[inline]
    //pub fn new(center: impl Into<Point>, radius: f64) -> Circle {
    pub fn new(center: Value, radius: f64) -> Circle {
        let p: kurbo::Point = Point::from(center).into();
        Circle( kurbo::Circle::new(p, radius) )
    }

    /// Create a [`CircleSegment`] by cutting out parts of this circle.
    pub fn segment(self, inner_radius: f64, start_angle: f64, sweep_angle: f64) -> CircleSegment {
        CircleSegment( self.0.segment(
            inner_radius,
            start_angle,
            sweep_angle,
        ) )
    }

    /// Is this circle finite?
    #[inline]
    pub fn is_finite(&self) -> bool {
        self.0.is_finite()
    }

    /// Is this circle NaN?
    #[inline]
    pub fn is_nan(&self) -> bool {
        self.0.is_nan()
    }
}

#[derive(Any)]
pub struct CircleSegment(kurbo::CircleSegment);

impl CircleSegment {
    //pub fn new(center: impl Into<Point>,outer_radius: f64,inner_radius: f64,start_angle: f64,sweep_angle: f64,) -> Self {
    pub fn new(center: Value,outer_radius: f64,inner_radius: f64,start_angle: f64,sweep_angle: f64,) -> Self {

        CircleSegment( kurbo::CircleSegment::new(
            Point::from(center).to_native(),
            outer_radius,
            inner_radius,
            start_angle,
            sweep_angle,
        ) )
    }

    #[inline]
    pub fn is_finite(&self) -> bool {
        self.0.is_finite()
    }

    #[inline]
    pub fn is_nan(&self) -> bool {
        self.0.is_nan()
    }
}

#[derive(Any, Clone, Copy, Default, Debug, PartialEq)]
pub struct Ellipse(pub kurbo::Ellipse);

impl Into<kurbo::Ellipse> for Ellipse {
    fn into(self) -> kurbo::Ellipse {
        self.0
    }
}

impl Ellipse {
    //pub fn new(center: impl Into<Point>, radii: impl Into<Vec2>, x_rotation: f64) -> Ellipse {
    pub fn new(center: Value, radii: Value, x_rotation: f64) -> Ellipse {
        Self( kurbo::Ellipse::new( Point::from(center).to_native(), Vec2::from(radii).to_native(), x_rotation) )
    }

    #[inline]
    pub fn from_rect(rect: &Rect) -> Self {
        Self( kurbo::Ellipse::from_rect(rect.to_native() ) )
    }

    #[inline]
    pub fn from_affine(affine: &Affine) -> Self {
        Self( kurbo::Ellipse::from_affine(affine.to_native()) )
    }

    #[inline]
    #[must_use]
    //pub fn with_center(self, new_center: Point) -> Ellipse {
    pub fn with_center(&self, new_center: &Point) -> Ellipse {
        Self(self.0.with_center(new_center.to_native()))
    }

    #[must_use]
    //pub fn with_radii(self, new_radii: Vec2) -> Ellipse {
    pub fn with_radii(&self, new_radii: &Vec2) -> Ellipse {
        Self(self.0.with_radii(new_radii.to_native() ))
    }

    #[must_use]
    //pub fn with_rotation(self, rotation: f64) -> Ellipse {
    pub fn with_rotation(&self, rotation: f64) -> Ellipse {
        Self(self.0.with_rotation(rotation))
    }

    #[inline]
    pub fn center(&self) -> Point {
        Point(self.0.center())
    }

    pub fn radii(&self) -> Vec2 {
        Vec2(self.0.radii())
    }

    pub fn rotation(&self) -> f64 {
        self.0.rotation()
    }

    #[inline]
    pub fn is_finite(&self) -> bool {
        self.0.is_finite()
    }

    #[inline]
    pub fn is_nan(&self) -> bool {
        self.0.is_nan()
    }
}


#[derive(Any, Copy, Clone, Default, PartialEq)]
pub struct Rect(pub kurbo::Rect);

impl Rect {
    pub fn to_native(&self) -> kurbo::Rect {
        self.clone().into()
    }

    pub const fn new(x0:f64, y0:f64, x1:f64, y1:f64) -> Self {
        Self( druid::kurbo::Rect::new( x0, y0, x1, y1 ) )
    }

    #[inline]
    //pub fn from_points(p0: impl Into<Point>, p1: impl Into<Point>) -> Rect {
    pub fn from_points(p0: Value, p1: Value) -> Rect {
        Rect( kurbo::Rect::from_points( Point::from(p0).to_native(), Point::from(p1).to_native() ) )
    }

    #[inline]
    //pub fn from_origin_size(origin: impl Into<Point>, size: impl Into<Size>) -> Rect {
    pub fn from_origin_size(origin: Value, size: Value) -> Rect {
        Rect( kurbo::Rect::from_origin_size( Point::from(origin).to_native(), Size::from(size).to_native() ) )
    }

    #[inline]
    //pub fn from_center_size(center: impl Into<Point>, size: impl Into<Size>) -> Rect {
    pub fn from_center_size(center: Value, size: Value) -> Rect {
        Rect( kurbo::Rect::from_center_size( Point::from(center).to_native(), Size::from(size).to_native() ) )
    }

    #[inline]
    //pub fn with_origin(self, origin: impl Into<Point>) -> Rect {
    pub fn with_origin(self, origin: Value) -> Rect {
        Rect( self.0.with_origin( Point::from(origin).to_native() ) )
    }

    #[inline]
    //pub fn with_size(self, size: impl Into<Size>) -> Rect {
    pub fn with_size(self, size: Value) -> Rect {
        Rect( self.0.with_size( Size::from(size).to_native() ) )
    }

    #[inline]
    //pub fn inset(self, insets: impl Into<Insets>) -> Rect {
    pub fn inset(&self, insets: Value) -> Rect {
        Rect( self.0.inset( Insets::from(insets).to_native() ) )
    }

    #[inline]
    pub fn width(&self) -> f64 {
        self.0.width()
    }

    #[inline]
    pub fn height(&self) -> f64 {
        self.0.height()
    }

    #[inline]
    pub fn min_x(&self) -> f64 {
        self.0.min_x()
    }

    #[inline]
    pub fn max_x(&self) -> f64 {
        self.0.min_y()
    }

    #[inline]
    pub fn min_y(&self) -> f64 {
        self.0.min_y()
    }

    #[inline]
    pub fn max_y(&self) -> f64 {
        self.0.max_y()
    }

    #[inline]
    pub fn origin(&self) -> Point {
        Point(self.0.origin())
    }

    #[inline]
    pub fn size(&self) -> Size {
        Size::new(self.width(), self.height())
    }

    #[inline]
    pub fn area(&self) -> f64 {
        self.0.area()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    #[inline]
    pub fn center(&self) -> Point {
        Point( self.0.center() )
    }

    #[inline]
    pub fn contains(&self, point: Point) -> bool {
        self.0.contains( point.into() )
    }

    #[inline]
    pub fn abs(&self) -> Rect {
        Rect( self.0.abs() )
    }

    #[inline]
    pub fn union(&self, other: Rect) -> Rect {
        Rect( self.0.union(other.into()) )
    }

    pub fn union_pt(&self, pt: Point) -> Rect {
        Rect( self.0.union_pt(pt.into()) )
    }

    #[inline]
    pub fn intersect(&self, other: Rect) -> Rect {
        Rect( self.0.intersect(other.into()) )
    }

    pub fn inflate(&self, width: f64, height: f64) -> Rect {
        Rect( self.0.inflate(width,height) )
    }

    #[inline]
    pub fn round(self) -> Rect {
        Rect( self.0.round() )
    }

    #[inline]
    pub fn ceil(self) -> Rect {
        Rect( self.0.ceil() )
    }

    #[inline]
    pub fn floor(self) -> Rect {
        Rect( self.0.floor() )
    }

    #[inline]
    pub fn expand(self) -> Rect {
        Rect(self.0.expand())
    }

    #[inline]
    pub fn trunc(self) -> Rect {
        Rect(self.0.trunc())
    }

    #[inline]
    pub fn scale_from_origin(&self, factor: f64) -> Rect {
        Rect(self.0.scale_from_origin(factor))
    }

    #[inline]
    //pub fn to_rounded_rect(self, radii: impl Into<RoundedRectRadii>) -> RoundedRect {
    pub fn to_rounded_rect(&self, radii: Value) -> RoundedRect {
        RoundedRect( self.0.to_rounded_rect( RoundedRectRadii::from(radii).to_native() ) )
    }

    #[inline]
    //pub fn to_ellipse(self) -> Ellipse {
    pub fn to_ellipse(&self) -> Ellipse {
        Ellipse::from_rect(&self)
    }

    #[inline]
    pub fn aspect_ratio(&self) -> f64 {
        self.0.aspect_ratio()
    }

    pub fn contained_rect_with_aspect_ratio(&self, aspect_ratio: f64) -> Rect {
        Rect(self.0.contained_rect_with_aspect_ratio(aspect_ratio))
    }

    #[inline]
    pub fn is_finite(&self) -> bool {
        self.0.is_finite()
    }

    #[inline]
    pub fn is_nan(&self) -> bool {
        self.0.is_nan()
    }
}
impl Into<druid::kurbo::Rect> for Rect {
    fn into(self) -> druid::kurbo::Rect {
        self.0
    }
}

#[derive(Any,Debug,Clone, Copy, Default, PartialEq)]
pub struct RoundedRectRadii(kurbo::RoundedRectRadii);

impl Into<kurbo::RoundedRectRadii> for RoundedRectRadii {
    fn into(self) -> kurbo::RoundedRectRadii {
        self.0
    }
}

impl RoundedRectRadii {
    pub const fn new(top_left: f64, top_right: f64, bottom_right: f64, bottom_left: f64) -> Self {
        RoundedRectRadii( kurbo::RoundedRectRadii::new(top_left,top_right,bottom_right,bottom_left) )
    }

    pub const fn from_single_radius(radius: f64) -> Self {
        RoundedRectRadii( kurbo::RoundedRectRadii::from_single_radius(radius) )
    }

    pub fn abs(&self) -> Self {
        RoundedRectRadii( self.0.abs() )
    }

    pub fn clamp(&self, max: f64) -> Self {
        RoundedRectRadii( self.0.clamp(max) )
    }

    pub fn is_finite(&self) -> bool {
        self.0.is_finite()
    }

    pub fn is_nan(&self) -> bool {
        self.0.is_nan()
    }

    pub fn as_single_radius(&self) -> Option<f64> {
        self.0.as_single_radius()
    }

    pub fn to_native(&self) -> kurbo::RoundedRectRadii {
        self.0.clone()
    }
}

impl From<Value> for RoundedRectRadii {
    fn from(v: Value) -> Self {
        match v {
            Value::Float(v) => Self::from_single_radius(v),
            Value::Tuple(_) => {
                if let Some( (a,b,c,d) ) = v.resolve_tuple4::<f64>() {
                    Self::new( a,b,c,d )
                } else {
                    panic!("invalid insets value")
                }
            }
            Value::Any(_) => v.resolve_as_copy::<Self>().unwrap(),
            _ => panic!("invalid insets value")
        }
    }
}



#[derive(Clone,Copy,Any,Debug)]
pub struct RoundedRect( kurbo::RoundedRect );

impl Into<kurbo::RoundedRect> for RoundedRect {
    fn into(self) -> kurbo::RoundedRect {
        self.0
    }
}

impl RoundedRect {
    pub fn to_native(self) -> kurbo::RoundedRect {
        self.0
    }

    #[inline]
    //pub fn new( x0: f64, y0: f64, x1: f64, y1: f64, radii: impl Into<RoundedRectRadii>,
    pub fn new( x0: f64, y0: f64, x1: f64, y1: f64, radii: Value,
    ) -> RoundedRect {
        RoundedRect( kurbo::RoundedRect::new(x0,y0,x1,y1,RoundedRectRadii::from(radii).to_native() ) )
    }

    #[inline]
    //pub fn from_rect(rect: Rect, radii: impl Into<RoundedRectRadii>) -> RoundedRect {
    pub fn from_rect(rect: &Rect, radii: Value) -> RoundedRect {
        RoundedRect( kurbo::RoundedRect::from_rect(rect.to_native(), RoundedRectRadii::from(radii)) )
    }

    #[inline]
    // pub fn from_points(p0: impl Into<Point>, p1: impl Into<Point>, radii: impl Into<RoundedRectRadii>,) -> RoundedRect {
    pub fn from_points(p0: Value, p1: Value, radii: Value) -> RoundedRect {
        RoundedRect( kurbo::RoundedRect::from_points( Point::from(p0),Point::from(p1),RoundedRectRadii::from(radii) ) )
    }

    #[inline]
    //pub fn from_origin_size(origin: impl Into<Point>, size: impl Into<Size>, radii: impl Into<RoundedRectRadii>,) -> RoundedRect {
    pub fn from_origin_size(origin: Value, size: Value, radii: Value,) -> RoundedRect {
        RoundedRect( kurbo::RoundedRect::from_origin_size(Point::from(origin), Size::from(size), RoundedRectRadii::from(radii) ) )
    }

    #[inline]
    pub fn width(&self) -> f64 {
        self.0.width()
    }

    #[inline]
    pub fn height(&self) -> f64 {
        self.0.height()
    }

    #[inline]
    pub fn radii(&self) -> RoundedRectRadii {
        RoundedRectRadii(self.0.radii())
    }

    pub fn rect(&self) -> Rect {
        Rect(self.0.rect())
    }

    #[inline]
    pub fn origin(&self) -> Point {
        Point(self.0.origin())
    }

    #[inline]
    pub fn center(&self) -> Point {
        Point(self.0.center())
    }

    #[inline]
    pub fn is_finite(&self) -> bool {
        self.0.is_finite()
    }

    #[inline]
    pub fn is_nan(&self) -> bool {
        self.0.is_nan()
    }
}

#[derive(Any,Debug)]
pub enum InterpolationMode {
    #[rune(constructor)] NearestNeighbor,
    #[rune(constructor)] Bilinear,
}

impl Into<piet::InterpolationMode > for InterpolationMode {
    fn into(self) -> piet::InterpolationMode {
        match self {
            InterpolationMode::NearestNeighbor => piet::InterpolationMode::NearestNeighbor,
            InterpolationMode::Bilinear => piet::InterpolationMode::Bilinear
        }
    }
}

#[derive(Any,Debug,Clone,Copy,PartialEq)]
pub enum ImageFormat {
    #[rune(constructor)] Grayscale,
    #[rune(constructor)] Rgb,
    #[rune(constructor)] RgbaSeparate,
    #[rune(constructor)] RgbaPremul,
}

impl Into<piet::ImageFormat > for ImageFormat {
    fn into(self) -> piet::ImageFormat {
        match self {
            ImageFormat::Grayscale => piet::ImageFormat::Grayscale,
            ImageFormat::Rgb => piet::ImageFormat::Rgb,
            ImageFormat::RgbaSeparate => piet::ImageFormat::RgbaSeparate,
            ImageFormat::RgbaPremul => piet::ImageFormat::RgbaPremul,
        }
    }
}

impl ImageFormat {
    pub fn bytes_per_pixel(&self) -> usize {
        Into::<piet::ImageFormat>::into( self.clone() ).bytes_per_pixel()
    }
}

#[derive(Any, Clone, Debug, PartialEq)]
pub struct QuadSpline(kurbo::QuadSpline);

impl Into<kurbo::QuadSpline> for QuadSpline {
    fn into(self) -> kurbo::QuadSpline {
        self.0
    }
}

impl QuadSpline {
    #[inline]
    //pub fn new(points: Vec<Point>) -> Self {
    pub fn new(points: Value) -> Self {
        if let Value::Vec(v) = points {
            let points = v.borrow_ref().unwrap().iter().map( |v| if let Value::Any(v) = v {
                v.downcast_borrow_ref::<Point>().unwrap().clone().into()
            } else {
                panic!("invalid Point type")
            }).collect();
            Self( kurbo::QuadSpline::new(points) )
        } else {
            panic!("invalid Point type")
        }
    }

    #[inline]
   //pub fn points(&self) -> &[Point] {
    pub fn points(&self) -> Vec<Point> {
        self.0.points().into_iter().map( |v| Point(v.clone()) ).collect()
   }
}

#[derive(Any,Clone,Copy)]
pub struct CubicBez( kurbo::CubicBez );

impl Into<kurbo::CubicBez> for CubicBez {
    fn into(self) -> kurbo::CubicBez {
        self.0
    }
}

impl CubicBez {
    #[inline]
    //pub fn new<P: Into<Point>>(p0: P, p1: P, p2: P, p3: P) -> CubicBez {
    pub fn new(p0: Value, p1: Value, p2: Value, p3: Value) -> CubicBez {
        CubicBez( kurbo::CubicBez::new(Point::from(p0),Point::from(p1),Point::from(p2),Point::from(p3) ) )
    }

    #[inline]
    //pub fn to_quads(&self, accuracy: f64) -> impl Iterator<Item = (f64, f64, QuadBez)> {
    pub fn to_quads(&self, accuracy: f64) -> Vec<(f64, f64, QuadBez)> {
        self.0.to_quads(accuracy).map( |(a,b,c)| (a,b,QuadBez(c)) ).collect()
    }

    pub fn approx_spline(&self, accuracy: f64) -> Option<QuadSpline> {
        self.0.approx_spline(accuracy).map( |v| QuadSpline(v) )
    }

    #[inline]
    pub fn is_finite(&self) -> bool {
        self.0.is_finite()
    }

    #[inline]
    pub fn is_nan(&self) -> bool {
        self.0.is_nan()
    }
}


#[derive(Any,Clone,Copy)]
pub struct QuadBez( kurbo::QuadBez );

impl Into<kurbo::QuadBez> for QuadBez {
    fn into(self) -> kurbo::QuadBez {
        self.0
    }
}

impl QuadBez {
    /// Create a new quadratic Bézier segment.
    #[inline]
    //pub fn new<V: Into<Point>>(p0: V, p1: V, p2: V) -> QuadBez {
    pub fn new(p0: Value, p1: Value, p2: Value) -> QuadBez {
        QuadBez( kurbo::QuadBez::new(Point::from(p0),Point::from(p1),Point::from(p2)) )
    }

    #[inline]
    pub fn raise(&self) -> CubicBez {
        CubicBez(
            self.0.raise()
        )
    }

    #[inline]
    pub fn is_finite(&self) -> bool {
        self.0.is_finite()
    }

    /// Is this quadratic Bezier curve NaN?
    #[inline]
    pub fn is_nan(&self) -> bool {
        self.0.is_nan()
    }
}

#[derive(Any,Clone, Copy, Debug, PartialEq)]
pub struct PathSeg(kurbo::PathSeg);

impl Into<kurbo::PathSeg> for PathSeg {
    fn into(self) -> kurbo::PathSeg {
        self.0
    }
}

impl PathSeg {
    pub fn Line(v:Line) -> Self {
        PathSeg(kurbo::PathSeg::Line(v.into()))
    }
    pub fn Quad(v:QuadBez) -> Self {
        PathSeg(kurbo::PathSeg::Quad(v.into()))
    }
    pub fn Cubic(v:CubicBez) -> Self {
        PathSeg(kurbo::PathSeg::Cubic(v.into()))
    }
}