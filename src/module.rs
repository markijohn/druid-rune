
use crate::widgets::QWidget;
use rune::{ContextError, Module};
use rune::runtime::Protocol;

pub fn module<T:rune::Any>() -> Result<Module,ContextError> {
    let mut module = Module::default();
    module.function("Q", |q:&str| -> Vec<QWidget<T>> {
        todo!()
    });

    module.ty::<QWidget<T>>()?;
    module.field_fn(Protocol::GET, "innerText", QWidget::<T>::get_text);
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