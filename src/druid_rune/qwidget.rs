use druid::widget::{Container, Flex};
use crate::widget::QChildWidget;
use rune::{ContextError, Module};
use rune::runtime::Protocol;

pub fn module<T:rune::Any>() -> Result<Module,ContextError> {
    let mut module = Module::default();
    module.function(&["Q"], |q:&str| -> Vec<QChildWidget<T>> {
        todo!()
    });

    module.ty::<QChildWidget<T>>()?;
    module.field_fn(Protocol::GET, "value", QChildWidget::<T>::get_value);
    module.field_fn(Protocol::SET, "value", QChildWidget::<T>::set_value);
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