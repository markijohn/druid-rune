use std::borrow::Borrow;
use std::ops::Deref;
use rune::{Any, Value};
use num_traits::FromPrimitive;

/// If called ( , )::from_value(Value) Value is always taken but
/// allow rust standard tuple impl `Copy` trait.
/// But rune dosen't allow (NotAccessibleTake)
/// So avoid taken runtime error and copy from reference.
pub trait ValueExt {
    fn resolve_as_copy<T:Any+Copy>(&self) -> Option<T>;
    fn resolve<T:FromPrimitive>(&self) -> Option<T>;
    fn resolve_tuple2<T:FromPrimitive>(&self) -> Option< (T,T) >;
    fn resolve_tuple3<T:FromPrimitive>(&self) -> Option< (T,T,T) >;
    fn resolve_tuple4<T:FromPrimitive>(&self) -> Option< (T,T,T,T) >;
}

impl ValueExt for Value {
    fn resolve_as_copy<T:Any+Copy>(&self) -> Option<T> {
        if let Value::Any(v) = self {
            if let Ok(ref_t) = v.downcast_borrow_ref::<T>() {
                return Some( ref_t.deref().clone() )
            }
        }
        None
    }

    fn resolve<T:FromPrimitive>(&self) -> Option<T> {
        match self {
            Value::Float(v) => {
                Some( T::from_f64(*v).unwrap() )
            },
            Value::Integer(v) => {
                Some( T::from_i64(*v).unwrap() )
            },
            _ => None
        }
    }

    fn resolve_tuple2<T:FromPrimitive>(&self) -> Option<(T, T)> {
        if let Value::Tuple(v) = self {
            let v = v.borrow_ref().unwrap();
            if v.len() == 2 {
                let mut i = v.iter();
                return Some( (i.next().unwrap().resolve::<T>().unwrap(), i.next().unwrap().resolve::<T>().unwrap()) )
            }
        }
        None
    }

    fn resolve_tuple3<T:FromPrimitive>(&self) -> Option<(T, T, T)> {
        if let Value::Tuple(v) = self {
            let v = v.borrow_ref().unwrap();
            if v.len() == 3 {
                let mut i = v.iter();
                return Some( (i.next().unwrap().resolve::<T>().unwrap(), i.next().unwrap().resolve::<T>().unwrap(), i.next().unwrap().resolve::<T>().unwrap()) )
            }
        }
        None
    }

    fn resolve_tuple4<T:FromPrimitive>(&self) -> Option<(T, T, T, T)> {
        if let Value::Tuple(v) = self {
            let v = v.borrow_ref().unwrap();
            if v.len() == 4 {
                let mut i = v.iter();
                return Some( (i.next().unwrap().resolve::<T>().unwrap(), i.next().unwrap().resolve::<T>().unwrap(), i.next().unwrap().resolve::<T>().unwrap(), i.next().unwrap().resolve::<T>().unwrap()) )
            }
        }
        None
    }
}