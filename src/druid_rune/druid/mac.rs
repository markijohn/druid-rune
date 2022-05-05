#[macro_export]
macro_rules! shape_type_resolve {
    ($_self:ident, $v:ident, |$__self:ident, $resolved:ident| $f:block ) => {
        shape_type_resolve_impl!($_self, $v, {Circle,Line,Rect,RoundedRect}, |$__self,$resolved| $f )
    };
}

#[macro_export]
macro_rules! shape_type_resolve_impl {
    ($_self:ident, $v:ident, {$($typs:ty),*}, |$__self:ident , $resolved:ident| $f:block ) => {
        let v_hash = $v.type_hash().unwrap();
        $( if v_hash == rune::Hash::from_type_id( TypeId::of::< $typs >() ) {
            let cb = |$__self:&mut PaintCtx, $resolved:$typs | $f;
            cb($_self, <$typs>::from_value($v).unwrap());
        } ) else *
        else {
            panic!("Unknown type : {}", $v.type_info().unwrap().to_string())
        }
    }
}

#[macro_export]
macro_rules! shape_type_ref_resolve {
    ($_self:ident, $v:ident, |$__self:ident, $resolved:ident| $f:block ) => {
        shape_type_ref_resolve_impl!($_self, $v, {Circle,Line,Rect,RoundedRect}, |$__self,$resolved| $f )
    };
}

#[macro_export]
macro_rules! shape_type_ref_resolve_impl {
    ($_self:ident, $v:ident, {$($typs:ty),*}, |$__self:ident , $resolved:ident| $f:block ) => {
        let v_hash = $v.type_hash().unwrap();
        $( if v_hash == rune::Hash::from_type_id( TypeId::of::< $typs >() ) {
            let cb = |$__self:&mut PaintCtx, $resolved:&$typs | $f;
            let vu = $v.into_any().unwrap();
            let resolved = vu.downcast_borrow_ref::< $typs >().unwrap();
            cb($_self, resolved.deref() );
        } ) else *
        else {
            panic!("Unknown type : {}", $v.type_info().unwrap().to_string())
        }
    }
}

#[macro_export]
macro_rules! widget_type_resolve {
    ($v:ident, $t:ident, |$resolved:ident| $f:block ) => {
        widget_type_resolve_impl!($v, {EnvScope<$t>,Scroll<$t>,Padding<$t>,Align<$t>,SizedBox<$t>,Container<$t>,Flex<$t>,Label<$t>,Button<$t>,ControllerHost<$t>,Checkbox<$t>,Either<$t>,Image<$t>,List<$t>,ProgressBar<$t>,Painter<$t>,RadioGroup<$t>,Slider<$t>,Stepper<$t>,TextBox<$t>,Switch<$t>,Spinner<$t>,Svg<$t>}, |$resolved| $f  )
    };
}

#[macro_export]
macro_rules! widget_type_resolve_impl {
    ($v:ident, {$($typs:ty),*}, |$resolved:ident| $f:block ) => { {
        let v_hash = $v.type_hash().unwrap();
        $( if v_hash == rune::Hash::from_type_id( TypeId::of::< $typs >() ) {
            let cb = |$resolved:$typs| $f;
            cb( <$typs>::from_value($v).unwrap() )
        } ) else *

    } };
}

#[macro_export]
macro_rules! type_resolve_impl {
    ($v:ident, {$($typs:ty),*}, |$resolved:ident| $f:block $($panic:literal)? ) => { {
        let v_hash = $v.type_hash().unwrap();
        $( if v_hash == rune::Hash::from_type_id( TypeId::of::< $typs >() ) {
            let cb = |$resolved:$typs| $f;
            cb( <$typs>::from_value($v).unwrap() )
        } ) else *
        else {
            panic!("Unknown type : {}", $v.type_info().unwrap().to_string())
        }
    } };
}

#[macro_export]
macro_rules! register_impl {
    ( $module:ident, $owner:ident, [$($consts:ident),*], [$($fields:ident),*], [$($const_fncs:ident),*], [$($inst_fncs:ident),*] ) => {
        $(
            //TODO : Any object constant not support
            $module.constant( &[ stringify!($owner), stringify!($consts) ], $owner::$consts)?;
        )*
        $(
            $module.field_fn( Protocol::GET, stringify!($fields), $owner::$fields)?;
            //TODO : SETTER?
        )*

        $(
            $module.function( &[ stringify!($owner), stringify!($const_fncs) ], $owner::$const_fncs)?;
        )*
        $module.ty::<$owner>();
        $(
            $module.inst_fn( stringify!($inst_fncs), $owner::$inst_fncs)?;
        )*
    }
}