use std::any::{TypeId};
use std::fmt::Formatter;
use std::marker::PhantomData;
use druid::{Lens, UnitPoint, WidgetExt};
use rune::{Any, ToValue, Value, FromValue, Hash};
use rune::compile::{Named};
use rune::runtime::{Args,Function};
use std::str::FromStr;
use druid::widget::SvgData;
use super::types::{Insets, Color, Rect, BackgroundBrush, RoundedRectRadii};
use super::render::PaintCtx;
//use super::click::Click;

macro_rules! gen_wrapper_ext {
    ( $T:ident ) => {
            /// Wrap this widget in a [`Padding`] widget with the given [`Insets`].
            ///
            /// Like [`Padding::new`], this can accept a variety of arguments, including
            /// a [`Key`] referring to [`Insets`] in the [`Env`].
            ///
            /// [`Key`]: crate::Key
            /// [`Insets`]: crate::Insets
            //fn padding(self, insets: impl Into<Insets>) -> Padding<T> {
            pub fn padding(self, v:Value) -> Padding<T> {
                Padding::<$T>::new( Insets::from(v), self.origin)
            }

            /// Wrap this widget in an [`Align`] widget, configured to center it.
            ///
            /// [`Align`]: widget/struct.Align.html
            pub fn center(self) -> Align<$T> {
                Align::centered(self.origin)
            }


            /// Wrap this widget in an [`Align`] widget, configured to align left.
            ///
            /// [`Align`]: widget/struct.Align.html
            pub fn align_left(self) -> Align<$T> {
                Align::left(self.origin)
            }

            /// Wrap this widget in an [`Align`] widget, configured to align right.
            ///
            /// [`Align`]: widget/struct.Align.html
            pub fn align_right(self) -> Align<$T> {
                Align::right(self.origin)
            }

            /// Wrap this widget in an [`Align`] widget, configured to align vertically.
            ///
            /// [`Align`]: widget/struct.Align.html
            pub fn align_vertical(self, align:(f64,f64) ) -> Align<$T> {
                Align::vertical( UnitPoint::new(align.0,align.1), self.origin )
            }

            /// Wrap this widget in an [`Align`] widget, configured to align horizontally.
            ///
            /// [`Align`]: widget/struct.Align.html
            pub fn align_horizontal(self, align:(f64,f64) ) -> Align<$T> {
                Align::horizontal( UnitPoint::new(align.0,align.1), self.origin )
            }

            /// Wrap this widget in a [`SizedBox`] with an explicit width.
            ///
            /// [`SizedBox`]: widget/struct.SizedBox.html
            pub fn fix_width(self, width: f64) -> SizedBox<$T> {
                SizedBox::new(self.origin).width(width)
            }

            /// Wrap this widget in a [`SizedBox`] with an explicit height.
            ///
            /// [`SizedBox`]: widget/struct.SizedBox.html
            pub fn fix_height(self, height: f64) -> SizedBox<$T> {
                SizedBox::new(self.origin).height(height)
            }

            /// Wrap this widget in an [`SizedBox`] with an explicit width and height
            ///
            /// [`SizedBox`]: widget/struct.SizedBox.html
            pub fn fix_size(
                self,
                width: f64,
                height: f64,
            ) -> SizedBox<$T> {
                SizedBox::new(self.origin).width(width).height(height)
            }

            /// Wrap this widget in a [`SizedBox`] with an infinite width and height.
            ///
            /// Only call this method if you want your widget to occupy all available
            /// space. If you only care about expanding in one of width or height, use
            /// [`expand_width`] or [`expand_height`] instead.
            ///
            /// [`expand_height`]: #method.expand_height
            /// [`expand_width`]: #method.expand_width
            /// [`SizedBox`]: widget/struct.SizedBox.html
            pub fn expand(self) -> SizedBox<$T> {
                SizedBox::new(self.origin)._expand()
            }

            /// Wrap this widget in a [`SizedBox`] with an infinite width.
            ///
            /// This will force the child to use all available space on the x-axis.
            ///
            /// [`SizedBox`]: widget/struct.SizedBox.html
            pub fn expand_width(self) -> SizedBox<$T> {
                SizedBox::new(self.origin)._expand_width()
            }

            /// Wrap this widget in a [`SizedBox`] with an infinite width.
            ///
            /// This will force the child to use all available space on the y-axis.
            ///
            /// [`SizedBox`]: widget/struct.SizedBox.html
            pub fn expand_height(self) -> SizedBox<$T> {
                SizedBox::new(self.origin)._expand_height()
            }

            /// Wrap this widget in a [`Container`] with the provided `background`.
            ///
            /// See [`Container::background`] for more information.
            ///
            /// [`Container`]: widget/struct.Container.html
            /// [`Container::background`]: widget/struct.Container.html#method.background
            //pub fn background(self, brush: BackgroundBrush<$T>) -> Container<$T> {
            pub fn background(self, v:Value) -> Container<$T> {
                Container::new( self.origin )._background( BackgroundBrush::<$T>::from(v) )
            }

            /// Wrap this widget in a [`Container`] with the given border.
            ///
            /// Arguments can be either concrete values, or a [`Key`] of the respective
            /// type.
            ///
            /// [`Container`]: widget/struct.Container.html
            /// [`Key`]: struct.Key.html

            //fn border(self, color: impl Into<KeyOrValue<Color>>, width: impl Into<KeyOrValue<f64>>, ) -> Container<T> {
            pub fn border(self, color: Value, width: f64, ) -> Container<$T> {
                Container::new(self.origin)._border(Color::from(color), width)
            }

            //EnvScope need static function(not dynamic)
            /// Wrap this widget in a [`EnvScope`] widget, modifying the parent
            /// [`Env`] with the provided closure.
            ///
            /// [`EnvScope`]: widget/struct.EnvScope.html
            /// [`Env`]: struct.Env.html
            //fn env_scope(self, f: impl Fn(&mut Env, &T) + 'static) -> EnvScope<T, Self> {
            pub fn env_scope(self, f: Function) -> EnvScope<T> {
                EnvScope::new( Box::new( move |env,t| {
                    //wrapping Env
                    let wenv = Env::unsafe_wrap( env );
                    let _:() = f.call( (wenv,) ).unwrap();
                }) , self.origin)
            }

        // /// Wrap this widget with the provided [`Controller`].
        // ///
        // /// [`Controller`]: widget/trait.Controller.html
        // fn controller<C: Controller<T, Self>>(self, controller: C) -> ControllerHost<Self, C> {
        //     ControllerHost::new(self, controller)
        // }
        //
        // /// Provide a closure that will be called when this widget is added to the widget tree.
        // ///
        // /// You can use this to perform any initial setup.
        // ///
        // /// This is equivalent to handling the [`LifeCycle::WidgetAdded`] event in a
        // /// custom [`Controller`].
        // ///
        // /// [`LifeCycle::WidgetAdded`]: crate::LifeCycle::WidgetAdded
        // fn on_added(
        //     self,
        //     f: impl Fn(&mut Self, &mut LifeCycleCtx, &T, &Env) + 'static,
        // ) -> ControllerHost<Self, Added<T, Self>> {
        //     ControllerHost::new(self, Added::new(f))
        // }
        //
            /// Control the events of this widget with a [`Click`] widget. The closure
            /// provided will be called when the widget is clicked with the left mouse
            /// button.
            ///
            /// The child widget will also be updated on [`LifeCycle::HotChanged`] and
            /// mouse down, which can be useful for painting based on `ctx.is_active()`
            /// and `ctx.is_hot()`.
            ///
            /// [`Click`]: widget/struct.Click.html
            /// [`LifeCycle::HotChanged`]: enum.LifeCycle.html#variant.HotChanged
            //fn on_click(self,f: impl Fn(&mut EventCtx, &mut T, &Env) + 'static,) -> ControllerHost<Self, Click<$T>> {
            pub fn on_click(self,f: Function) -> ControllerHost<$T> {
                let fb = move |ctx:&mut druid::EventCtx, t:&mut $T, env:&druid::Env| {
                    let _:() = f.call( () ).unwrap();
                };
                ControllerHost::new(self.origin, druid::widget::Click::new(fb))
            }
        //
        // /// Draw the [`layout`] `Rect`s of  this widget and its children.
        // ///
        // /// [`layout`]: trait.Widget.html#tymethod.layout
        // fn debug_paint_layout(self) -> EnvScope<T, Self> {
        //     EnvScope::new(|env, _| env.set(Env::DEBUG_PAINT, true), self)
        // }
        //
        // /// Display the `WidgetId`s for this widget and its children, when hot.
        // ///
        // /// When this is `true`, widgets that are `hot` (are under the mouse cursor)
        // /// will display their ids in their bottom right corner.
        // ///
        // /// These ids may overlap; in this case the id of a child will obscure
        // /// the id of its parent.
        // fn debug_widget_id(self) -> EnvScope<T, Self> {
        //     EnvScope::new(|env, _| env.set(Env::DEBUG_WIDGET_ID, true), self)
        // }
        //
        // /// Draw a color-changing rectangle over this widget, allowing you to see the
        // /// invalidation regions.
        // fn debug_invalidation(self) -> DebugInvalidation<T, Self> {
        //     DebugInvalidation::new(self)
        // }
        //
        // /// Set the [`DEBUG_WIDGET`] env variable for this widget (and its descendants).
        // ///
        // /// This does nothing by default, but you can use this variable while
        // /// debugging to only print messages from particular instances of a widget.
        // ///
        // /// [`DEBUG_WIDGET`]: struct.Env.html#associatedconstant.DEBUG_WIDGET
        // fn debug_widget(self) -> EnvScope<T, Self> {
        //     EnvScope::new(|env, _| env.set(Env::DEBUG_WIDGET, true), self)
        // }
        //
        // /// Wrap this widget in a [`LensWrap`] widget for the provided [`Lens`].
        // fn lens<S: Data, L: Lens<S, T>>(self, lens: L) -> LensWrap<S, T, L, Self> {
        //     LensWrap::new(self, lens)
        // }
        //
        // /// Parse a `Widget<String>`'s contents
        // #[deprecated(since = "0.7.0", note = "Use TextBox::with_formatter instead")]
        // fn parse(self) -> Parse<Self>
        // where
        //     Self: Widget<String>,
        // {
        //     Parse::new(self)
        // }
        //
        // /// Assign the widget a specific [`WidgetId`].
        // ///
        // /// You must ensure that a given [`WidgetId`] is only ever used for
        // /// a single widget at a time.
        // ///
        // /// An id _may_ be reused over time; for instance if you replace one
        // /// widget with another, you may reuse the first widget's id.
        // ///
        // /// [`WidgetId`]: struct.WidgetId.html
        // fn with_id(self, id: WidgetId) -> IdentityWrapper<Self> {
        //     IdentityWrapper::wrap(self, id)
        // }
        //

        // /// Wrap this widget in a `Box`.   ----- Can't support this
        // fn boxed(self) -> Box<dyn druid::Widget<$T>> {
        //     Box::new(self)
        // }
        //
        /// Wrap this widget in a [`Scroll`] widget.
        ///
        /// [`Scroll`]: widget/struct.Scroll.html
        pub fn scroll(self) -> Scroll<$T> {
            Scroll::new(self.origin)
        }
        //
        // /// Wrap this widget in a [`DisabledIf`] widget.
        // ///
        // /// The provided closure will determine if the widget is disabled.
        // /// See [`is_disabled`] or [`set_disabled`] for more info about disabled state.
        // ///
        // /// [`is_disabled`]: crate::EventCtx::is_disabled
        // /// [`set_disabled`]: crate::EventCtx::set_disabled
        // /// [`DisabledIf`]: crate::widget::DisabledIf
        // fn disabled_if(self, disabled_if: impl Fn(&T, &Env) -> bool + 'static) -> DisabledIf<T, Self> {
        //     DisabledIf::new(self, disabled_if)
        // }
    }
}

macro_rules! gen_wrapper {
    ( $origin:ident ) => {
        #[derive(Any)]
        pub struct $origin<T:'static + Named + druid::Data> {
            #[allow(unused)]
            pub origin : druid::widget::$origin<T>
        }

        ///WidgetExt functions
        impl <T:'static + Named + druid::Data> $origin<T> {
            gen_wrapper_ext! { T }
        }
    };
    ( $origin:ident boxed ) => {
        #[derive(Any)]
        pub struct $origin<T:'static + Named + druid::Data> {
            #[allow(unused)]
            pub origin : Box< dyn druid::widget::Widget<T> >,
        }

        ///WidgetExt functions
        impl <T:'static + Named + druid::Data> $origin<T> {
            gen_wrapper_ext! { T }
        }
    };

    ( $origin:ident $alter:ident ) => {
        #[derive(Any)]
        pub struct $origin<T:'static + Named + druid::Data> {
            #[allow(unused)]
            pub origin : druid::widget::$origin,
            p : PhantomData<T>
        }

        ///WidgetExt functions
        impl <T:'static + Named + druid::Data> $origin<T> {
            gen_wrapper_ext! { $alter }
        }
    };
}

#[derive(Any,Debug)]
pub struct Env {
    origin : *mut druid::Env
}


impl Env {
    fn unsafe_wrap(origin: &mut druid::Env) -> Self {
        Self { origin : origin as _ }
    }

    fn de(&self) -> &mut druid::Env {
        unsafe { &mut *self.origin as _ }
    }

    pub fn set(&mut self, key:&'static str, value:Value) {

        match value {
            Value::Float(v) => self.de().set( druid::Key::<f64>::new(key), v),
            Value::Tuple(v) => {
                self.de().set( druid::Key::<druid::Color>::new(key), Color::from( Value::Tuple(v) ).to_native() );
            }
            Value::Any(v) => {
                if let Ok(resolved) = v.downcast_borrow_ref::<Color>() {
                    self.de().set( druid::Key::<druid::Color>::new(key), resolved.clone().to_native() );
                } else if let Ok(resolved) = v.downcast_borrow_ref::<Insets>() {
                    self.de().set( druid::Key::<druid::Insets>::new(key), resolved.clone().to_native() );
                } else if let Ok(resolved) = v.downcast_borrow_ref::<RoundedRectRadii>() {
                    self.de().set( druid::Key::<druid::RoundedRectRadii>::new(key), resolved.clone().to_native() );
                } else {
                    panic!("unknwon env value : {:?}", v)
                }
            }
            _ => panic!("invalid env value : {:?}", value.type_info().unwrap())
        }
    }
}

#[derive(Any)]
pub struct EnvScope<T:'static + Named + druid::Data> {
    pub origin : super::env_scope::EnvScope<T, Box<dyn druid::widget::Widget<T>> >
}
impl <T:'static + Named + druid::Data> EnvScope<T> {
    //pub fn new(f: impl for<'a> Fn(&'a mut Env, &'a T) , child : impl druid::widget::Widget<T> + 'static) -> EnvScope<T> {
    pub fn new(f: Box<dyn Fn(&mut druid::Env,&T)>, child : impl druid::widget::Widget<T> + 'static) -> EnvScope<T> {
        //impl Fn(&mut druid::Env, &T)
        // let fb = |&mut env, &T| {
        //     //need env call;
        //     // let _:() = f.call( () ).unwrap();
        //     println!(".....");
        // };

        Self {
            origin : super::env_scope::EnvScope::new( f, Box::new(child) )
        }
    }

    gen_wrapper_ext!( T );
}

#[derive(Any)]
pub struct Scroll<T:'static + Named + druid::Data> {
    pub origin : druid::widget::Scroll<T, Box<dyn druid::widget::Widget<T>> >
}
impl <T:'static + Named + druid::Data> Scroll<T> {
    pub fn new(child : impl druid::widget::Widget<T> + 'static) -> Scroll<T> {
        Self {
            origin : druid::widget::Scroll::new( Box::new(child) )
        }
    }

    gen_wrapper_ext!( T );
}

#[derive(Any)]
pub struct Padding<T:'static + Named + druid::Data> {
    #[allow(unused)]
    pub origin : druid::widget::Padding<T, Box<dyn druid::widget::Widget<T>> >
}
impl <T:'static + Named + druid::Data> Padding<T> {
    pub fn new(insets:Insets, org : impl druid::widget::Widget<T> + 'static) -> Padding<T> {
        Self { origin : druid::widget::Padding::<T,_>::new(insets.to_native(), Box::new(org) ) }
    }

    gen_wrapper_ext! { T }
}

gen_wrapper!( Align );
impl <T:'static + Named + druid::Data> Align<T> {
    pub fn centered(org : impl druid::widget::Widget<T> + 'static) -> Align<T> {
        Self { origin : druid::widget::Align::centered(org) }
    }

    pub fn left(org : impl druid::widget::Widget<T> + 'static) -> Align<T> {
        Self { origin : druid::widget::Align::left(org) }
    }

    pub fn right(org : impl druid::widget::Widget<T> + 'static) -> Align<T> {
        Self { origin : druid::widget::Align::right(org) }
    }

    pub fn vertical(align:UnitPoint, org : impl druid::widget::Widget<T> + 'static) -> Align<T> {
        Self { origin : druid::widget::Align::vertical(align,org) }
    }

    pub fn horizontal(align:UnitPoint, org : impl druid::widget::Widget<T> + 'static) -> Align<T> {
        Self { origin : druid::widget::Align::horizontal(align,org) }
    }
}

gen_wrapper!( SizedBox );
impl <T:'static + Named + druid::Data> SizedBox<T> {
    pub fn new(w: impl druid::widget::Widget<T> + 'static) -> Self {
        Self { origin : druid::widget::SizedBox::new(w) }
    }

    pub fn width(mut self, w:f64) -> SizedBox<T> {
        self.origin = self.origin.width(w);
        self
    }

    pub fn height(mut self, h:f64) -> SizedBox<T> {
        self.origin = self.origin.height(h);
        self
    }

    pub fn _expand(mut self) -> SizedBox<T> {
        self.origin = self.origin.expand();
        self
    }

    pub fn _expand_width(mut self) -> SizedBox<T> {
        self.origin = self.origin.expand_width();
        self
    }

    pub fn _expand_height(mut self) -> SizedBox<T> {
        self.origin = self.origin.expand_height();
        self
    }
}


gen_wrapper!( Container );
impl <T:'static + Named + druid::Data> Container<T> {
    pub fn new(w: impl druid::widget::Widget<T> + 'static) -> Self {
        Self { origin : druid::widget::Container::new(w) }
    }

    // pub fn _background(mut self, w:BackgroundBrush<T>) -> Self {
    pub fn _background(mut self, w:BackgroundBrush<T>) -> Self {
        self.origin.set_background( w.to_native() );
        self
    }

    pub fn _border(mut self, c:Color, width:f64) -> Self {
        self.origin.set_border(c.to_native(), width);
        self
    }
}

#[derive(Debug,Any)]
pub enum CrossAxisAlignment {
    #[rune(constructor)]
    Start,
    #[rune(constructor)]
    Center,
    #[rune(constructor)]
    End,
    #[rune(constructor)]
    Baseline
}

impl CrossAxisAlignment {
    fn to_native(self) -> druid::widget::CrossAxisAlignment {
        match self {
            CrossAxisAlignment::Start => druid::widget::CrossAxisAlignment::Start,
            CrossAxisAlignment::Center => druid::widget::CrossAxisAlignment::Center,
            CrossAxisAlignment::End => druid::widget::CrossAxisAlignment::End,
            CrossAxisAlignment::Baseline => druid::widget::CrossAxisAlignment::Baseline,
        }
    }
}

#[derive(Any,Clone)]
pub struct FlexParams {
    origin : druid::widget::FlexParams
}

impl std::fmt::Debug for FlexParams {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"FlexParams")
    }
}

impl FlexParams {
    pub fn new(flex:f64, alignment:Option<CrossAxisAlignment>) -> Self {
        Self {
            origin : druid::widget::FlexParams::new(flex, alignment.map(|v| v.to_native()) )
        }
    }

    pub fn to_native(&self) -> druid::widget::FlexParams {
        self.origin
    }
}

impl From<Value> for FlexParams {
    fn from(v: Value) -> Self {
        match v {
            Value::Float(v) => Self::new(v,None),
            Value::Any(v) => v.downcast_borrow_ref::<Self>().unwrap().clone(),
            _ => panic!("invalid FlexParams value : {:?}", v)
        }
    }
}

#[derive(Debug,Any)]
pub enum MainAxisAlignment {
    #[rune(constructor)]
    Start,
    #[rune(constructor)]
    Center,
    #[rune(constructor)]
    End,
    #[rune(constructor)]
    SpaceBetween,
    #[rune(constructor)]
    SpaceEvenly,
    #[rune(constructor)]
    SpaceAround,
}

impl MainAxisAlignment {
    pub fn to_native(self) -> druid::widget::MainAxisAlignment {
        match self {
            Self::Start => druid::widget::MainAxisAlignment::Start,
            Self::Center => druid::widget::MainAxisAlignment::Center,
            Self::End => druid::widget::MainAxisAlignment::End,
            Self::SpaceBetween => druid::widget::MainAxisAlignment::SpaceBetween,
            Self::SpaceEvenly => druid::widget::MainAxisAlignment::SpaceEvenly,
            Self::SpaceAround => druid::widget::MainAxisAlignment::SpaceAround,
        }
    }
}

gen_wrapper!( Flex );

macro_rules! type_resolve_re {
    ( $_self:ident, $v:ident, $TT:ty, $fnc:ident $(,$param:ident)? ) => {
        type_resolve!($_self, $v, $TT, $fnc $(,$param)?
, {ControllerHost<$TT> Padding<$TT> Align<$TT> SizedBox<$TT> Container<$TT> Flex<$TT> Label<$TT>
Button<$TT> Checkbox<$TT> Either<$TT> ProgressBar<$TT> List<$TT> RadioGroup<$TT> Slider<$TT>
Stepper<$TT> TextBox<$TT> Spinner<$TT> Switch<$TT> Stepper<$TT> Image<$TT> Svg<$TT> EnvScope<$TT> Split<$TT>
 } );
    }
}

macro_rules! type_resolve {
    ($_self:ident, $v:ident, $TT:ty, $fnc:ident) => {
        type_resolve_re!( $_self, $v, $TT, $fnc );
    };
    ($_self:ident, $v:ident, $TT:ty, $fnc:ident, $param:ident) => {
        type_resolve_re!( $_self, $v, $TT, $fnc, $param );
    };
    ($_self:ident, $v:ident, $TT:ty, add, { $($typs:ty)* } ) => { {
        let v_hash = $v.type_hash().unwrap();
        $( if v_hash == rune::Hash::from_type_id( TypeId::of::< $typs >() ) {
            $_self.origin.add_child( <$typs>::from_value($v).unwrap().origin );
        } ) else *
        else {
            panic!("Unknown widget : {}", $v.type_info().unwrap().to_string())
        }
    } };
    ($_self:ident, $v:ident, $TT:ty, add_flex, $param:ident, { $($typs:ty)* } ) => {
        let v_hash = $v.type_hash().unwrap();
        $( if v_hash == rune::Hash::from_type_id( TypeId::of::< $typs >() ) {
            $_self.origin.add_flex_child( <$typs>::from_value($v).unwrap().origin, $param.origin );
        } ) else *
        else {
            panic!("Unknown widget : {}", $v.type_info().unwrap().to_string())
        }
    };
    ($_self:ident, $v:ident, $TT:ty, boxed, { $($typs:ty)* } ) => { {
        let v_hash = $v.type_hash().unwrap();
        $( if v_hash == rune::Hash::from_type_id( TypeId::of::< $typs >() ) {
            Box::new( <$typs>::from_value($v).unwrap().origin )
        } ) else *
        else {
            panic!("Unknown widget : {}", $v.type_info().unwrap().to_string())
        }
    } };
}
impl <T:'static + Named + druid::Data> Flex<T> {
    pub fn new() -> Self {
        Self {
            origin : druid::widget::Flex::<T>::row()
        }
    }

    pub fn row() -> Self {
        Self {
            origin : druid::widget::Flex::<T>::row()
        }
    }

    pub fn column() -> Self {
        Self {
            origin : druid::widget::Flex::<T>::column()
        }
    }

    pub fn cross_axis_alignment(mut self, alignment: CrossAxisAlignment) -> Self {
        self.origin = self.origin.cross_axis_alignment( alignment.to_native() );
        self
    }

    pub fn main_axis_alignment(mut self, alignment: MainAxisAlignment) -> Self {
        self.origin = self.origin.main_axis_alignment(alignment.to_native() );
        self
    }

    pub fn must_fill_main_axis(mut self, fill: bool) -> Self {
        self.origin = self.origin.must_fill_main_axis(fill);
        self
    }

    pub fn with_child(mut self, v:Value) -> Self {
        self.add_child(v);
        self
    }

    pub fn with_flex_child(mut self, v:Value, params: Value ) -> Self {
        self.add_flex_child(v,params );
        self
    }

    pub fn with_default_spacer(mut self) -> Self {
        self.origin = self.origin.with_default_spacer();
        self
    }

    pub fn with_spacer(mut self, len:f64) -> Self {
        self.origin = self.origin.with_spacer(len);
        self
    }

    pub fn with_flex_spacer(mut self, flex: f64) -> Self {
        self.origin = self.origin.with_flex_spacer(flex);
        self
    }

    pub fn set_cross_axis_alignment(&mut self, alignment: CrossAxisAlignment) {
        self.origin.set_cross_axis_alignment(alignment.to_native());
    }

    pub fn set_main_axis_alignment(&mut self, alignment: MainAxisAlignment) {
        self.origin.set_main_axis_alignment(alignment.to_native());
    }

    pub fn set_must_fill_main_axis(&mut self, fill: bool) {
        self.origin.set_must_fill_main_axis(fill);
    }

    //rune script dosen't support override function
    //so get the wrapped AnyObj(Value)
    pub fn add_child(&mut self, v:Value) {
        type_resolve!(self , v, T, add);
    }

    pub fn add_flex_child(&mut self, v:Value, fp:Value) {
        let fp = FlexParams::from(fp);
        type_resolve!(self , v, T, add_flex, fp );
    }
}

#[derive(Debug,Any)]
pub enum LineBreaking {
    /// Lines are broken at word boundaries.
    #[rune(constructor)] WordWrap,
    /// Lines are truncated to the width of the label.
    #[rune(constructor)] Clip,
    /// Lines overflow the label.
    #[rune(constructor)] Overflow,
}

impl Into<druid::widget::LineBreaking> for LineBreaking {
    fn into(self) -> druid::widget::LineBreaking {
        match self {
            LineBreaking::WordWrap => druid::widget::LineBreaking::WordWrap,
            LineBreaking::Clip => druid::widget::LineBreaking::Clip,
            LineBreaking::Overflow => druid::widget::LineBreaking::Overflow,
        }
    }
}

#[derive(Debug,Any)]
pub enum TextAlignment {
    /// Text is aligned to the left edge in left-to-right scripts, and the
    /// right edge in right-to-left scripts.
    #[rune(constructor)] Start,
    /// Text is aligned to the right edge in left-to-right scripts, and the
    /// left edge in right-to-left scripts.
    #[rune(constructor)] End,
    /// Lines are centered in the available space.
    #[rune(constructor)] Center,
    /// Line width is increased to fill available space.
    ///
    /// This may be achieved through increases in word or character spacing,
    /// or through ligatures where available.
    #[rune(constructor)] Justified,
}

impl Into<druid::piet::TextAlignment> for TextAlignment {
    fn into(self) -> druid::TextAlignment {
        match self {
            TextAlignment::Start => druid::piet::TextAlignment::Start,
            TextAlignment::End => druid::piet::TextAlignment::End,
            TextAlignment::Center => druid::piet::TextAlignment::Center,
            TextAlignment::Justified => druid::piet::TextAlignment::Justified,
        }
    }
}

gen_wrapper!( Label );
impl <T: Named + druid::Data> Label<T> {
    pub fn new(name:&str) -> Self {
        Self {
            origin : druid::widget::Label::new(name)
        }
    }

    pub fn with_text_color(mut self, color: Color) -> Self {
        self.origin.set_text_color(color.to_native());
        self
    }

    pub fn with_text_size(mut self, size: f64) -> Self {
        self.origin.set_text_size(size);
        self
    }

    // pub fn with_font(mut self, font: impl Into<KeyOrValue<FontDescriptor>>) -> Self {
    //     self.set_font(font);
    //     self
    // }

    pub fn with_line_break_mode(mut self, mode: LineBreaking) -> Self {
        self.origin.set_line_break_mode( mode.into() );
        self
    }

    pub fn with_text_alignment(mut self, alignment: TextAlignment) -> Self {
        self.origin.set_text_alignment(alignment.into());
        self
    }

    pub fn set_text_color(&mut self, color: Color) {
        self.origin.set_text_color( color.to_native() );
    }

    pub fn set_text_size(&mut self, size: f64) {
        self.origin.set_text_size(size);
    }

    // pub fn set_font(&mut self, font: impl Into<KeyOrValue<FontDescriptor>>) {
    //     self.layout.set_font(font);
    // }

    pub fn set_line_break_mode(&mut self, mode: LineBreaking) {
        self.origin.set_line_break_mode(mode.into());
    }

    pub fn set_text_alignment(&mut self, alignment: TextAlignment) {
        self.origin.set_text_alignment(alignment.into());
    }

    // pub fn draw_at(&self, ctx: &mut PaintCtx, origin: impl Into<Point>) {
    //     self.layout.draw(ctx, origin)
    // }

    pub fn baseline_offset(&self) -> f64 {
        bool::BASE_NAME;
        self.origin.baseline_offset()
    }
}



gen_wrapper!( Button );
impl <T: Named + druid::Data> Button<T> {
    pub fn new(name:&str) -> Self {
        Self {
            origin : druid::widget::Button::new(name)
        }
    }

    // pub fn on_click(self, f:Function) -> ControllerHost<T> {
    //     let fb = move |ctx:&mut druid::EventCtx, t:&mut T, env:&druid::Env| {
    //         let _:() = f.call( () ).unwrap();
    //     };
    //     ControllerHost::new(self.origin, druid::widget::Click::new(fb) )
    // }
}

#[derive(Any)]
pub struct ControllerHost<T:'static + Named + druid::Data> {
    #[allow(unused)]
    pub origin : Box<dyn druid::widget::Widget<T>>,//druid::widget::ControllerHost<W,C>,
    p : PhantomData<T>
}

impl <T:'static + Named + druid::Data> ControllerHost<T> {
    pub fn new<W:'static + druid::widget::Widget<T>, C:'static + druid::widget::Controller<T, W>>(w:W, c:C) -> Self {
        Self {
            origin : Box::new( druid::widget::ControllerHost::new(w,c) ),
            p : PhantomData
        }
    }

    gen_wrapper_ext!{ T }
}


// I first see the `Lens` concept and it's ok.
// It's just looking good and it's actually annoying and get more complex. (I think magic code always does)

#[derive(Default)]
struct DummyLens<T,A> {
    o : PhantomData<T>,
    a : A
}

impl <T,A> DummyLens<T,A> {
    fn new(a : A) -> Self {
        Self { o : PhantomData, a : a }
    }
}

impl <T:druid::Data,U> druid::Lens<T, U> for DummyLens<T,U> {
    fn with<V, F: FnOnce(&U) -> V>(&self, data: &T, f: F) -> V {
        f(&self.a)
    }

    fn with_mut<V, F: FnOnce(&mut U) -> V>(&self, data: &mut T, f: F) -> V {
        #[allow(mutable_transmutes)]
        f( unsafe { std::mem::transmute::<&U,&mut U>(&self.a) } )
    }
}

gen_wrapper!( Checkbox boxed );
impl <T: Named + druid::Data> Checkbox<T> {
    pub fn new(name:&str) -> Self {
        Self {
            origin : Box::new( druid::widget::Checkbox::new(name).lens( DummyLens::<T,_>::new( false) ) )
        }
    }
}

gen_wrapper!( Either );
impl <T:'static + Named + druid::Data> Either<T> {
    pub fn new(f:Value, v1:Value, v2:Value) -> Self {
        let f = Function::from_value(f).unwrap();
        let dummy_self = ();
        let v1b:Box<dyn druid::Widget<T>> = type_resolve!( _dummy_self, v1, T, boxed );
        let v2b:Box<dyn druid::Widget<T>> = type_resolve!( _dummy_self, v2, T, boxed );
        let either = druid::widget::Either::new(  move |t,env| -> bool {
            f.call( () ).unwrap()
        }, v1b, v2b);
        Self {
            //we can't resolve two types as same type (It's actually possible, but that's pretty stupid code.)
            //so just wrap boxed dyn type
            origin : either
        }
    }
}

#[derive(Any, Clone, Copy, PartialEq)]
pub enum FillStrat {
    /// As large as posible without changing aspect ratio of image and all of image shown
    #[rune(constructor)]Contain,
    /// As large as posible with no dead space so that some of the image may be clipped
    #[rune(constructor)]Cover,
    /// Fill the widget with no dead space, aspect ratio of widget is used
    #[rune(constructor)]Fill,
    /// Fill the hight with the images aspect ratio, some of the image may be clipped
    #[rune(constructor)]FitHeight,
    /// Fill the width with the images aspect ratio, some of the image may be clipped
    #[rune(constructor)]FitWidth,
    /// Do not scale
    #[rune(constructor)]None,
    /// Scale down to fit but do not scale up
    #[rune(constructor)]ScaleDown,
}

impl Default for FillStrat {
    fn default() -> Self {
        FillStrat::Contain
    }
}

impl Into<druid::widget::FillStrat> for FillStrat {
    fn into(self) -> druid::widget::FillStrat {
        match self {
            FillStrat::Contain => druid::widget::FillStrat::Contain,
            FillStrat::Cover => druid::widget::FillStrat::Cover,
            FillStrat::Fill => druid::widget::FillStrat::Fill,
            FillStrat::FitHeight => druid::widget::FillStrat::FitHeight,
            FillStrat::FitWidth => druid::widget::FillStrat::FitWidth,
            FillStrat::None => druid::widget::FillStrat::None,
            FillStrat::ScaleDown => druid::widget::FillStrat::ScaleDown,
        }
    }
}

#[derive(Any, Clone, Copy, PartialEq)]
pub enum InterpolationMode {
    /// Don't interpolate, use nearest neighbor.
    #[rune(constructor)]NearestNeighbor,
    /// Use bilinear interpolation.
    #[rune(constructor)]Bilinear,
}

impl Into<druid::piet::InterpolationMode> for InterpolationMode {
    fn into(self) -> druid::piet::InterpolationMode {
        match self {
            InterpolationMode::NearestNeighbor => druid::piet::InterpolationMode::NearestNeighbor,
            InterpolationMode::Bilinear => druid::piet::InterpolationMode::Bilinear
        }
    }
}

#[derive(Any)]
pub struct Image<T:'static + Named + druid::Data> {
    #[allow(unused)]
    pub origin : druid::widget::Image,
    p : PhantomData<T>
}

///WidgetExt functions
impl <T:'static + Named + druid::Data> Image<T> {
    pub fn new(v:&str) -> Self {
        let buf = druid::ImageBuf::from_file(v).unwrap();
        Self {
            origin : druid::widget::Image::new(buf),
            p : PhantomData
        }
    }

    gen_wrapper_ext! { T }

    #[inline]
    pub fn fill_mode(mut self, mode: FillStrat) -> Self {
        self.origin = self.origin.fill_mode(mode.into());
        self
    }

    /// Modify the widget's fill strategy.
    #[inline]
    pub fn set_fill_mode(&mut self, newfil: FillStrat) {
        self.origin.set_fill_mode( newfil.into ());
    }

    /// Builder-style method for specifying the interpolation strategy.
    #[inline]
    pub fn interpolation_mode(mut self, interpolation: InterpolationMode) -> Self {
        self.origin = self.origin.interpolation_mode(interpolation.into());
        self
    }

    /// Modify the widget's interpolation mode.
    #[inline]
    pub fn set_interpolation_mode(&mut self, interpolation: InterpolationMode) {
        self.origin.set_interpolation_mode(interpolation.into());
    }

    /// Builder-style method for setting the area of the image that will be displayed.
    ///
    /// If `None`, then the whole image will be displayed.
    #[inline]
    pub fn clip_area(mut self, clip_area: Option<Rect>) -> Self {
        self.origin = self.origin.clip_area( clip_area.map( |v| v.into() ) );
        self
    }

    /// Set the area of the image that will be displayed.
    ///
    /// If `None`, then the whole image will be displayed.
    #[inline]
    pub fn set_clip_area(&mut self, clip_area: Option<Rect>) {
        self.origin.set_clip_area(clip_area.map(|v| v.into()));
        // Invalidation not necessary
    }

    // TODO
    // /// Set new `ImageBuf`.
    // #[inline]
    // pub fn set_image_data(&mut self, image_data: ImageBuf) {
    //     self.origin.set_image_data();
    // }

}

gen_wrapper!( List boxed );
impl <T:'static + Named + druid::Data> List<T> {
    pub fn new(f:Value) -> Self {
        let f = Function::from_value(f).unwrap();
        let dummy_self = ();
        let list = druid::widget::List::new(  move || {
            // f.call( () ).unwrap()
            druid::widget::Label::new("Temp")
        });
        Self { origin : Box::new( list.lens( DummyLens::<T,_>::new( std::sync::Arc::new(Vec::<T>::new()) ) ) ) }
    }
}

gen_wrapper!( ProgressBar boxed );
impl <T: Named + druid::Data> ProgressBar<T> {
    pub fn new() -> Self {
        Self {
            origin : Box::new( druid::widget::ProgressBar::new().lens( DummyLens::<T,_>::new( 0f64) ) )
        }
    }
}

gen_wrapper!( Painter );
impl <T: Named + druid::Data> Painter<T> {
    pub fn new( f:Function ) -> Self {
        let fb = move |ctx:&mut druid::PaintCtx, t:&T, env:&druid::Env| {
            let ctx = PaintCtx::new(ctx);
            let _:() = f.call( (ctx,) ).unwrap();
        };
        Self {
            origin : druid::widget::Painter::new( fb )
        }
    }
}

gen_wrapper!( RadioGroup boxed );
///WidgetExt functions
impl <T:'static + Named + druid::Data> RadioGroup<T> {
    pub fn new(val:Vec<String>) -> Self {
        let val2:Vec< (String,usize) > = val.into_iter().enumerate().map( |(i,v) | (v,i) ).collect();
        // let r = druid::widget::RadioGroup::column(val2 ).lens( DummyLens::<T,_>::new(0) );
        let r = druid::widget::RadioGroup::new(val2 ).lens( DummyLens::<T,_>::new(0) );
        Self {
            origin : Box::new( r ),
        }
    }

    // pub fn row(val:Vec<String>) -> Self {
    //     let val2:Vec< (String,usize) > = val.into_iter().enumerate().map( |(i,v) | (v,i) ).collect();
    //     let r = druid::widget::RadioGroup::row(val2 ).lens( DummyLens::<T,_>::new(0) );
    //     Self {
    //         origin : Box::new( r ),
    //     }
    // }
}

gen_wrapper!( Slider boxed );
impl <T: Named + druid::Data> Slider<T> {
    pub fn new() -> Self {
        Self {
            origin : Box::new( druid::widget::Slider::new().lens( DummyLens::<T,_>::new( 0f64) ) )
        }
    }
}

gen_wrapper!( Stepper boxed );
impl <T: Named + druid::Data> Stepper<T> {
    pub fn new() -> Self {
        Self {
            origin : Box::new( druid::widget::Stepper::new().lens( DummyLens::<T,_>::new( 0f64) ) )
        }
    }
}

gen_wrapper!( TextBox boxed );
impl <T: Named + druid::Data> TextBox<T> {
    pub fn new() -> Self {
        Self {
            origin : Box::new( druid::widget::TextBox::new().lens( DummyLens::<T,_>::new( String::new()) ) )
        }
    }
}

gen_wrapper!( Switch boxed );
impl <T: Named + druid::Data> Switch<T> {
    pub fn new() -> Self {
        Self {
            origin : Box::new( druid::widget::Switch::new().lens( DummyLens::<T,_>::new( false) ) )
        }
    }
}

gen_wrapper!( Spinner boxed );
impl <T: Named + druid::Data> Spinner<T> {
    pub fn new() -> Self {
        Self {
            origin : Box::new( druid::widget::Spinner::new().lens( DummyLens::<T,_>::new( 0f64) ) )
        }
    }
}

#[derive(Any)]
pub struct Svg<T:'static + Named + druid::Data> {
    #[allow(unused)]
    pub origin : druid::widget::Svg,
    p : PhantomData<T>
}

///WidgetExt functions
impl <T:'static + Named + druid::Data> Svg<T> {
    pub fn new(v:&str) -> Self {
        let svg = druid::widget::Svg::new( SvgData::from_str(v).unwrap() );
        Self {
            origin : svg,
            p : PhantomData
        }
    }

    gen_wrapper_ext!{ T }

    /// Builder-style method for specifying the fill strategy.
    pub fn fill_mode(mut self, mode: FillStrat) -> Self {
        self.origin.set_fill_mode( mode.into() );
        self
    }

    /// Modify the widget's `FillStrat`.
    pub fn set_fill_mode(&mut self, newfil: FillStrat) {
        self.set_fill_mode(newfil.into())
    }

    // /// Set the svg data.
    // pub fn set_svg_data(&mut self, svg_data: SvgData) {
    //     self.origin.set_svg_data()
    // }
}

gen_wrapper!( Split );
impl <T: Named + druid::Data> Split<T> {
    // pub fn columns(child1: Value, child2: Value) -> Self {
    //     let _dummy = ();
    //     let child1 = type_resolve!( _dummy , child1, T, boxed);
    //     let child2 = type_resolve!( _dummy , child2, T, boxed);
    //     Self::new(druid::widget::Axis::Horizontal, child1, child2)
    // }
    //
    // pub fn rows(child1: Value, child2: Value) -> Self {
    //     let _dummy = ();
    //     let child1 = type_resolve!( _dummy , child1, T, boxed);
    //     let child2 = type_resolve!( _dummy , child2, T, boxed);
    //     Self::new(druid::widget::Axis::Vertical, child1, child2)
    // }

    pub fn split_point(mut self, split_point: f64) -> Self {
        self.origin = self.origin.split_point( split_point );
        self
    }

    pub fn min_size(mut self, first: f64, second: f64) -> Self {
        self.origin = self.origin.min_size( first, second );
        self
    }

    pub fn bar_size(mut self, bar_size: f64) -> Self {
        self.origin = self.origin.bar_size( bar_size );
        self
    }

    pub fn min_bar_area(mut self, min_bar_area: f64) -> Self {
        self.origin = self.origin.min_bar_area( min_bar_area );
        self
    }

    pub fn draggable(mut self, draggable: bool) -> Self {
        self.origin = self.origin.draggable(draggable);
        self
    }

    pub fn solid_bar(mut self, solid: bool) -> Self {
        self.origin = self.origin.solid_bar(solid);
        self
    }
}
